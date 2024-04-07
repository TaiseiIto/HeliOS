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
mod timer;
mod x64;

use {
    alloc::{
        boxed::Box,
        collections::BTreeMap,
        vec::Vec,
    },
    core::{
        arch::asm,
        ops::Range,
        panic::PanicInfo,
    },
};

#[derive(Debug)]
pub struct Argument<'a> {
    processor_boot_loader: processor::boot::Loader,
    com2: &'a mut rs232c::Com,
    cpuid: x64::Cpuid,
    efi_system_table: &'a mut efi::SystemTable<'a>,
    #[allow(dead_code)]
    fonts: BTreeMap<usize, efi::Font<'a>>,
    #[allow(dead_code)]
    graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
    heap_start: usize,
    hello_application: elf::File,
    memory_map: efi::memory::Map,
    my_processor_number: Option<usize>,
    paging: memory::Paging,
    processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation>,
}

const PRIVILEGE_LEVEL: u8 = 0;

#[no_mangle]
fn main(argument: &'static mut Argument<'static>) {
    let Argument {
        processor_boot_loader,
        com2,
        cpuid,
        efi_system_table,
        fonts: _,
        graphics_output_protocol: _,
        heap_start,
        hello_application,
        memory_map,
        my_processor_number,
        paging,
        processor_informations,
    } = argument;
    rs232c::set_com2(com2);
    let heap_size: usize = allocator::initialize(paging, memory_map, *heap_start);
    com2_println!("processor_boot_loader = {:#x?}", processor_boot_loader);
    com2_println!("efi_system_table = {:#x?}", efi_system_table);
    com2_println!("heap_size = {:#x?}", heap_size);
    com2_println!("cpuid = {:#x?}", cpuid);
    com2_println!("hello_application = {:#x?}", hello_application);
    let memory_map: Vec<&efi::memory::Descriptor> = memory_map
        .iter()
        .collect();
    com2_println!("memory_map = {:#x?}", memory_map);
    com2_println!("processor_informations = {:#x?}", processor_informations);
    com2_println!("my_processor_number = {:#x?}", my_processor_number);
    let mut gdt = memory::segment::descriptor::Table::get();
    com2_println!("gdt = {:#x?}", gdt);
    let gdtr: memory::segment::descriptor::table::Register = (&gdt).into();
    com2_println!("gdtr = {:#x?}", gdtr);
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
    com2_println!("kernel_code_segment_descriptor = {:#x?}", kernel_code_segment_descriptor);
    let kernel_data_segment_descriptor: memory::segment::descriptor::Interface = gdt
        .descriptor(&ds)
        .unwrap();
    com2_println!("kernel_data_segment_descriptor = {:#x?}", kernel_data_segment_descriptor);
    let application_code_segment_descriptor: memory::segment::descriptor::Interface = kernel_code_segment_descriptor
        .with_dpl(application::PRIVILEGE_LEVEL);
    com2_println!("application_code_segment_descriptor = {:#x?}", application_code_segment_descriptor);
    let application_data_segment_descriptor: memory::segment::descriptor::Interface = kernel_data_segment_descriptor
        .with_dpl(application::PRIVILEGE_LEVEL);
    com2_println!("application_data_segment_descriptor = {:#x?}", application_data_segment_descriptor);
    let segment_descriptors = [
        kernel_code_segment_descriptor,
        kernel_data_segment_descriptor,
        application_data_segment_descriptor,
        application_code_segment_descriptor,
    ];
    let segment_descriptors: &[memory::segment::descriptor::Interface] = segment_descriptors.as_slice();
    let mut segment_descriptor_indices: Range<usize> = gdt.continuous_free_descriptor_indices(segment_descriptors.len()).unwrap();
    com2_println!("segment_descriptor_indices = {:#x?}", segment_descriptor_indices);
    segment_descriptor_indices
        .clone()
        .zip(segment_descriptors.iter())
        .for_each(|(index, descriptor)| gdt.set_descriptor(index, descriptor));
    com2_println!("gdt = {:#x?}", gdt);
    let kernel_code_segment_index: usize = segment_descriptor_indices.next().unwrap();
    com2_println!("kernel_code_segment_index = {:#x?}", kernel_code_segment_index);
    let kernel_data_segment_index: usize = segment_descriptor_indices.next().unwrap();
    com2_println!("kernel_data_segment_index = {:#x?}", kernel_data_segment_index);
    let application_data_segment_index: usize = segment_descriptor_indices.next().unwrap();
    com2_println!("application_data_segment_index = {:#x?}", application_data_segment_index);
    let application_code_segment_index: usize = segment_descriptor_indices.next().unwrap();
    com2_println!("application_code_segment_index = {:#x?}", application_code_segment_index);
    let is_ldt: bool = false;
    let kernel_code_segment_selector = memory::segment::Selector::create(kernel_code_segment_index as u16, is_ldt, PRIVILEGE_LEVEL);
    com2_println!("kernel_code_segment_selector = {:#x?}", kernel_code_segment_selector);
    let kernel_data_segment_selector = memory::segment::Selector::create(kernel_data_segment_index as u16, is_ldt, PRIVILEGE_LEVEL);
    com2_println!("kernel_data_segment_selector = {:#x?}", kernel_data_segment_selector);
    let application_code_segment_selector = memory::segment::Selector::create(application_code_segment_index as u16, is_ldt, application::PRIVILEGE_LEVEL);
    com2_println!("application_code_segment_selector = {:#x?}", application_code_segment_selector);
    let application_data_segment_selector = memory::segment::Selector::create(application_data_segment_index as u16, is_ldt, application::PRIVILEGE_LEVEL);
    com2_println!("application_data_segment_selector = {:#x?}", application_data_segment_selector);
    x64::set_segment_registers(&kernel_code_segment_selector, &kernel_data_segment_selector); // Don't rewrite segment registers before exiting boot services.
    let mut idt = interrupt::descriptor::Table::get();
    com2_println!("idt = {:#x?}", idt);
    let idtr: interrupt::descriptor::table::Register = (&idt).into();
    com2_println!("idtr = {:#x?}", idtr);
    idtr.set();
    let interrupt_stacks: Vec<memory::Stack> = (0..x64::task::state::Segment::NUMBER_OF_INTERRUPT_STACKS + x64::task::state::Segment::NUMBER_OF_STACK_POINTERS)
        .map(|index| {
            let pages: usize = 0x10;
            let floor: usize = *heap_start - (2 * index + 1) * pages * memory::page::SIZE;
            memory::Stack::new(paging, floor, pages)
        })
        .collect();
    let task_state_segment_and_io_permission_bit_map: Box<x64::task::state::segment::AndIoPermissionBitMap> = x64::task::state::segment::AndIoPermissionBitMap::new(&interrupt_stacks);
    let task_state_segment_descriptor: memory::segment::long::Descriptor = (task_state_segment_and_io_permission_bit_map.as_ref()).into();
    com2_println!("task_state_segment_descriptor = {:#x?}", task_state_segment_descriptor);
    let task_state_segment_selector: memory::segment::Selector = gdt.set_task_state_segment_descriptor(&task_state_segment_descriptor);
    com2_println!("task_state_segment_selector = {:#x?}", task_state_segment_selector);
    let task_register: x64::task::Register = task_state_segment_selector.into();
    com2_println!("task_register = {:#x?}", task_register);
    task_register.set();
    com2_println!("gdt = {:#x?}", gdt);
    let task_register = x64::task::Register::get();
    com2_println!("task_register = {:#x?}", task_register);
    interrupt::register_handlers(&mut idt);
    com2_println!("idt = {:#x?}", idt);
    let io_apic: &mut interrupt::apic::io::Registers = efi_system_table
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
    let hpet: &mut timer::hpet::Registers = efi_system_table
        .rsdp_mut()
        .xsdt_mut()
        .hpet_mut()
        .registers_mut();
    hpet.start_counting();
    com2_println!("hpet = {:#x?}", hpet);
    // HPET wait test
    com2_println!("hpet.wait_seconds(1) START!!!");
    hpet.wait_seconds(1);
    com2_println!("hpet.wait_seconds(1) END!!!");
    let mut ia32_apic_base = x64::msr::ia32::ApicBase::get(cpuid).unwrap();
    ia32_apic_base.enable();
    com2_println!("ia32_apic_base = {:#x?}", ia32_apic_base);
    let apic_registers: &interrupt::apic::local::Registers = ia32_apic_base.registers();
    com2_println!("apic_registers = {:#x?}", apic_registers);
    syscall::initialize(cpuid, &kernel_code_segment_selector, &kernel_data_segment_selector, &application_code_segment_selector, &application_data_segment_selector);
    unsafe {
        asm!("int 0x80");
    }
    efi_system_table.shutdown();
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

