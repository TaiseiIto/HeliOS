use {
    alloc::{
        boxed::Box,
        collections::BTreeMap,
        vec::Vec,
    },
    core::ops::Range,
    crate::{
        com2_println,
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
        loader: &Loader,
        memory_map: efi::memory::Map,
        paging: memory::Paging,
    ) -> Self {
        let heap_start: usize = loader.heap_start();
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

pub struct Loader {
    elf: elf::File,
    #[allow(dead_code)]
    elf_vaddr2frame: BTreeMap<usize, Box<memory::Frame>>,
    #[allow(dead_code)]
    stack_vaddr2frame: BTreeMap<usize, Box<memory::Frame>>,
    stack_floor: usize,
    heap_start: usize,
}

impl Loader {
    pub fn new(path: &str, directory_tree: &efi::file::system::Tree, paging: &mut memory::Paging) -> Self {
        let elf: elf::File = directory_tree
            .get(path)
            .unwrap()
            .read()
            .into();
        let elf_vaddr2frame: BTreeMap<usize, Box<memory::Frame>> = elf.deploy(paging);
        let stack_pages: usize = 0x200;
        let stack_vaddr2frame: BTreeMap<usize, Box<memory::Frame>> = (0..stack_pages)
            .map(|stack_page_index| (usize::MAX - (stack_page_index + 1) * memory::page::SIZE + 1, Box::default()))
            .collect();
        stack_vaddr2frame
            .iter()
            .for_each(|(vaddr, frame)| {
                let present: bool = true;
                let writable: bool = true;
                let executable: bool = false;
                paging.set_page(*vaddr, frame.paddr(), present, writable, executable);
            });
        let stack_floor: usize = 0;
        let higher_half_range: Range<u128> = paging.higher_half_range();
        let heap_start: u128 = (higher_half_range.start + higher_half_range.end) / 2;
        let heap_start: usize = heap_start as usize;
        let heap_pages: usize = efi::SystemTable::get()
            .memory_map()
            .unwrap()
            .iter()
            .filter(|memory_descriptor| memory_descriptor.is_available())
            .map(|memory_descriptor| memory_descriptor.number_of_pages())
            .sum();
        (0..heap_pages)
            .for_each(|heap_page_index| {
                let vaddr: usize = heap_start + heap_page_index * memory::page::SIZE;
                let paddr: usize = 0;
                let present: bool = false;
                let writable: bool = false;
                let executable: bool = false;
                paging.set_page(vaddr, paddr, present, writable, executable);
            });
        Self {
            elf,
            elf_vaddr2frame,
            stack_vaddr2frame,
            stack_floor,
            heap_start,
        }
    }

    pub fn run(&self, argument: &Argument) {
        self.elf.run(self.stack_floor, argument);
    }

    fn heap_start(&self) -> usize {
        self.heap_start
    }
}

