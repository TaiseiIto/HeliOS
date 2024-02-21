pub use register::Register;

use {
    alloc::{
        collections::BTreeSet,
        vec::Vec,
    },
    core::{
        fmt,
        iter,
        mem,
        slice,
    },
    crate::x64,
    super::{
        Interface,
        super::{
            Descriptor,
            Selector,
        },
    },
};

mod register;

/// Segment Descriptor Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 3.5.1 Segment Descriptor Tables
pub struct Table {
    descriptors: Vec<Descriptor>,
}

impl Table {
    pub fn base(&self) -> u64 {
        self.descriptors
            .as_slice()
            .as_ptr() as u64
    }

    #[allow(dead_code)]
    pub fn get() -> Self {
        Register::get().into()
    }

    pub fn limit(&self) -> u16 {
        let length: usize = self.descriptors.len();
        let size: usize = length * mem::size_of::<Descriptor>();
        let limit: usize = size - 1;
        limit as u16
    }

    pub fn set_task_state_segment_descriptor(&mut self, task_state_segment_descriptor: &x64::task::state::segment::Descriptor) -> Selector {
        let task_state_segment_descriptor: u128 = (*task_state_segment_descriptor).into();
        let lower_descriptor: u64 = (task_state_segment_descriptor & ((1 << u64::BITS) - 1)) as u64;
        let lower_descriptor: Descriptor = lower_descriptor.into();
        let higher_descriptor: u64 = (task_state_segment_descriptor >> u64::BITS) as u64;
        let higher_descriptor: Descriptor = higher_descriptor.into();
        let free_descriptor_indices: Vec<usize> = self
            .free_descriptor_indices()
            .into_iter()
            .collect();
        let free_descriptor_indices: &[usize] = free_descriptor_indices.as_slice();
        let lower_descriptor_indices: &[usize] = &free_descriptor_indices[..free_descriptor_indices.len() - 1];
        let higher_descriptor_indices: &[usize] = &free_descriptor_indices[1..];
        let (lower_descriptor_index, higher_descriptor_index): (&usize, &usize) = lower_descriptor_indices
            .iter()
            .zip(higher_descriptor_indices.iter())
            .find(|(lower_descriptor_index, higher_descriptor_index)| *lower_descriptor_index + 1 == **higher_descriptor_index)
            .unwrap();
        self.descriptors[*lower_descriptor_index] = lower_descriptor;
        self.descriptors[*higher_descriptor_index] = higher_descriptor;
        let segment_selector: u16 = (lower_descriptor_index * mem::size_of::<Descriptor>()) as u16;
        segment_selector.into()
    }

    fn free_descriptor_indices(&self) -> BTreeSet<usize> {
        self.descriptors
            .iter()
            .enumerate()
            .fold((BTreeSet::<usize>::new(), false), |(free_descriptor_indices, previous_descriptor_is_lower_of_long), (index, descriptor)| if previous_descriptor_is_lower_of_long {
                (free_descriptor_indices, false)
            } else {
                let interface: Option<Interface> = descriptor.into();
                match interface {
                    Some(interface) => if interface.is_long_descriptor() {
                        (free_descriptor_indices, true)
                    } else {
                        (free_descriptor_indices, false)
                    },
                    None => {
                        free_descriptor_indices.insert(index);
                        (free_descriptor_indices, false)
                    },
                }
            })
            .0
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries(self.descriptors
                .iter()
                .enumerate()
                .filter_map(|(index, descriptor)| {
                    let selector: u16 = (index * mem::size_of::<Descriptor>()) as u16;
                    let descriptor: Option<Interface> = descriptor.into();
                    descriptor.map(|descriptor| (selector, descriptor))
                }))
            .finish()
    }
}

impl From<Register> for Table {
    fn from(register: Register) -> Self {
        let descriptors: &[Descriptor] = unsafe {
            slice::from_raw_parts(register.base(), register.length())
        };
        let descriptors: Vec<Descriptor> = (u16::MIN..=u16::MAX)
            .step_by(mem::size_of::<Descriptor>())
            .map(|segment_selector| *descriptors
                .get(segment_selector as usize / mem::size_of::<Descriptor>())
                .unwrap_or(&Descriptor::default()))
            .collect();
        Self {
            descriptors,
        }
    }
}

