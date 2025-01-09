//! # Kernel argumnents.

use {
    alloc::{
        collections::BTreeMap,
        vec::Vec,
    },
    core::cell::OnceCell,
    crate::{
        efi,
        elf,
        memory,
        processor,
        rs232c,
        x64,
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
    memory_map: efi::memory::Map,
    paging: memory::Paging,
}

impl Argument<'static> {
    pub fn com2_mut(&mut self) -> &mut rs232c::Com {
        self.com2
    }

    pub fn cpuid(&self) -> &x64::Cpuid {
        &self.cpuid
    }

    pub fn efi_system_table(&self) -> &efi::SystemTable {
        self.efi_system_table
    }

    pub fn efi_system_table_mut(&'static mut self) -> &'static mut efi::SystemTable {
        self.efi_system_table
    }

    pub fn get() -> &'static mut Self {
        unsafe {
            ARGUMENT.get_mut()
        }.unwrap()
    }

    pub fn heap_start(&self) -> usize {
        self.heap_start
    }

    pub fn memory_map(&self) -> &efi::memory::Map {
        &self.memory_map
    }

    pub fn paging(&self) -> &memory::Paging {
        &self.paging
    }

    pub fn paging_mut(&mut self) -> &mut memory::Paging {
        &mut self.paging
    }

    pub fn processor_boot_loader_mut(&mut self) -> &mut processor::boot::Loader {
        &mut self.processor_boot_loader
    }

    pub fn processor_kernel(&self) -> &[u8] {
        &self.processor_kernel
    }

    pub fn set(&'static mut self) {
        unsafe {
            ARGUMENT.set(self)
        }.unwrap();
        rs232c::set_com2(Self::get().com2_mut());
    }
}

