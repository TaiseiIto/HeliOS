use {
    alloc::{
        collections::BTreeMap,
        vec::Vec,
    },
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
    processor_kernel: Vec<u8>,
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
    memory_map: efi::memory::Map,
    #[allow(dead_code)]
    paging: memory::Paging,
}

impl<'a> Argument<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        processor_boot_loader: processor::boot::Loader,
        processor_kernel: Vec<u8>,
        com2: &'a mut rs232c::Com,
        cpuid: x64::Cpuid,
        efi_system_table: &'a mut efi::SystemTable<'a>,
        fonts: BTreeMap<usize, efi::Font<'a>>,
        graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
        heap_start: usize,
        memory_map: efi::memory::Map,
        paging: memory::Paging,
    ) -> Self {
        Self {
            processor_boot_loader,
            processor_kernel,
            com2,
            cpuid,
            efi_system_table,
            fonts,
            graphics_output_protocol,
            heap_start,
            memory_map,
            paging,
        }
    }
}

