//! # Advanced Programmable Interrupt Controller (APIC)
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A Chapter 11 Advanced Programmable Interrupt Controller (APIC)

pub mod arbitration_priority;
pub mod current_count;
pub mod destination_format;
pub mod divide_configuration;
pub mod end_of_interrupt;
pub mod error_status;
pub mod in_service;
pub mod initial_count;
pub mod interrupt_command;
pub mod interrupt_request;
pub mod local_apic_id;
pub mod local_apic_version;
pub mod local_vector_table;
pub mod logical_destination;
pub mod processor_priority;
pub mod spurious_interrupt_vector;
pub mod task_priority;
pub mod trigger_mode;

use {
    core::fmt,
    crate::{
        timer,
        x64,
    },
    super::{
        DeliveryMode,
        TriggerMode,
        super::APIC_TIMER_INTERRUPT,
    },
};

/// # Local APIC Registers
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.4.1 Table 11-1. Local APIC Register Address Map
#[repr(packed)]
pub struct Registers {
    // 0xfee00000
    #[allow(dead_code)]
    reserved0: [u128; 2],
    // 0xfee00020
    local_apic_id: local_apic_id::FatRegister,
    // 0xfee00030
    local_apic_version: local_apic_version::FatRegister,
    // 0xfee00040
    #[allow(dead_code)]
    reserved1: [u128; 4],
    // 0xfee00080
    task_priority: task_priority::FatRegister,
    // 0xfee00090
    arbitration_priority: arbitration_priority::FatRegister,
    // 0xfee000a0
    processor_priority: processor_priority::FatRegister,
    // 0xfee000b0
    end_of_interrupt: end_of_interrupt::FatRegister,
    // 0xfee000c0
    remote_read: u128,
    // 0xfee000d0
    logical_destination: logical_destination::FatRegister,
    // 0xfee000e0
    destination_format: destination_format::FatRegister,
    // 0xfee000f0
    spurious_interrupt_vector: spurious_interrupt_vector::FatRegister,
    // 0xfee00100
    in_service: in_service::FatRegisters,
    // 0xfee00180
    trigger_mode_register: trigger_mode::FatRegisters,
    // 0xfee00200
    interrupt_request_register: interrupt_request::FatRegisters,
    // 0xfee00280
    error_status: error_status::FatRegister,
    // 0xfee00290
    #[allow(dead_code)]
    reserved2: [u128; 6],
    // 0xfee002f0
    lvt_corrected_machine_check_interrupt: local_vector_table::FatRegister,
    // 0xfee00300
    interrupt_command: interrupt_command::Register,
    // 0xfee00320
    lvt_timer: local_vector_table::FatRegister,
    // 0xfee00330
    lvt_thermal_sensor: local_vector_table::FatRegister,
    // 0xfee00340
    lvt_performance_monitoring_counters: local_vector_table::FatRegister,
    // 0xfee00350
    lvt_lint: [local_vector_table::FatRegister; 2],
    // 0xfee00370
    lvt_error: local_vector_table::FatRegister,
    // 0xfee00380
    initial_count: initial_count::FatRegister,
    // 0xfee00390
    current_count: current_count::FatRegister,
    // 0xfee003a0
    #[allow(dead_code)]
    reserved3: [u128; 4],
    // 0xfee003e0
    divide_configuration: divide_configuration::FatRegister,
    // 0xfee003f0
    #[allow(dead_code)]
    reserved4: u128,
}

impl Registers {
    pub fn apic_id(&self) -> u8 {
        self.local_apic_id.apic_id()
    }

    pub fn disable_periodic_interrupt(&mut self) {
        self.lvt_timer.disable_periodic_interrupt();
    }

    pub fn enable_periodic_interrupt(&mut self, hpet: &timer::hpet::Registers, interrupt_frequency: usize) {
        let timer_frequency: usize = self.timer_frequency(hpet);
        let initial_count: u32 = (timer_frequency / interrupt_frequency) as u32;
        self.lvt_timer.set(APIC_TIMER_INTERRUPT, DeliveryMode::Fixed, local_vector_table::InterruptInputPinPolarity::ActiveHigh, TriggerMode::Edge, local_vector_table::Mask::EnableInterrupt, local_vector_table::TimerMode::Periodic);
        self.initial_count.set(initial_count);
    }

    pub fn enable_spurious_interrupt(&mut self, focus_processor_checking: bool, eoi_broadcast: bool, spurious_vector: u8) {
        self.spurious_interrupt_vector.enable(focus_processor_checking, eoi_broadcast, spurious_vector);
    }

    pub fn end_interruption(&mut self) {
        self.end_of_interrupt.write(0);
    }

    pub fn get(apic_base: &x64::msr::ia32::ApicBase) -> &Self {
        apic_base.registers()
    }

