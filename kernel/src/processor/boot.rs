use {
    super::{message, Controller},
    crate::{com2_println, memory, sync},
    alloc::{string::String, vec::Vec},
    core::{
        fmt,
        mem::{size_of, MaybeUninit},
        ops::Range,
        ptr, slice,
    },
};

pub mod real_mode;

pub struct Loader {
    program_address_range: Range<usize>,
    stack_address_range: Range<usize>,
}

impl Loader {
    pub fn entry_point(&self) -> usize {
        self.program_address_range.start
    }

    pub fn initialize(
        &mut self,
        controller: &Controller,
        bsp_heap_start: usize,
        bsp_local_apic_id: u8,
    ) {
        self.initialize_stack();
        self.set_arguments(controller, bsp_heap_start, bsp_local_apic_id);
        self.set_temporary_pml4_table(controller);
    }

    pub fn log(&self) -> String {
        let log: Vec<u8> = self
            .stack()
            .iter()
            .copied()
            .take_while(|byte| *byte != 0)
            .collect();
        String::from_utf8(log).unwrap()
    }

    pub fn program(&self) -> Vec<u8> {
        self.program_address_range
            .clone()
            .map(|program_address| program_address as *const u8)
            .map(|program_address| unsafe { ptr::read_volatile(program_address) })
            .collect()
    }

    pub fn stack(&self) -> Vec<u8> {
        self.stack_address_range
            .clone()
            .map(|stack_address| stack_address as *const u8)
            .map(|stack_address| unsafe { ptr::read_volatile(stack_address) })
            .collect()
    }

    fn arguments_mut(&mut self) -> &mut Arguments {
        let arguments: usize = self.program_address_range.end - size_of::<Arguments>();
        let arguments: *mut Arguments = arguments as *mut Arguments;
        unsafe { &mut *arguments }
    }

    fn initialize_stack(&mut self) {
        self.stack_mut().iter_mut().for_each(|byte| *byte = 0)
    }

    fn set_arguments(
        &mut self,
        controller: &Controller,
        bsp_heap_start: usize,
        bsp_local_apic_id: u8,
    ) {
        *self.arguments_mut() = Arguments::new(self, controller, bsp_heap_start, bsp_local_apic_id);
    }

    fn set_temporary_pml4_table(&mut self, controller: &Controller) {
        self.temporary_pml4_table_mut()
            .copy_from_slice(controller.paging().table())
    }

    fn stack_mut(&mut self) -> &mut [u8] {
        let start: *mut u8 = self.stack_address_range.start as *mut u8;
        let length: usize = self.stack_address_range.end - self.stack_address_range.start;
        unsafe { slice::from_raw_parts_mut(start, length) }
    }

    fn ss(&self) -> u16 {
        let stack_floor: usize = self.stack_address_range.end;
        let stack_ceil: usize = stack_floor - real_mode::segment::SIZE;
        let ss: usize = stack_ceil >> real_mode::segment::SHIFT;
        ss as u16
    }

    fn temporary_pml4_table_mut(&mut self) -> &mut [u8] {
        let temporary_pml4_table: usize =
            self.program_address_range.end - size_of::<Arguments>() - memory::page::SIZE;
        com2_println!("temporary_pml4_table = {:#x?}", temporary_pml4_table);
        let temporary_pml4_table: *mut u8 = temporary_pml4_table as *mut u8;
        let length: usize = memory::page::SIZE;
        unsafe { slice::from_raw_parts_mut(temporary_pml4_table, length) }
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
    bsp_heap_start: usize,
    #[allow(dead_code)]
    heap_start: usize,
    #[allow(dead_code)]
    heap_size: usize,
    #[allow(dead_code)]
    receiver: usize,
    #[allow(dead_code)]
    sender: usize,
    #[allow(dead_code)]
    ss: u16,
    #[allow(dead_code)]
    bsp_local_apic_id: u8,
}

impl Arguments {
    pub fn new(
        loader: &Loader,
        controller: &Controller,
        bsp_heap_start: usize,
        bsp_local_apic_id: u8,
    ) -> Self {
        let paging: &memory::Paging = controller.paging();
        let cr3: u64 = paging.cr3().into();
        let kernel_entry: usize = controller.kernel_entry();
        let kernel_stack_floor: usize = controller.kernel_stack_floor();
        let heap: &[MaybeUninit<u8>] = controller.heap();
        let heap_start: usize = heap.as_ptr() as usize;
        let heap_size: usize = heap.len();
        let receiver: &sync::spin::Lock<Option<message::Content>> = controller.receiver();
        let receiver: *const sync::spin::Lock<Option<message::Content>> =
            receiver as *const sync::spin::Lock<Option<message::Content>>;
        let receiver: usize = receiver as usize;
        let sender: &sync::spin::Lock<Option<message::Content>> = controller.sender();
        let sender: *const sync::spin::Lock<Option<message::Content>> =
            sender as *const sync::spin::Lock<Option<message::Content>>;
        let sender: usize = sender as usize;
        let ss: u16 = loader.ss();
        Self {
            cr3,
            kernel_entry,
            kernel_stack_floor,
            bsp_heap_start,
            heap_start,
            heap_size,
            receiver,
            sender,
            ss,
            bsp_local_apic_id,
        }
    }
}
