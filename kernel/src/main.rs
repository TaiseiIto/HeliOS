//! # The kernel

#![no_main]
#![no_std]

extern crate alloc;

mod allocator;
mod efi;
mod interrupt;
mod memory;
mod rs232c;
mod x64;

use {
    alloc::{
        collections::BTreeMap,
        vec::Vec,
    },
    core::panic::PanicInfo,
};

#[derive(Debug)]
pub struct Argument<'a> {
    com2: &'a mut rs232c::Com,
    cpuid: Option<x64::Cpuid>,
    efi_system_table: &'a mut efi::SystemTable<'a>,
    #[allow(dead_code)]
    fonts: BTreeMap<usize, efi::Font<'a>>,
    #[allow(dead_code)]
    gdt: memory::segment::descriptor::Table,
    #[allow(dead_code)]
    graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
    heap_start: usize,
    #[allow(dead_code)]
    idt: interrupt::descriptor::Table,
    memory_map: efi::memory::Map,
    my_processor_number: Option<usize>,
    processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation>,
    paging: memory::Paging,
}

#[no_mangle]
fn main(argument: &'static mut Argument<'static>) {
    let Argument {
        com2,
        cpuid,
        efi_system_table,
        fonts: _,
        gdt: _,
        graphics_output_protocol: _,
        heap_start,
        idt: _,
        memory_map,
        my_processor_number,
        processor_informations,
        paging,
    } = argument;
    efi_system_table.set();
    rs232c::set_com2(com2);
    allocator::initialize(paging, memory_map, *heap_start);
    com2_println!("cpuid = {:#x?}", cpuid);
    com2_println!("my_processor_number = {:#x?}", my_processor_number);
    com2_println!("processor_informations = {:#x?}", processor_informations);
    let task_register = x64::task::Register::get();
    com2_println!("task_register = {:#x?}", task_register);
    let interrupt_stacks: Vec<memory::Stack> = (0..x64::task::state::Segment::NUMBER_OF_INTERRUPT_STACKS)
        .map(|index| {
            let floor: usize = *heap_start - (2 * index + 1) * memory::page::SIZE;
            let pages: usize = 0x10;
            memory::Stack::new(paging, floor, pages)
        })
        .collect();
    let task_state_segment_and_io_permission_bit_map = x64::task::state::segment::AndIoPermissionBitMap::new(&interrupt_stacks);
    com2_println!("task_state_segment_and_io_permission_bit_map = {:#x?}", task_state_segment_and_io_permission_bit_map);
    let task_state_segment_descriptor: memory::segment::Descriptor = (&task_state_segment_and_io_permission_bit_map).into();
    com2_println!("task_state_segment_descriptor = {:#x?}", task_state_segment_descriptor);
    efi::SystemTable::get().shutdown();
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

