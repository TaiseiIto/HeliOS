//! # The bootloader

#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

extern crate alloc;

mod application;
mod efi;
mod elf;
mod interrupt;
mod kernel;
mod memory;
mod rs232c;
mod x64;

use {
    alloc::{
        boxed::Box,
        collections::BTreeMap,
        vec::Vec,
    },
    core::{
        panic::PanicInfo,
        ops::Range,
    },
};

/// # The entry point of the OS
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.1 UEFI Image Entry Point
#[no_mangle]
fn efi_main(image_handle: efi::Handle, system_table: &'static mut efi::SystemTable<'static>) -> efi::Status {
    system_table.set();
    com2_println!("image_handle = {:#x?}", image_handle);
    com2_println!("system_table = {:#x?}", efi::SystemTable::get());
    let font_protocol = efi::font::Protocol::get();
    com2_println!("font_protocol = {:#x?}", font_protocol);
    let fonts: BTreeMap<usize, efi::Font> = font_protocol.fonts();
    let graphics_output_protocol = efi::graphics_output::Protocol::get();
    com2_println!("graphics_output_protocol = {:#x?}", graphics_output_protocol);
    let mp_services_protocol = efi::mp_services::Protocol::get();
    com2_println!("mp_services_protocol = {:#x?}", mp_services_protocol);
    let my_processor_number: Option<usize> = mp_services_protocol.my_processor_number().ok();
    com2_println!("my_processor_number = {:#x?}", my_processor_number);
    let processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation> = mp_services_protocol.get_all_processor_informations();
    com2_println!("processor_informations = {:#x?}", processor_informations);
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
    let kernel_code_segment_selector = memory::segment::Selector::create(kernel_code_segment_index as u16, is_ldt, kernel::PRIVILEGE_LEVEL);
    com2_println!("kernel_code_segment_selector = {:#x?}", kernel_code_segment_selector);
    let kernel_data_segment_selector = memory::segment::Selector::create(kernel_data_segment_index as u16, is_ldt, kernel::PRIVILEGE_LEVEL);
    com2_println!("kernel_data_segment_selector = {:#x?}", kernel_data_segment_selector);
    let idt = interrupt::descriptor::Table::get();
    com2_println!("idt = {:#x?}", idt);
    let idtr: interrupt::descriptor::table::Register = (&idt).into();
    com2_println!("idtr = {:#x?}", idtr);
    idtr.set();
    let cpuid: Option<x64::Cpuid> = x64::Cpuid::get();
    let execute_disable_bit_available: bool = x64::msr::Ia32Efer::enable_execute_disable_bit(&cpuid);
    com2_println!("execute_disable_bit_available = {:#x?}", execute_disable_bit_available);
    let mut paging = memory::Paging::get(&cpuid);
    paging.set();
    let directory_tree: efi::file::system::Tree = efi::file::system::Protocol::get().tree();
    let kernel: elf::File = directory_tree
        .get("HeliOS/kernel.elf")
        .unwrap()
        .read()
        .into();
    let kernel_vaddr2frame: BTreeMap<usize, Box<memory::Frame>> = kernel.deploy(&mut paging);
    com2_println!("kernel_vaddr2frame = {:#x?}", kernel_vaddr2frame);
    let kernel_stack_pages: usize = 0x400;
    let kernel_stack_vaddr2frame: BTreeMap<usize, Box<memory::Frame>> = (0..kernel_stack_pages)
        .map(|kernel_stack_page_index| (usize::MAX - (kernel_stack_page_index + 1) * memory::page::SIZE + 1, Box::default()))
        .collect();
    kernel_stack_vaddr2frame
        .iter()
        .for_each(|(vaddr, frame)| {
            let present: bool = true;
            let writable: bool = true;
            let executable: bool = false;
            paging.set_page(*vaddr, frame.paddr(), present, writable, executable);
        });
    let kernel_stack_floor: usize = 0;
    efi_println!("Hello, World!");
    let memory_map: Vec<efi::memory::Descriptor> = efi::SystemTable::get()
        .memory_map()
        .unwrap()
        .into();
    let higher_half_range: Range<u128> = paging.higher_half_range();
    let kernel_heap_start: u128 = (higher_half_range.start + higher_half_range.end) / 2;
    let kernel_heap_start: usize = kernel_heap_start as usize;
    com2_println!("kernel_heap_start = {:#x?}", kernel_heap_start);
    let kernel_heap_pages: usize = memory_map
        .into_iter()
        .filter(|memory_descriptor| memory_descriptor.is_available())
        .map(|memory_descriptor| memory_descriptor.number_of_pages())
        .sum();
    (0..kernel_heap_pages)
        .for_each(|heap_page_index| {
            let vaddr: usize = kernel_heap_start + heap_page_index * memory::page::SIZE;
            let paddr: usize = 0;
            let present: bool = false;
            let writable: bool = false;
            let executable: bool = false;
            paging.set_page(vaddr, paddr, present, writable, executable);
        });
    let memory_map: efi::memory::Map = efi::SystemTable::get()
        .exit_boot_services(image_handle)
        .unwrap();
    let kernel_argument = kernel::Argument::new(
        application_code_segment_index,
        application_data_segment_index,
        rs232c::get_com2(),
        cpuid,
        efi::SystemTable::get(),
        fonts,
        gdt,
        graphics_output_protocol,
        kernel_heap_start,
        idt,
        kernel_code_segment_index,
        kernel_data_segment_index,
        memory_map,
        my_processor_number,
        paging,
        processor_informations);
    kernel.run(kernel_stack_floor, &kernel_argument);
    efi::SystemTable::get().shutdown();
    efi::Status::ABORTED
}

/// # A panic handler of the boot loader
#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    com2_println!("BOOT PANIC!!!");
    com2_println!("{}", panic);
    loop {
        x64::hlt();
    }
}

