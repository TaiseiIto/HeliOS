pub use register::Register;

use {
    alloc::{
        collections::{
            BTreeMap,
            BTreeSet,
        },
        vec::Vec,
    },
    core::{
        fmt,
        mem,
        ops::Range,
        slice,
    },
    super::{
        Interface,
        super::{
            long,
            short,
            Selector,
        },
    },
};

mod register;

/// Segment Descriptor Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 3.5.1 Segment Descriptor Tables
pub struct Table {
    descriptors: Vec<u64>,
}

impl Table {
    pub fn base(&self) -> u64 {
        self.descriptors
            .as_slice()
            .as_ptr() as u64
    }

    pub fn continuous_free_descriptor_indices(&self, length: usize) -> Option<Range<usize>> {
        let indices: Range<usize> = self.free_descriptor_indices()
            .into_iter()
            .fold(0..0, |indices, index| if indices.len() == length {
                indices
            } else if indices.end == index {
                indices.start..index + 1
            } else {
                index..index + 1
            });
        (indices.len() == length).then_some(indices)
    }

    pub fn descriptor(&self, selector: &Selector) -> Option<Interface> {
        self.selector2descriptor()
            .into_iter()
            .find_map(|(key_selector, interface)| (key_selector.get_index() == selector.get_index()).then_some(interface))
    }

    pub fn get() -> Self {
        Register::get().into()
    }

    pub fn limit(&self) -> u16 {
        let length: usize = self.descriptors.len();
        let size: usize = length * mem::size_of::<short::Descriptor>();
        let limit: usize = size - 1;
        limit as u16
    }

    pub fn selector2descriptor(&self) -> BTreeMap<Selector, Interface> {
        let long_descriptor_indices: BTreeSet<usize> = self.long_descriptor_indices();
        let short_descriptor_indices: BTreeSet<usize> = self.short_descriptor_indices();
        (0..self.descriptors.len())
            .filter_map(move |index| {
                let selector: u16 = (index * mem::size_of::<short::Descriptor>()) as u16;
                let selector: Selector = selector.into();
                if short_descriptor_indices.contains(&index) {
                    let descriptor: short::Descriptor = self.descriptors[index].into();
                    let descriptor: Option<Interface> = (&descriptor).into();
                    descriptor.map(|descriptor| (selector, descriptor))
                } else if long_descriptor_indices.contains(&index) {
                    let lower_descriptor: u64 = self.descriptors[index];
                    let higher_descriptor: u64 = self.descriptors[index + 1];
                    let descriptor: u128 = ((higher_descriptor as u128) << u64::BITS) + (lower_descriptor as u128);
                    let descriptor: long::Descriptor = descriptor.into();
                    let descriptor: Option<Interface> = (&descriptor).into();
                    descriptor.map(|descriptor| (selector, descriptor))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn set_descriptor(&mut self, index: usize, descriptor: &Interface) {
        if descriptor.is_long_descriptor() {
            let descriptor: long::Descriptor = descriptor.into();
            let descriptor: u128 = descriptor.into();
            let lower_descriptor: u64 = (descriptor & ((1 << u64::BITS) - 1)) as u64;
            let higher_descriptor: u64 = (descriptor >> u64::BITS) as u64;
            self.descriptors[index] = lower_descriptor;
            self.descriptors[index + 1] = higher_descriptor;
        } else if descriptor.is_short_descriptor() {
            let descriptor: short::Descriptor = descriptor.into();
            let descriptor: u64 = descriptor.into();
            self.descriptors[index] = descriptor;
        } else {
            panic!("Can't set a segment descriptor.");
        }
    }

    fn free_descriptor_indices(&self) -> BTreeSet<usize> {
        self.descriptors
            .iter()
            .enumerate()
            .fold((BTreeSet::<usize>::new(), false), |(mut free_descriptor_indices, previous_descriptor_is_lower_of_long), (index, descriptor)| if previous_descriptor_is_lower_of_long {
                (free_descriptor_indices, false)
            } else {
                let descriptor: short::Descriptor = (*descriptor).into();
                let interface: Option<Interface> = (&descriptor).into();
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

    fn long_descriptor_indices(&self) -> BTreeSet<usize> {
        self.descriptors
            .iter()
            .enumerate()
            .fold((BTreeSet::<usize>::new(), false), |(mut long_descriptor_indices, previous_descriptor_is_lower_of_long), (index, descriptor)| if previous_descriptor_is_lower_of_long {
                (long_descriptor_indices, false)
            } else {
                let descriptor: short::Descriptor = (*descriptor).into();
                let interface: Option<Interface> = (&descriptor).into();
                match interface {
                    Some(interface) => if interface.is_long_descriptor() {
                        long_descriptor_indices.insert(index);
                        (long_descriptor_indices, true)
                    } else {
                        (long_descriptor_indices, false)
                    },
                    None => (long_descriptor_indices, false),
                }
            })
            .0
    }

    fn short_descriptor_indices(&self) -> BTreeSet<usize> {
        self.descriptors
            .iter()
            .enumerate()
            .fold((BTreeSet::<usize>::new(), false), |(mut short_descriptor_indices, previous_descriptor_is_lower_of_short), (index, descriptor)| if previous_descriptor_is_lower_of_short {
                (short_descriptor_indices, false)
            } else {
                let descriptor: short::Descriptor = (*descriptor).into();
                let interface: Option<Interface> = (&descriptor).into();
                match interface {
                    Some(interface) => if interface.is_short_descriptor() {
                        short_descriptor_indices.insert(index);
                        (short_descriptor_indices, false)
                    } else {
                        (short_descriptor_indices, true)
                    },
                    None => (short_descriptor_indices, false),
                }
            })
            .0
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries(self.selector2descriptor())
            .finish()
    }
}

impl From<Register> for Table {
    fn from(register: Register) -> Self {
        let descriptors: &[u64] = unsafe {
            slice::from_raw_parts(register.base(), register.length())
        };
        let descriptors: Vec<u64> = (u16::MIN..=u16::MAX)
            .step_by(mem::size_of::<short::Descriptor>())
            .map(|segment_selector| *descriptors
                .get(segment_selector as usize / mem::size_of::<short::Descriptor>())
                .unwrap_or(&0))
            .collect();
        Self {
            descriptors,
        }
    }
}

