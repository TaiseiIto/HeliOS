//! # The kernel

#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

extern crate alloc;

mod acpi;
mod allocator;
mod application;
mod processor;
mod efi;
mod elf;
mod interrupt;
mod memory;
mod rs232c;
mod syscall;
mod task;
mod timer;
mod x64;

use {
    alloc::{
        boxed::Box,
        collections::{
            BTreeMap,
            BTreeSet,
        },
        vec::Vec,
    },
    core::{
        arch::asm,
        cell::OnceCell,
        ops::Range,
        panic::PanicInfo,
    },
};

static mut ARGUMENT: OnceCell<&'static mut Argument<'static>> = OnceCell::new();

#[derive(Debug)]
pub struct Argument<'a> {
    processor_boot_loader: processor::boot::Loader,
    processor_kernel: Vec<u8>,
    com2: &'a mut rs232c::Com,
    cpuid: x64::Cpuid,
    efi_system_table: &'a mut efi::SystemTable<'a>,
    #[allow(dead_code)]
    fonts: BTreeMap<usize, efi::Font<'a>>,
    #[allow(dead_code)]
    graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
    heap_start: usize,
    #[allow(dead_code)]
    hello_application: elf::File,
    memory_map: efi::memory::Map,
    paging: memory::Paging,
}

impl Argument<'_> {
    pub fn cpuid(&self) -> &x64::Cpuid {
        &self.cpuid
    }

    pub fn heap_start(&self) -> usize {
        self.heap_start
    }
}

impl Argument<'static> {
    pub fn get() -> &'static mut Self {
        unsafe {
            ARGUMENT
                .get_mut()
                .unwrap()
        }
    }

    pub fn set(&'static mut self) {
        unsafe {
            ARGUMENT.set(self)
        }.unwrap()
    }
}

const PRIVILEGE_LEVEL: u8 = 0;

#[no_mangle]
fn main(argument: &'static mut Argument<'static>) {
    argument.set();
    rs232c::set_com2(Argument::get().com2);
    com2_println!("Hello from /HeliOS/kernel.elf");
    // Initialize allocator.
    let heap_size: usize = allocator::initialize(&mut Argument::get().paging, &Argument::get().memory_map, Argument::get().heap_start());
    com2_println!("heap_size = {:#x?}", heap_size);
    // Check memory map.
    let memory_map: Vec<&efi::memory::Descriptor> = Argument::get()
        .memory_map
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
    let idtr: interrupt::descriptor::table::Register = (&idt).into();
    idtr.set();
    let interrupt_stacks: Vec<memory::Stack> = (0..x64::task::state::Segment::NUMBER_OF_INTERRUPT_STACKS + x64::task::state::Segment::NUMBER_OF_STACK_POINTERS)
        .map(|index| {
            let pages: usize = 0x10;
            let floor_inclusive: usize = Argument::get().heap_start() - (2 * index + 1) * pages * memory::page::SIZE - 1;
            memory::Stack::new(&mut Argument::get().paging, floor_inclusive, pages)
        })
        .collect();
    let task_state_segment_and_io_permission_bit_map: Box<x64::task::state::segment::AndIoPermissionBitMap> = x64::task::state::segment::AndIoPermissionBitMap::new(&interrupt_stacks);
    let task_state_segment_descriptor: memory::segment::long::Descriptor = (task_state_segment_and_io_permission_bit_map.as_ref()).into();
    let task_state_segment_selector: memory::segment::Selector = gdt.set_task_state_segment_descriptor(&task_state_segment_descriptor);
    let task_register: x64::task::Register = task_state_segment_selector.into();
    task_register.set();
    let task_register = x64::task::Register::get();
    com2_println!("task_register = {:#x?}", task_register);
    interrupt::register_handlers(&mut idt);
    // Initialize syscall.
    syscall::initialize(&Argument::get().cpuid, &kernel_code_segment_selector, &kernel_data_segment_selector, &application_code_segment_selector, &application_data_segment_selector);
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
        .efi_system_table
        .rsdp()
        .is_correct());
    // Set APIC.
    let io_apic: &mut interrupt::apic::io::Registers = Argument::get()
        .efi_system_table
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
    let mut ia32_apic_base = x64::msr::ia32::ApicBase::get(&Argument::get().cpuid).unwrap();
    ia32_apic_base.enable();
    let local_apic_registers: &mut interrupt::apic::local::Registers = ia32_apic_base.registers_mut();
    // Start HPET.
    Argument::get()
        .efi_system_table
        .rsdp_mut()
        .xsdt_mut()
        .hpet_mut()
        .registers_mut()
        .start_counting();
    let hpet: &timer::hpet::Registers = Argument::get()
        .efi_system_table
        .rsdp()
        .xsdt()
        .hpet()
        .registers();
    // Boot application processors.
    let my_local_apic_id: u8 = local_apic_registers.apic_id();
    let mut processor_paging: memory::Paging = Argument::get()
        .paging
        .clone();
    let processor_kernel: elf::File = Argument::get()
        .processor_kernel
        .clone()
        .into();
    let processor_kernel_read_only_pages: Vec<memory::Page> = processor_kernel.deploy_unwritable_segments(&mut processor_paging);
    let mut processors: Vec<processor::Controller> = Argument::get()
        .efi_system_table
        .rsdp()
        .xsdt()
        .madt()
        .processor_local_apic_structures()
        .iter()
        .map(|processor_local_apic| processor::Controller::new(processor_local_apic.clone(), processor_paging.clone()))
        .collect();
    processors
        .iter_mut()
        .filter(|processor| processor.local_apic_id() != my_local_apic_id)
        .for_each(|processor| processor.boot(&mut Argument::get().processor_boot_loader, local_apic_registers, hpet, &processor_kernel, my_local_apic_id));
    com2_println!("processors = {:#x?}", processors);
    let rflags = x64::Rflags::get();
    com2_println!("rflags = {:#x?}", rflags);
    // Shutdown.
    Argument::get()
        .efi_system_table
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

