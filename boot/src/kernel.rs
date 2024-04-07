use {
    alloc::collections::BTreeMap,
    crate::{
        processor,
        efi,
        elf,
        memory,
        rs232c,
        x64,
    },
};

#[derive(Debug)]
pub struct Argument<'a> {
    #[allow(dead_code)]
    processor_boot_loader: processor::boot::Loader,
    #[allow(dead_code)]
    com2: &'a mut rs232c::Com,
    #[allow(dead_code)]
    cpuid: x64::Cpuid,
    #[allow(dead_code)]
    efi_system_table: &'a mut efi::SystemTable<'a>,
    #[allow(dead_code)]
    fonts: BTreeMap<usize, efi::Font<'a>>,
    #[allow(dead_code)]
    graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
    #[allow(dead_code)]
    heap_start: usize,
    #[allow(dead_code)]
    hello_application: elf::File,
    #[allow(dead_code)]
    memory_map: efi::memory::Map,
    #[allow(dead_code)]
    my_processor_number: Option<usize>,
    #[allow(dead_code)]
    paging: memory::Paging,
    #[allow(dead_code)]
    processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation>,
}

impl<'a> Argument<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        processor_boot_loader: processor::boot::Loader,
        com2: &'a mut rs232c::Com,
        cpuid: x64::Cpuid,
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
            processor_boot_loader,
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

