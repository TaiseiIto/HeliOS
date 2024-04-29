use {
    alloc::{
        vec::Vec,
        string::String,
    },
    core::{
        fmt,
        mem,
        ops::Range,
        slice,
    },
    crate::{
        com2_print,
        com2_println,
        memory,
        x64,
    },
};

pub struct Loader {
    program_address_range: Range<usize>,
    stack_address_range: Range<usize>,
}

impl Loader {
    pub fn entry_point(&self) -> usize {
        self.program_address_range.start
    }

    pub fn initialize(&mut self, paging: &memory::Paging, kernel_entry: usize, kernel_stack_floor: usize, my_local_apic_id: u8) {
        self.initialize_stack();
        self.set_arguments(paging, kernel_entry, kernel_stack_floor, my_local_apic_id);
    }

    pub fn log(&self) -> String {
        let log: Vec<u8> = self.stack()
            .iter()
            .copied()
            .take_while(|byte| *byte != 0)
            .collect();
        String::from_utf8(log).unwrap()
    }

    pub fn program(&self) -> &[u8] {
        let start: *const u8 = self.program_address_range.start as *const u8;
        let length: usize = self.program_address_range.end - self.program_address_range.start;
        unsafe {
            slice::from_raw_parts(start, length)
        }
    }

    pub fn stack(&self) -> &[u8] {
        let start: *const u8 = self.stack_address_range.start as *const u8;
        let length: usize = self.stack_address_range.end - self.stack_address_range.start;
        unsafe {
            slice::from_raw_parts(start, length)
        }
    }

    fn arguments_mut(&mut self) -> &mut Arguments {
        let arguments: usize = self.program_address_range.end - mem::size_of::<Arguments>();
        let arguments: *mut Arguments = arguments as *mut Arguments;
        unsafe {
            &mut *arguments
        }
    }

    fn initialize_stack(&mut self) {
        self.stack_mut()
            .iter_mut()
            .for_each(|byte| *byte = 0)
    }

    fn set_arguments(&mut self, paging: &memory::Paging, kernel_entry: usize, kernel_stack_floor: usize, my_local_apic_id: u8) {
        *self.arguments_mut() = Arguments::new(paging, kernel_entry, kernel_stack_floor, my_local_apic_id);
    }

    fn stack_mut(&mut self) -> &mut [u8] {
        let start: *mut u8 = self.stack_address_range.start as *mut u8;
        let length: usize = self.stack_address_range.end - self.stack_address_range.start;
        unsafe {
            slice::from_raw_parts_mut(start, length)
        }
    }
}

impl fmt::Debug for Loader {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Loader")
            .field("program", &self.program())
            .field("stack", &self.stack())
            .finish()
    }
}

#[repr(packed)]
struct Arguments {
    #[allow(dead_code)]
    cr3: u64,
    kernel_entry: usize,
    kernel_stack_floor: usize,
    bsp_local_apic_id: u8,
}

impl Arguments {
    pub fn new(paging: &memory::Paging, kernel_entry: usize, kernel_stack_floor: usize, bsp_local_apic_id: u8) -> Self {
        let cr3: u64 = paging.cr3().into();
        com2_println!("cr3 = {:#x?}", cr3);
        Self {
            cr3,
            kernel_entry,
            kernel_stack_floor,
            bsp_local_apic_id,
        }
    }
}

