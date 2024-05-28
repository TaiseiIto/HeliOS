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
        sync,
    },
    super::message,
};

pub struct Loader {
    program_address_range: Range<usize>,
    stack_address_range: Range<usize>,
}

impl Loader {
    pub fn entry_point(&self) -> usize {
        self.program_address_range.start
    }

    pub fn initialize(&mut self, paging: &memory::Paging, kernel_entry: usize, kernel_stack_floor: usize, my_local_apic_id: u8, message: &sync::spin::Lock<Option<message::Content>>) {
        self.initialize_stack();
        self.set_arguments(paging, kernel_entry, kernel_stack_floor, my_local_apic_id, message);
        self.set_temporary_pml4_table(paging);
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

    fn set_arguments(&mut self, paging: &memory::Paging, kernel_entry: usize, kernel_stack_floor: usize, my_local_apic_id: u8, message: &sync::spin::Lock<Option<message::Content>>) {
        *self.arguments_mut() = Arguments::new(paging, kernel_entry, kernel_stack_floor, my_local_apic_id, message);
    }

    fn set_temporary_pml4_table(&mut self, paging: &memory::Paging) {
        self.temporary_pml4_table_mut()
            .copy_from_slice(paging.table())
    }

    fn stack_mut(&mut self) -> &mut [u8] {
        let start: *mut u8 = self.stack_address_range.start as *mut u8;
        let length: usize = self.stack_address_range.end - self.stack_address_range.start;
        unsafe {
            slice::from_raw_parts_mut(start, length)
        }
    }

    fn temporary_pml4_table_mut(&mut self) -> &mut [u8] {
        let temporary_pml4_table: usize = self.program_address_range.end - mem::size_of::<Arguments>() - memory::page::SIZE;
        com2_println!("temporary_pml4_table = {:#x?}", temporary_pml4_table);
        let temporary_pml4_table: *mut u8 = temporary_pml4_table as *mut u8;
        let length: usize = memory::page::SIZE;
        unsafe {
            slice::from_raw_parts_mut(temporary_pml4_table, length)
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
    #[allow(dead_code)]
    kernel_entry: usize,
    #[allow(dead_code)]
    kernel_stack_floor: usize,
    #[allow(dead_code)]
    message: usize,
    #[allow(dead_code)]
    bsp_local_apic_id: u8,
}

impl Arguments {
    pub fn new(paging: &memory::Paging, kernel_entry: usize, kernel_stack_floor: usize, bsp_local_apic_id: u8, message: &sync::spin::Lock<Option<message::Content>>) -> Self {
        let message: *const sync::spin::Lock<Option<message::Content>> = message as *const sync::spin::Lock<Option<message::Content>>;
        let message: usize = message as usize;
        let cr3: u64 = paging.cr3().into();
        com2_println!("cr3 = {:#x?}", cr3);
        Self {
            cr3,
            kernel_entry,
            kernel_stack_floor,
            message,
            bsp_local_apic_id,
        }
    }
}

