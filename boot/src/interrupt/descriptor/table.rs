pub mod register;

use {
    core::slice,
    super::super::Descriptor,
};

pub use register::Register;

/// # Interrupt Descriptor
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.10 Interrupt Descriptor Table (IDT)
#[derive(Debug)]
pub struct Table<'a>(&'a [Descriptor]);

impl Table<'static> {
    pub fn get() -> Self {
        Register::get().into()
    }
}

impl From<Register> for Table<'static> {
    fn from(register: Register) -> Self {
        Self(unsafe {
            slice::from_raw_parts(register.base(), register.length())
        })
    }
}

