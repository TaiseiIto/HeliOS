use {
    alloc::collections::BTreeMap,
    crate::{
        efi,
        elf,
        memory,
        rs232c,
        x64,
    },
};

pub const PRIVILEGE_LEVEL: u8 = 0;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Argument<'a> {
    com2: &'a mut rs232c::Com,
    cpuid: Option<x64::Cpuid>,
    efi_system_table: &'a mut efi::SystemTable<'a>,
    fonts: BTreeMap<usize, efi::Font<'a>>,
    graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
    heap_start: usize,
    hello_application: elf::File,
    memory_map: efi::memory::Map,
    my_processor_number: Option<usize>,
    paging: memory::Paging,
    processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation>,
}

impl<'a> Argument<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        com2: &'a mut rs232c::Com,
        cpuid: Option<x64::Cpuid>,
        efi_system_table: &'a mut efi::SystemTable<'a>,
        fonts: BTreeMap<usize, efi::Font<'a>>,
        graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
        heap_start: usize,
        hello_application: elf::File,
        memory_map: efi::memory::Map,
        my_processor_number: Option<usize>,
        paging: memory::Paging,
        processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation>,
    ) -> Self {
        Self {
            com2,
            cpuid,
            efi_system_table,
            fonts,
            graphics_output_protocol,
            heap_start,
            hello_application,
            memory_map,
            my_processor_number,
            paging,
            processor_informations,
        }
    }
}

