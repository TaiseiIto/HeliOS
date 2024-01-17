pub mod register;

use {
    alloc::vec::Vec,
    core::{
        fmt,
        slice,
    },
    super::{
        Interface,
        super::Descriptor,
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
    pub fn get() -> Self {
        Register::get().into()
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

