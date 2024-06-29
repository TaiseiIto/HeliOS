//! # The kernel

#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

extern crate alloc;

mod acpi;
mod application;
mod argument;
mod processor;
mod efi;
mod elf;
mod interrupt;
mod io;
mod memory;
mod rs232c;
mod sync;
mod syscall;
mod task;
mod timer;
mod x64;

pub use argument::Argument;

use {
    alloc::{
        boxed::Box,
        collections::BTreeMap,
        vec::Vec,
    },
    core::{
        arch::asm,
        mem::MaybeUninit,
        ops::Range,
        panic::PanicInfo,
    },
};

const PRIVILEGE_LEVEL: u8 = 0;

#[no_mangle]
fn main(argument: &'static mut Argument<'static>) {
    x64::cli();
    argument.set();
    rs232c::set_com2(Argument::get().com2_mut());
    com2_println!("Hello from /HeliOS/kernel.elf");
    // Initialize allocator.
    let heap_size: usize = memory::initialize(Argument::get().paging_mut(), Argument::get().memory_map(), Argument::get().heap_start());
    com2_println!("heap_size = {:#x?}", heap_size);
    // Check memory map.
    let memory_map: Vec<&efi::memory::Descriptor> = Argument::get()
        .memory_map()
        .iter()
        .collect();
    com2_println!("memory_map = {:#x?}", memory_map);
    // Initialize GDT.
    let mut gdt = memory::segment::descriptor::Table::get();
    let gdtr: memory::segment::descriptor::table::Register = (&gdt).into();
    gdtr.set();
    let cs: memory::segment::Selector = memory::segment::Selector::cs();
    com2_println!("cs = {:#x?}", cs);
    let ds: memory::segment::Selector = memory::segment::Selector::ds();
    com2_println!("ds = {:#x?}", ds);
    let es: memory::segment::Selector = memory::segment::Selector::es();
    com2_println!("es = {:#x?}", es);
    let fs: memory::segment::Selector = memory::segment::Selector::fs();
    com2_println!("fs = {:#x?}", fs);
    let gs: memory::segment::Selector = memory::segment::Selector::gs();
    com2_println!("gs = {:#x?}", gs);
    let ss: memory::segment::Selector = memory::segment::Selector::ss();
    com2_println!("ss = {:#x?}", ss);
    let kernel_code_segment_descriptor: memory::segment::descriptor::Interface = gdt
        .descriptor(&cs)
        .unwrap();
    let kernel_data_segment_descriptor: memory::segment::descriptor::Interface = gdt
        .descriptor(&ds)
        .unwrap();
    let application_code_segment_descriptor: memory::segment::descriptor::Interface = kernel_code_segment_descriptor
        .with_dpl(application::PRIVILEGE_LEVEL);
    let application_data_segment_descriptor: memory::segment::descriptor::Interface = kernel_data_segment_descriptor
        .with_dpl(application::PRIVILEGE_LEVEL);
    let segment_descriptors = [
        kernel_code_segment_descriptor,
        kernel_data_segment_descriptor,
        application_data_segment_descriptor,
        application_code_segment_descriptor,
    ];
    let segment_descriptors: &[memory::segment::descriptor::Interface] = segment_descriptors.as_slice();
    let mut segment_descriptor_indices: Range<usize> = gdt.continuous_free_descriptor_indices(segment_descriptors.len()).unwrap();
    segment_descriptor_indices
        .clone()
        .zip(segment_descriptors.iter())
        .for_each(|(index, descriptor)| gdt.set_descriptor(index, descriptor));
    let kernel_code_segment_index: usize = segment_descriptor_indices.next().unwrap();
    let kernel_data_segment_index: usize = segment_descriptor_indices.next().unwrap();
    let application_data_segment_index: usize = segment_descriptor_indices.next().unwrap();
    let application_code_segment_index: usize = segment_descriptor_indices.next().unwrap();
    let is_ldt: bool = false;
    let kernel_code_segment_selector = memory::segment::Selector::create(kernel_code_segment_index as u16, is_ldt, PRIVILEGE_LEVEL);
    let kernel_data_segment_selector = memory::segment::Selector::create(kernel_data_segment_index as u16, is_ldt, PRIVILEGE_LEVEL);
    let application_code_segment_selector = memory::segment::Selector::create(application_code_segment_index as u16, is_ldt, application::PRIVILEGE_LEVEL);
    let application_data_segment_selector = memory::segment::Selector::create(application_data_segment_index as u16, is_ldt, application::PRIVILEGE_LEVEL);
    x64::set_segment_registers(&kernel_code_segment_selector, &kernel_data_segment_selector); // Don't rewrite segment registers before exiting boot services.
    // Initialize IDT.
    let mut idt = interrupt::descriptor::Table::get();
    interrupt::register_handlers(&mut idt);
    let idtr: interrupt::descriptor::table::Register = (&idt).into();
    idtr.set();
    let interrupt_stacks: Vec<memory::Stack> = (0..x64::task::state::Segment::NUMBER_OF_INTERRUPT_STACKS + x64::task::state::Segment::NUMBER_OF_STACK_POINTERS)
        .map(|index| {
            let pages: usize = 0x10;
            let floor_inclusive: usize = Argument::get().heap_start() - (2 * index + 1) * pages * memory::page::SIZE - 1;
            memory::Stack::new(Argument::get().paging_mut(), floor_inclusive, pages)
        })
        .collect();
    let task_state_segment_and_io_permission_bit_map: Box<x64::task::state::segment::AndIoPermissionBitMap> = x64::task::state::segment::AndIoPermissionBitMap::new(&interrupt_stacks);
    let task_state_segment_descriptor: memory::segment::long::Descriptor = (task_state_segment_and_io_permission_bit_map.as_ref()).into();
    let task_state_segment_selector: memory::segment::Selector = gdt.set_task_state_segment_descriptor(&task_state_segment_descriptor);
    let task_register: x64::task::Register = task_state_segment_selector.into();
    task_register.set();
    let task_register = x64::task::Register::get();
    com2_println!("task_register = {:#x?}", task_register);
    // Initialize syscall.
    syscall::initialize(Argument::get().cpuid(), &kernel_code_segment_selector, &kernel_data_segment_selector, &application_code_segment_selector, &application_data_segment_selector);
    // Initialize a current task.
    task::Controller::set_current();
    task::Controller::get_current_mut()
        .unwrap()
        .sti();
    // Test interrupt.
    unsafe {
        asm!("int 0x80");
    }
    // Check RSDP.
    assert!(Argument::get()
        .efi_system_table()
        .rsdp()
        .is_correct());
    // Set APIC.
    let io_apic: &mut interrupt::apic::io::Registers = Argument::get()
        .efi_system_table_mut()
        .rsdp_mut()
        .xsdt_mut()
        .madt_mut()
        .io_apic_mut()
        .registers_mut();
    let io_apic_identification: interrupt::apic::io::identification::Register = io_apic.identification();
    com2_println!("io_apic_identification = {:#x?}", io_apic_identification);
    let io_apic_version: interrupt::apic::io::version::Register = io_apic.version();
    com2_println!("io_apic_version = {:#x?}", io_apic_version);
    let io_apic_redirection_table_entries: Vec<interrupt::apic::io::redirection::table::Entry> = io_apic.redirection_table_entries();
    com2_println!("io_apic_redirection_table_entries = {:#x?}", io_apic_redirection_table_entries);
    let mut ia32_apic_base = x64::msr::ia32::ApicBase::get(Argument::get().cpuid()).unwrap();
    ia32_apic_base.enable();
    let local_apic_registers: &mut interrupt::apic::local::Registers = ia32_apic_base.registers_mut();
    // Set PIT.
    let pit_frequency: usize = 0x20; // Hz
    let pit_irq: u8 = timer::pit::set_periodic_interrupt(pit_frequency);
    Argument::get()
        .efi_system_table_mut()
        .rsdp_mut()
        .xsdt_mut()
        .madt_mut()
        .io_apic_mut()
        .registers_mut()
        .redirect(pit_irq, local_apic_registers.apic_id(), interrupt::PIT_INTERRUPT);
    // Set RTC.
    let status_register_a = timer::rtc::status_register::A::read();
    com2_println!("status_register_a = {:#x?}", status_register_a);
    let status_register_b = timer::rtc::status_register::B::read();
    com2_println!("status_register_b = {:#x?}", status_register_b);
    let status_register_c = timer::rtc::status_register::C::read();
    com2_println!("status_register_c = {:#x?}", status_register_c);
    let status_register_d = timer::rtc::status_register::D::read();
    com2_println!("status_register_d = {:#x?}", status_register_d);
    let time = timer::rtc::Time::get();
    com2_println!("time = {:#?}", time);
    let rtc_frequency: usize = 0x2; // Hz
    let rtc_irq: u8 = timer::rtc::set_periodic_interrupt(rtc_frequency);
    Argument::get()
        .efi_system_table_mut()
        .rsdp_mut()
        .xsdt_mut()
        .madt_mut()
        .io_apic_mut()
        .registers_mut()
        .redirect(rtc_irq, local_apic_registers.apic_id(), interrupt::RTC_INTERRUPT);
    // Set HPET.
    let hpet: &mut timer::hpet::Registers = Argument::get()
        .efi_system_table_mut()
        .rsdp_mut()
        .xsdt_mut()
        .hpet_mut()
        .registers_mut();
    let hpet_interrupt_period_milliseconds: usize = 1000;
    let hpet_irq: u8 = hpet.set_periodic_interrupt(hpet_interrupt_period_milliseconds);
    Argument::get()
        .efi_system_table_mut()
        .rsdp_mut()
        .xsdt_mut()
        .madt_mut()
        .io_apic_mut()
        .registers_mut()
        .redirect(hpet_irq, local_apic_registers.apic_id(), interrupt::HPET_INTERRUPT);
    hpet.start();
    let hpet: &timer::hpet::Registers = Argument::get()
        .efi_system_table()
        .rsdp()
        .xsdt()
        .hpet()
        .registers();
    com2_println!("hpet = {:#x?}", hpet);
    // Test ACPI Timer
    com2_println!("ACPI timer counter value = {:#x?}", timer::acpi::read_counter_value());
    // Boot application processors.
    let my_local_apic_id: u8 = local_apic_registers.apic_id();
    let mut processor_paging: memory::Paging = Argument::get()
        .paging()
        .clone();
    let processor_kernel: elf::File = Argument::get()
        .processor_kernel()
        .clone()
        .into();
    let _processor_kernel_read_only_pages: Vec<memory::Page> = processor_kernel.deploy_unwritable_segments(&mut processor_paging);
    let processors: Vec<acpi::multiple_apic_description::processor_local_apic::Structure> = Argument::get()
        .efi_system_table()
        .rsdp()
        .xsdt()
        .madt()
        .processor_local_apic_structures()
        .into_iter()
        .filter(|processor_local_apic| processor_local_apic.is_enabled())
        .collect();
    let number_of_processors: usize = processors.len();
    com2_println!("number_of_processors = {:#x?}", number_of_processors);
    let processor_heap_size: usize = (heap_size / number_of_processors + 1).next_power_of_two();
    let processor_heap_size: usize = processor_heap_size / if processor_heap_size / 2 + (number_of_processors - 1) * processor_heap_size < heap_size {
        1
    } else {
        2
    };
    com2_println!("processor_heap_size = {:#x?}", processor_heap_size);
    let processors: Vec<processor::Controller> = processors
        .into_iter()
        .filter(|processor_local_apic| processor_local_apic.apic_id() != my_local_apic_id)
        .map(|processor_local_apic| {
            let mut heap: Vec<MaybeUninit<u8>> = Vec::with_capacity(processor_heap_size);
            unsafe {
                heap.set_len(processor_heap_size);
            }
            processor::Controller::new(processor_local_apic.clone(), processor_paging.clone(), &processor_kernel, heap)
        })
        .collect();
    processor::Controller::set_all(processors);
    processor::Controller::get_all().for_each(|processor| processor.boot(Argument::get().processor_boot_loader_mut(), local_apic_registers, hpet, my_local_apic_id, Argument::get().heap_start()));
    while !processor::Controller::get_all().all(|processor| processor.kernel_is_completed()) {
        x64::pause();
    }
    let local_apic_id2log: BTreeMap<u8, &str> = processor::Controller::get_all()
        .map(|processor| (processor.local_apic_id(), processor.log()))
        .collect();
    local_apic_id2log
        .into_iter()
        .for_each(|(local_apic_id, log)| {
            com2_println!("Application processor log");
            com2_println!("Local APIC ID = {:#x?}", local_apic_id);
            com2_println!("{}", log);
        });
    // Shutdown.
    Argument::get()
        .efi_system_table()
        .shutdown();
    panic!("End of kernel.elf");
}

/// # A panic handler of the kernel
#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    com2_println!("KERNEL PANIC!!!");
    com2_println!("{}", panic);
    loop {
        x64::hlt();
    }
}

