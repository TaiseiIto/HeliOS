use {
    alloc::collections::BTreeMap,
    crate::{
        efi,
        interrupt,
        memory,
        rs232c,
        x64,
    },
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Argument<'a> {
    com2: &'a mut rs232c::Com,
    cpuid: Option<x64::Cpuid>,
    efi_system_table: &'a mut efi::SystemTable<'a>,
    fonts: BTreeMap<usize, efi::Font<'a>>,
    gdt: memory::segment::descriptor::Table,
    graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
    heap_base: usize,
    idt: interrupt::descriptor::Table,
    memory_map: efi::memory::Map,
    my_processor_number: Option<usize>,
    processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation>,
    paging: memory::Paging,
}

impl<'a> Argument<'a> {
    pub fn new(
        com2: &'a mut rs232c::Com,
        cpuid: Option<x64::Cpuid>,
        efi_system_table: &'a mut efi::SystemTable<'a>,
        fonts: BTreeMap<usize, efi::Font<'a>>,
        gdt: memory::segment::descriptor::Table,
        graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
        heap_base: usize,
        idt: interrupt::descriptor::Table,
        memory_map: efi::memory::Map,
        my_processor_number: Option<usize>,
        processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation>,
        paging: memory::Paging
    ) -> Self {
        Self {
            com2,
            cpuid,
            efi_system_table,
            fonts,
            gdt,
            graphics_output_protocol,
            heap_base,
            idt,
            memory_map,
            my_processor_number,
            processor_informations,
            paging,
        }
    }
}