    pub fn send_init(&mut self, processor_local_apic_id: u8, hpet: &timer::hpet::Registers) {
        self.error_status.clear_all_errors();
        self.interrupt_command.assert_init(processor_local_apic_id);
        hpet.wait_microseconds(100);
        self.interrupt_command.wait_to_send();
        self.interrupt_command.deassert_init(processor_local_apic_id);
        hpet.wait_milliseconds(10);
        self.interrupt_command.wait_to_send();
    }

    pub fn send_interrupt(&mut self, destination_local_apic_id: u8, destination_vector: u8) {
        self.error_status.clear_all_errors();
        self.interrupt_command.wait_to_send();
        self.interrupt_command.send_interrupt(destination_local_apic_id, destination_vector);
    }

    pub fn send_sipi(&mut self, processor_local_apic_id: u8, entry_point: usize, hpet: &timer::hpet::Registers) {
        self.error_status.clear_all_errors();
        self.interrupt_command.send_sipi(processor_local_apic_id, entry_point);
        hpet.wait_microseconds(200);
        self.interrupt_command.wait_to_send();
    }

    fn timer_frequency(&mut self, hpet: &timer::hpet::Registers) -> usize {
        let divisor: u8 = 1;
        self.divide_configuration.set(divisor);
        self.lvt_timer.set(APIC_TIMER_INTERRUPT, DeliveryMode::Fixed, local_vector_table::InterruptInputPinPolarity::ActiveHigh, TriggerMode::Edge, local_vector_table::Mask::InhibitInterrupt, local_vector_table::TimerMode::OneShot);
        let start = u32::MAX;
        self.initial_count.set(start);
        hpet.wait_seconds(1);
        let end: u32 = self.current_count.get();
        (start - end) as usize
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let local_apic_id: local_apic_id::FatRegister = self.local_apic_id;
        let local_apic_version: local_apic_version::FatRegister = self.local_apic_version;
        let task_priority: task_priority::FatRegister = self.task_priority;
        let arbitration_priority: arbitration_priority::FatRegister = self.arbitration_priority;
        let processor_priority: processor_priority::FatRegister = self.processor_priority;
        let end_of_interrupt: end_of_interrupt::FatRegister = self.end_of_interrupt;
        let remote_read: u128 = self.remote_read;
        let logical_destination: logical_destination::FatRegister = self.logical_destination;
        let destination_format: destination_format::FatRegister = self.destination_format;
        let spurious_interrupt_vector: spurious_interrupt_vector::FatRegister = self.spurious_interrupt_vector;
        let in_service: in_service::FatRegisters = self.in_service;
        let trigger_mode_register: trigger_mode::FatRegisters = self.trigger_mode_register;
        let interrupt_request_register: interrupt_request::FatRegisters = self.interrupt_request_register;
        let error_status: error_status::FatRegister = self.error_status;
        let lvt_corrected_machine_check_interrupt: local_vector_table::FatRegister = self.lvt_corrected_machine_check_interrupt;
        let interrupt_command: interrupt_command::Register = self.interrupt_command;
        let lvt_timer: local_vector_table::FatRegister = self.lvt_timer;
        let lvt_thermal_sensor: local_vector_table::FatRegister = self.lvt_thermal_sensor;
        let lvt_performance_monitoring_counters: local_vector_table::FatRegister = self.lvt_performance_monitoring_counters;
        let lvt_lint: [local_vector_table::FatRegister; 2] = self.lvt_lint;
        let lvt_error: local_vector_table::FatRegister = self.lvt_error;
        let initial_count: initial_count::FatRegister = self.initial_count;
        let current_count: current_count::FatRegister = self.current_count;
        let divide_configuration: divide_configuration::FatRegister = self.divide_configuration;
        formatter
            .debug_struct("Registers")
            .field("local_apic_id", &local_apic_id)
            .field("local_apic_version", &local_apic_version)
            .field("task_priority", &task_priority)
            .field("arbitration_priority", &arbitration_priority)
            .field("processor_priority", &processor_priority)
            .field("end_of_interrupt", &end_of_interrupt)
            .field("remote_read", &remote_read)
            .field("logical_destination", &logical_destination)
            .field("destination_format", &destination_format)
            .field("spurious_interrupt_vector", &spurious_interrupt_vector)
            .field("in_service", &in_service)
            .field("trigger_mode_register", &trigger_mode_register)
            .field("interrupt_request_register", &interrupt_request_register)
            .field("error_status", &error_status)
            .field("lvt_corrected_machine_check_interrupt", &lvt_corrected_machine_check_interrupt)
            .field("interrupt_command", &interrupt_command)
            .field("lvt_timer", &lvt_timer)
            .field("lvt_thermal_sensor", &lvt_thermal_sensor)
            .field("lvt_performance_monitoring_counters", &lvt_performance_monitoring_counters)
            .field("lvt_lint", &lvt_lint)
            .field("lvt_error", &lvt_error)
            .field("initial_count", &initial_count)
            .field("current_count", &current_count)
            .field("divide_configuration", &divide_configuration)
            .finish()
    }
}

