//! # The application processor kernel

#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

extern crate alloc;

mod application;
mod argument;
mod interrupt;
mod memory;
mod processor;
mod sync;
mod task;
mod x64;

pub use argument::Argument;

use {
    alloc::{
        boxed::Box,
        vec::Vec,
    },
    core::{
        arch::asm,
        ops::Range,
        panic::PanicInfo,
    },
};

const PRIVILEGE_LEVEL: u8 = 0;

#[no_mangle]
fn main(argument: &'static Argument<'static>) {
    x64::cli();
    Argument::set(argument.clone());
    Argument::get_mut().boot_complete();
    memory::initialize(Argument::get().heap_range());
    bsp_println!("Hello, World!");
    bsp_println!("argument = {:#x?}", Argument::get());
    let cpuid: x64::Cpuid = x64::Cpuid::get().unwrap();
    bsp_println!("cpuid = {:#x?}", cpuid);
    let mut paging = memory::Paging::get(&cpuid);
    paging.set();
    // Initialize GDT.
    let mut gdt = memory::segment::descriptor::Table::get();
    let gdtr: memory::segment::descriptor::table::Register = (&gdt).into();
    gdtr.set();
    let cs: memory::segment::Selector = memory::segment::Selector::cs();
    let ds: memory::segment::Selector = memory::segment::Selector::ds();
    let es: memory::segment::Selector = memory::segment::Selector::es();
    let fs: memory::segment::Selector = memory::segment::Selector::fs();
    let gs: memory::segment::Selector = memory::segment::Selector::gs();
    let ss: memory::segment::Selector = memory::segment::Selector::ss();
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
    x64::set_segment_registers(&kernel_code_segment_selector, &kernel_data_segment_selector);
    bsp_println!("gdt = {:#x?}", gdt);
    let cs: memory::segment::Selector = memory::segment::Selector::cs();
    bsp_println!("cs = {:#x?}", cs);
    let ds: memory::segment::Selector = memory::segment::Selector::ds();
    bsp_println!("ds = {:#x?}", ds);
    let es: memory::segment::Selector = memory::segment::Selector::es();
    bsp_println!("es = {:#x?}", es);
    let fs: memory::segment::Selector = memory::segment::Selector::fs();
    bsp_println!("fs = {:#x?}", fs);
    let gs: memory::segment::Selector = memory::segment::Selector::gs();
    bsp_println!("gs = {:#x?}", gs);
    let ss: memory::segment::Selector = memory::segment::Selector::ss();
    bsp_println!("ss = {:#x?}", ss);
    // Initialize IDT.
    let mut idt = interrupt::descriptor::Table::new();
    interrupt::register_handlers(&mut idt);
    let idtr: interrupt::descriptor::table::Register = (&idt).into();
    idtr.set();
    bsp_println!("idt = {:#x?}", idt);
    let interrupt_stacks: Vec<memory::Stack> = (0..x64::task::state::Segment::NUMBER_OF_INTERRUPT_STACKS + x64::task::state::Segment::NUMBER_OF_STACK_POINTERS)
        .map(|index| {
            let pages: usize = 0x10;
            let floor_inclusive: usize = Argument::get().bsp_heap_start() - (2 * index + 1) * pages * memory::page::SIZE - 1;
            memory::Stack::new(&mut paging, floor_inclusive, pages)
        })
        .collect();
    let task_state_segment_and_io_permission_bit_map: Box<x64::task::state::segment::AndIoPermissionBitMap> = x64::task::state::segment::AndIoPermissionBitMap::new(&interrupt_stacks);
    let task_state_segment_descriptor: memory::segment::long::Descriptor = (task_state_segment_and_io_permission_bit_map.as_ref()).into();
    let task_state_segment_selector: memory::segment::Selector = gdt.set_task_state_segment_descriptor(&task_state_segment_descriptor);
    let task_register: x64::task::Register = task_state_segment_selector.into();
    task_register.set();
    let task_register = x64::task::Register::get();
    bsp_println!("task_register = {:#x?}", task_register);
    unimplemented!();
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    bsp_println!("APPLICATION PROCESSOR KERNEL PANIC!!!");
    bsp_println!("{}", panic);
    Argument::get_mut().kernel_complete();
    loop {
        x64::hlt();
    }
}

