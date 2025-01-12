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

use core::panic::PanicInfo;

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
    // Boot application processors.
    processor::Manager::initialize(local_apic_id, local_apic_registers, heap_size, hpet);
    // Enumerate PCI devices.
    let pci = pci::Configuration::read();
    com2_println!("pci = {:#x?}", pci);
    // Kernel loop.
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
    timer::hpet::Registers::finalize();
    // Stop APIC interruptions.
    local_apic_registers.disable_periodic_interrupt();
    // Disable all interruptions.
    task::Controller::get_current_mut()
        .unwrap()
        .cli();
    // Print AP log.
    processor::Manager::finalize();
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

