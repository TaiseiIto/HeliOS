pub use register::Register;

use {
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
pub struct Table<'a>(&'a [Descriptor]);

impl Table<'static> {
    pub fn get() -> Self {
        Register::get().into()
    }
}

impl fmt::Debug for Table<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries(self.0
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

impl From<Register> for Table<'static> {
    fn from(register: Register) -> Self {
        Self(unsafe {
            slice::from_raw_parts(register.base(), register.length())
        })
    }
}

