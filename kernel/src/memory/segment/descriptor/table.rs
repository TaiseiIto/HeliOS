pub use register::Register;

use {
    alloc::vec::Vec,
    core::{
        fmt,
        mem,
        slice,
    },
    super::{
        Interface,
        super::Descriptor,
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

