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
    application_code_segment_index: usize,
    application_data_segment_index: usize,
    com2: &'a mut rs232c::Com,
    cpuid: Option<x64::Cpuid>,
    efi_system_table: &'a mut efi::SystemTable<'a>,
    fonts: BTreeMap<usize, efi::Font<'a>>,
    gdt: memory::segment::descriptor::Table,
    graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
    heap_start: usize,
    idt: interrupt::descriptor::Table,
    kernel_code_segment_index: usize,
    kernel_data_segment_index: usize,
    memory_map: efi::memory::Map,
    my_processor_number: Option<usize>,
    paging: memory::Paging,
    processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation>,
}

impl<'a> Argument<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        application_code_segment_index: usize,
        application_data_segment_index: usize,
        com2: &'a mut rs232c::Com,
        cpuid: Option<x64::Cpuid>,
        efi_system_table: &'a mut efi::SystemTable<'a>,
        fonts: BTreeMap<usize, efi::Font<'a>>,
        gdt: memory::segment::descriptor::Table,
        graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
        heap_start: usize,
        idt: interrupt::descriptor::Table,
        kernel_code_segment_index: usize,
        kernel_data_segment_index: usize,
        memory_map: efi::memory::Map,
        my_processor_number: Option<usize>,
        paging: memory::Paging,
        processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation>,
    ) -> Self {
        Self {
            application_code_segment_index,
            application_data_segment_index,
            com2,
            cpuid,
            efi_system_table,
            fonts,
            gdt,
            graphics_output_protocol,
            heap_start,
            idt,
            kernel_code_segment_index,
            kernel_data_segment_index,
            memory_map,
            my_processor_number,
            processor_informations,
            paging,
        }
    }
}

