//! # The kernel

#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

extern crate alloc;

mod acpi;
mod argument;
mod efi;
mod elf;
mod interrupt;
mod io;
mod memory;
mod pci;
mod processor;
mod rs232c;
mod sync;
mod syscall;
mod task;
mod timer;
mod x64;

pub use argument::Argument;

use {
    alloc::{
        collections::BTreeMap,
        vec::Vec,
    },
    core::{
        mem::MaybeUninit,
        panic::PanicInfo,
    },
};

#[no_mangle]
fn main(argument: &'static mut Argument<'static>) {
    // Prohibit interrupts.
    x64::cli();
    // Set argument from the bootloader to the kernel.
    argument.set();
    // Initialize heap memory.
    let heap_size: usize = memory::initialize(Argument::get().paging_mut(), Argument::get().memory_map(), Argument::get().heap_start());
    // Initialize GDT.
    let mut gdt = memory::segment::descriptor::table::Controller::new();
    // Initialize IDT.
    let _idt = interrupt::descriptor::table::Controller::new(&mut gdt);
    // Initialize syscall.
    syscall::initialize(Argument::get().cpuid(), gdt.kernel_code_segment_selector(), gdt.kernel_data_segment_selector(), gdt.application_code_segment_selector(), gdt.application_data_segment_selector());
    // Initialize a current task.
    task::Controller::set_current();
    // Allow interrupts.
    task::Controller::get_current_mut()
        .unwrap()
        .sti();
    // Check RSDP.
    assert!(Argument::get()
        .efi_system_table()
        .rsdp()
        .is_correct());
    // Set APIC.
    let mut ia32_apic_base = x64::msr::ia32::ApicBase::get().unwrap();
    let local_apic_registers = interrupt::apic::local::Registers::initialize(&mut ia32_apic_base);
    let local_apic_id: u8 = local_apic_registers.apic_id();
    // Set PIT.
    timer::pit::initialize(local_apic_id);
    // Set RTC.
    timer::rtc::initialize(local_apic_id);
    // Set HPET.
    let hpet = timer::hpet::Registers::initialize(local_apic_id);
    // Set APIC Timer.
    local_apic_registers.initialize_apic(hpet);
    // Test ACPI Timer.
    com2_println!("ACPI timer bits = {:#x?}", timer::acpi::bits());
    com2_println!("ACPI timer counter value = {:#x?}", timer::acpi::counter_value());
    // Test TSC.
    com2_println!("Time stamp counter is {}", if timer::tsc::is_invariant() {
        "invariant"
    } else {
        "variant"
    });
    com2_println!("Time stamp counter frequency = {:#x?}", timer::tsc::frequency());
    com2_println!("Time stamp counter = {:#x?}", timer::tsc::counter_value());
    // Boot application processors.
    let mut processor_paging: memory::Paging = Argument::get()
        .paging()
        .clone();
    let processor_kernel: elf::File = Argument::get()
        .processor_kernel()
        .to_vec()
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
        .filter(|processor_local_apic| processor_local_apic.apic_id() != local_apic_id)
        .map(|processor_local_apic| {
            let mut heap: Vec<MaybeUninit<u8>> = Vec::with_capacity(processor_heap_size);
            unsafe {
                heap.set_len(processor_heap_size);
            }
            processor::Controller::new(processor_local_apic.clone(), processor_paging.clone(), &processor_kernel, heap)
        })
        .collect();
    processor::Controller::set_all(processors);
    processor::Controller::get_all().for_each(|processor| processor.boot(Argument::get().processor_boot_loader_mut(), local_apic_registers, hpet, local_apic_id, Argument::get().heap_start()));
    let mut shutdown: bool = false;
    let mut loop_counter: usize = 0;
    while !shutdown {
        match interrupt::Event::pop() {
            Some(event) => event.process(),
            None => x64::hlt(),
        }
        loop_counter += if processor::Controller::get_all().all(|processor| processor.is_initialized()) {
            1
        } else {
            0
        };
        shutdown = 0x100 <= loop_counter;
    }
    // Stop RTC interruptions.
    timer::rtc::disable_periodic_interrupt();
    // Stop HPET.
    let hpet: &mut timer::hpet::Registers = Argument::get()
        .efi_system_table_mut()
        .rsdp_mut()
        .xsdt_mut()
        .hpet_mut()
        .registers_mut();
    hpet.stop();
    hpet.disable_periodic_interrupt();
    // Stop APIC interruptions.
    local_apic_registers.disable_periodic_interrupt();
    // Disable all interruptions.
    task::Controller::get_current_mut()
        .unwrap()
        .cli();
    // Print AP log.
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
    // Enumerate PCI devices.
    let pci = pci::Configuration::read();
    com2_println!("pci = {:#x?}", pci);
    // Shutdown.
    com2_println!("Shutting down.");
    Argument::get()
        .efi_system_table_mut()
        .rsdp_mut()
        .xsdt_mut()
        .fadt_mut()
        .shutdown();
    unreachable!();
}

/// # A panic handler of the kernel
#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    com2_println!("KERNEL PANIC!!!");
    com2_println!("{}", panic);
    Argument::get()
        .efi_system_table()
        .shutdown()
}

