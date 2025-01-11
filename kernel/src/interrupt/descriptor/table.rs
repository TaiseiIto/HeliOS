pub mod register;

use {
    alloc::{
        boxed::Box,
        vec::Vec,
    },
    core::{
        fmt,
        mem::size_of,
        slice,
    },
    crate::{
        Argument,
        memory,
        x64,
    },
    super::{
        Interface,
        super::{
            Descriptor,
            register_handlers,
        },
    },
};

pub use register::Register;

/// # Interrupt Descriptor
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.10 Interrupt Descriptor Table (IDT)
pub struct Table {
    descriptors: Vec<Descriptor>,
}

impl Table {
    pub fn base(&self) -> u64 {
        self.descriptors
            .as_slice()
            .as_ptr() as u64
    }

    pub fn get() -> Self {
        Register::get().into()
    }

    pub fn initialize(gdt: &mut memory::segment::descriptor::table::Controller) {
        let mut idt = Table::get();
        register_handlers(&mut idt);
        let idtr: Register = (&idt).into();
        idtr.set();
        let interrupt_stacks: Vec<memory::Stack> = (0..x64::task::state::Segment::NUMBER_OF_INTERRUPT_STACKS + x64::task::state::Segment::NUMBER_OF_STACK_POINTERS)
            .map(|index| {
                let pages: usize = 0x10;
                let floor_inclusive: usize = Argument::get().heap_start() - (2 * index + 1) * pages * memory::page::SIZE - 1;
                memory::Stack::new(Argument::get().paging_mut(), floor_inclusive, pages)
            })
            .collect();
        let task_state_segment_and_io_permission_bit_map: Box<x64::task::state::segment::AndIoPermissionBitMap> = x64::task::state::segment::AndIoPermissionBitMap::new(&interrupt_stacks);
        let task_state_segment_descriptor: memory::segment::long::Descriptor = (task_state_segment_and_io_permission_bit_map.as_ref()).into();
        let task_state_segment_selector: memory::segment::Selector = gdt.set_task_state_segment_descriptor(&task_state_segment_descriptor);
        let task_register: x64::task::Register = task_state_segment_selector.into();
        task_register.set();
        let task_register = x64::task::Register::get();
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Descriptor> {
        self.descriptors.iter_mut()
    }

    pub fn limit(&self) -> u16 {
        let length: usize = self.descriptors.len();
        let size: usize = length * size_of::<Descriptor>();
        let limit: usize = size - 1;
        limit as u16
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries(self.descriptors
                .iter()
                .enumerate()
                .filter_map(|(interrupt_number, descriptor)| {
                    let descriptor: Option<Interface> = descriptor.into();
                    descriptor.map(|descriptor| (interrupt_number, descriptor))
                }))
            .finish()
    }
}

impl From<Register> for Table {
    fn from(register: Register) -> Self {
        let descriptors: &[Descriptor] = unsafe {
            slice::from_raw_parts(register.base(), register.length())
        };
        let descriptors: Vec<Descriptor> = (u8::MIN..=u8::MAX)
            .map(|interrupt_number| *descriptors
                .get(interrupt_number as usize)
                .unwrap_or(&Descriptor::default()))
            .collect();
        Self {
            descriptors,
        }
    }
}

