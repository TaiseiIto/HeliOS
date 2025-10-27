pub use register::Register;

use {
    super::{
        super::{long, short, Selector},
        Interface,
    },
    crate::{com2_println, x64},
    alloc::{
        collections::{BTreeMap, BTreeSet},
        vec::Vec,
    },
    core::{fmt, mem, ops::Range, slice},
};

mod register;

const APPLICATION_PRIVILEGE_LEVEL: u8 = 3;
const KERNEL_PRIVILEGE_LEVEL: u8 = 0;

pub struct Controller {
    table: Table,
    application_code_segment_selector: Selector,
    application_data_segment_selector: Selector,
    kernel_code_segment_selector: Selector,
    kernel_data_segment_selector: Selector,
}

impl Controller {
    pub fn new() -> Self {
        let mut table = Table::get();
        let register: Register = (&table).into();
        register.set();
        let cs = Selector::cs();
        let ds = Selector::ds();
        let kernel_code_segment_descriptor: Interface = table.descriptor(&cs).unwrap();
        let kernel_data_segment_descriptor: Interface = table.descriptor(&ds).unwrap();
        let application_code_segment_descriptor: Interface =
            kernel_code_segment_descriptor.with_dpl(APPLICATION_PRIVILEGE_LEVEL);
        let application_data_segment_descriptor: Interface =
            kernel_data_segment_descriptor.with_dpl(APPLICATION_PRIVILEGE_LEVEL);
        let segment_descriptors = [
            kernel_code_segment_descriptor,
            kernel_data_segment_descriptor,
            application_data_segment_descriptor,
            application_code_segment_descriptor,
        ];
        let segment_descriptors: &[Interface] = segment_descriptors.as_slice();
        let mut segment_descriptor_indices: Range<usize> = table
            .continuous_free_descriptor_indices(segment_descriptors.len())
            .unwrap();
        segment_descriptor_indices
            .clone()
            .zip(segment_descriptors.iter())
            .for_each(|(index, descriptor)| table.set_descriptor(index, descriptor));
        let kernel_code_segment_index: usize = segment_descriptor_indices.next().unwrap();
        let kernel_data_segment_index: usize = segment_descriptor_indices.next().unwrap();
        let application_data_segment_index: usize = segment_descriptor_indices.next().unwrap();
        let application_code_segment_index: usize = segment_descriptor_indices.next().unwrap();
        let is_ldt: bool = false;
        let kernel_code_segment_selector = Selector::create(
            kernel_code_segment_index as u16,
            is_ldt,
            KERNEL_PRIVILEGE_LEVEL,
        );
        let kernel_data_segment_selector = Selector::create(
            kernel_data_segment_index as u16,
            is_ldt,
            KERNEL_PRIVILEGE_LEVEL,
        );
        let application_code_segment_selector = Selector::create(
            application_code_segment_index as u16,
            is_ldt,
            APPLICATION_PRIVILEGE_LEVEL,
        );
        let application_data_segment_selector = Selector::create(
            application_data_segment_index as u16,
            is_ldt,
            APPLICATION_PRIVILEGE_LEVEL,
        );
        x64::set_segment_registers(&kernel_code_segment_selector, &kernel_data_segment_selector); // Don't rewrite segment registers before exiting boot services.
        com2_println!("gdt = {:#x?}", table);
        let cs: Selector = Selector::cs();
        com2_println!("cs = {:#x?}", cs);
        let ds: Selector = Selector::ds();
        com2_println!("ds = {:#x?}", ds);
        let es: Selector = Selector::es();
        com2_println!("es = {:#x?}", es);
        let fs: Selector = Selector::fs();
        com2_println!("fs = {:#x?}", fs);
        let gs: Selector = Selector::gs();
        com2_println!("gs = {:#x?}", gs);
        let ss: Selector = Selector::ss();
        com2_println!("ss = {:#x?}", ss);
        Self {
            table,
            application_code_segment_selector,
            application_data_segment_selector,
            kernel_code_segment_selector,
            kernel_data_segment_selector,
        }
    }

