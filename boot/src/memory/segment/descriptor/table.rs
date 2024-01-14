pub use register::Register;

use {
    core::slice,
    super::super::Descriptor,
};

mod register;

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

