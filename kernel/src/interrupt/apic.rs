//! # Advanced Programmable Interrupt Controller (APIC)
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A Chapter 11 Advanced Programmable Interrupt Controller (APIC)

pub mod divide_configuration;
pub mod error_status;
pub mod local_apic_version;
pub mod local_vector_table;

use crate::x64;

/// # Local APIC Registers
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.4.1 Table 11-1. Local APIC Register Address Map
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    // 0xfee00000
    reserved0: [u128; 2],
    // 0xfee00020
    local_apic_id: u128,
    // 0xfee00030
    local_apic_version: local_apic_version::Register,
    // 0xfee00040
    reserved1: [u128; 4],
    // 0xfee00080
    task_priority: u128,
    // 0xfee00090
    arbitration_priority: u128,
    // 0xfee000a0
    processor_priority: u128,
    // 0xfee000b0
    eoi: u128,
    // 0xfee000c0
    remote_read: u128,
    // 0xfee000d0
    local_destination: u128,
    // 0xfee000e0
    destination_format: u128,
    // 0xfee000f0
    spurious_interrupt_vector: u128,
    // 0xfee00100
    in_service: [u128; 8],
    // 0xfee00180
    trigger_mode_register: [u128; 8],
    // 0xfee00200
    interrupt_request_register: [u128; 8],
    // 0xfee00280
    error_status: error_status::Register,
    // 0xfee00290
    reserved2: [u128; 6],
    // 0xfee002f0
    lvt_corrected_machine_check_interrupt: local_vector_table::Register,
    // 0xfee00300
    interrupt_command: [u128; 2],
    // 0xfee00320
    lvt_timer: u128,
    // 0xfee00330
    lvt_thermal_sensor: local_vector_table::Register,
    // 0xfee00340
    lvt_performance_monitoring_counters: local_vector_table::Register,
    // 0xfee00350
    lvt_lint: [local_vector_table::Register; 2],
    // 0xfee00370
    lvt_error: local_vector_table::Register,
    // 0xfee00380
    initial_count: u128,
    // 0xfee00390
    current_count: u128,
    // 0xfee003a0
    reserved3: [u128; 4],
    // 0xfee003e0
    divide_configuration: divide_configuration::Register,
    // 0xfee003f0
    reserved: u128,
}

impl Registers {
    pub fn get(apic_base: &x64::msr::ia32::ApicBase) -> &Self {
        apic_base.registers()
    }
}