    pub fn application_code_segment_selector(&self) -> &Selector {
        &self.application_code_segment_selector
    }

    pub fn application_data_segment_selector(&self) -> &Selector {
        &self.application_data_segment_selector
    }

    pub fn kernel_code_segment_selector(&self) -> &Selector {
        &self.kernel_code_segment_selector
    }

    pub fn kernel_data_segment_selector(&self) -> &Selector {
        &self.kernel_data_segment_selector
    }

    pub fn set_task_state_segment_descriptor(
        &mut self,
        task_state_segment_descriptor: &long::Descriptor,
    ) -> Selector {
        self.table
            .set_task_state_segment_descriptor(task_state_segment_descriptor)
    }
}

/// Segment Descriptor Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 3.5.1 Segment Descriptor Tables
pub struct Table {
    descriptors: Vec<u64>,
}

impl Table {
    pub fn base(&self) -> u64 {
        self.descriptors.as_slice().as_ptr() as u64
    }

    pub fn continuous_free_descriptor_indices(&self, length: usize) -> Option<Range<usize>> {
        let indices: Range<usize> =
            self.free_descriptor_indices()
                .into_iter()
                .fold(0..0, |indices, index| {
                    if indices.len() == length {
                        indices
                    } else if indices.end == index {
                        indices.start..index + 1
                    } else {
                        index..index + 1
                    }
                });
        (indices.len() == length).then_some(indices)
    }

    pub fn descriptor(&self, selector: &Selector) -> Option<Interface> {
        self.index2descriptor()
            .into_iter()
            .find_map(|(index, interface)| {
                (index == selector.get_index() as usize).then_some(interface)
            })
    }

    pub fn get() -> Self {
        Register::get().into()
    }

    pub fn index2descriptor(&self) -> BTreeMap<usize, Interface> {
        let long_descriptor_indices: BTreeSet<usize> = self.long_descriptor_indices();
        let short_descriptor_indices: BTreeSet<usize> = self.short_descriptor_indices();
        (0..self.descriptors.len())
            .filter_map(move |index| {
                if short_descriptor_indices.contains(&index) {
                    let descriptor: short::Descriptor = self.descriptors[index].into();
                    let descriptor: Option<Interface> = (&descriptor).into();
                    descriptor.map(|descriptor| (index, descriptor))
                } else if long_descriptor_indices.contains(&index) {
                    let lower_descriptor: u64 = self.descriptors[index];
                    let higher_descriptor: u64 = self.descriptors[index + 1];
                    let descriptor: u128 =
                        ((higher_descriptor as u128) << u64::BITS) + (lower_descriptor as u128);
                    let descriptor: long::Descriptor = descriptor.into();
                    let descriptor: Option<Interface> = (&descriptor).into();
                    descriptor.map(|descriptor| (index, descriptor))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn limit(&self) -> u16 {
        let length: usize = self.descriptors.len();
        let size: usize = length * mem::size_of::<short::Descriptor>();
        let limit: usize = size - 1;
        limit as u16
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

    pub fn set_task_state_segment_descriptor(
        &mut self,
        task_state_segment_descriptor: &long::Descriptor,
    ) -> Selector {
        let task_state_segment_descriptor: u128 = (*task_state_segment_descriptor).into();
        let lower_descriptor: u64 = (task_state_segment_descriptor & ((1 << u64::BITS) - 1)) as u64;
        let higher_descriptor: u64 = (task_state_segment_descriptor >> u64::BITS) as u64;
        let free_descriptor_indices: Vec<usize> =
            self.free_descriptor_indices().into_iter().collect();
        let free_descriptor_indices: &[usize] = free_descriptor_indices.as_slice();
        let lower_descriptor_indices: &[usize] =
            &free_descriptor_indices[..free_descriptor_indices.len() - 1];
        let higher_descriptor_indices: &[usize] = &free_descriptor_indices[1..];
        let (lower_descriptor_index, higher_descriptor_index): (&usize, &usize) =
            lower_descriptor_indices
                .iter()
                .zip(higher_descriptor_indices.iter())
                .find(|(lower_descriptor_index, higher_descriptor_index)| {
                    *lower_descriptor_index + 1 == **higher_descriptor_index
                })
                .unwrap();
        self.descriptors[*lower_descriptor_index] = lower_descriptor;
        self.descriptors[*higher_descriptor_index] = higher_descriptor;
        let segment_selector: u16 =
            (lower_descriptor_index * mem::size_of::<short::Descriptor>()) as u16;
        segment_selector.into()
    }

    fn free_descriptor_indices(&self) -> BTreeSet<usize> {
        self.descriptors
            .iter()
            .enumerate()
            .fold(
                (BTreeSet::<usize>::new(), false),
                |(mut free_descriptor_indices, previous_descriptor_is_lower_of_long),
                 (index, descriptor)| {
                    if previous_descriptor_is_lower_of_long {
                        (free_descriptor_indices, false)
                    } else {
                        let descriptor: short::Descriptor = (*descriptor).into();
                        let interface: Option<Interface> = (&descriptor).into();
                        match interface {
                            Some(interface) => {
                                if interface.is_long_descriptor() {
                                    (free_descriptor_indices, true)
                                } else {
                                    (free_descriptor_indices, false)
                                }
                            }
                            None => {
                                free_descriptor_indices.insert(index);
                                (free_descriptor_indices, false)
                            }
                        }
                    }
                },
            )
            .0
    }

    fn long_descriptor_indices(&self) -> BTreeSet<usize> {
        self.descriptors
            .iter()
            .enumerate()
            .fold(
                (BTreeSet::<usize>::new(), false),
                |(mut long_descriptor_indices, previous_descriptor_is_lower_of_long),
                 (index, descriptor)| {
                    if previous_descriptor_is_lower_of_long {
                        (long_descriptor_indices, false)
                    } else {
                        let descriptor: short::Descriptor = (*descriptor).into();
                        let interface: Option<Interface> = (&descriptor).into();
                        match interface {
                            Some(interface) => {
                                if interface.is_long_descriptor() {
                                    long_descriptor_indices.insert(index);
                                    (long_descriptor_indices, true)
                                } else {
                                    (long_descriptor_indices, false)
                                }
                            }
                            None => (long_descriptor_indices, false),
                        }
                    }
                },
            )
            .0
    }

    fn short_descriptor_indices(&self) -> BTreeSet<usize> {
        self.descriptors
            .iter()
            .enumerate()
            .fold(
                (BTreeSet::<usize>::new(), false),
                |(mut short_descriptor_indices, previous_descriptor_is_lower_of_short),
                 (index, descriptor)| {
                    if previous_descriptor_is_lower_of_short {
                        (short_descriptor_indices, false)
                    } else {
                        let descriptor: short::Descriptor = (*descriptor).into();
                        let interface: Option<Interface> = (&descriptor).into();
                        match interface {
                            Some(interface) => {
                                if interface.is_short_descriptor() {
                                    short_descriptor_indices.insert(index);
                                    (short_descriptor_indices, false)
                                } else {
                                    (short_descriptor_indices, true)
                                }
                            }
                            None => (short_descriptor_indices, false),
                        }
                    }
                },
            )
            .0
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries(self.index2descriptor())
            .finish()
    }
}

impl From<Register> for Table {
    fn from(register: Register) -> Self {
        let descriptors: &[u64] =
            unsafe { slice::from_raw_parts(register.base(), register.length()) };
        let descriptors: Vec<u64> = (u16::MIN..=u16::MAX)
            .step_by(mem::size_of::<short::Descriptor>())
            .map(|segment_selector| {
                *descriptors
                    .get(segment_selector as usize / mem::size_of::<short::Descriptor>())
                    .unwrap_or(&0)
            })
            .collect();
        Self { descriptors }
    }
}
