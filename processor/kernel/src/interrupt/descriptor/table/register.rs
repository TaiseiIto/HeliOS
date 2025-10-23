use {
    super::super::{super::Descriptor, Table},
    core::{arch::asm, mem::size_of},
};

#[derive(Debug, Default)]
#[repr(packed)]
pub struct Register {
    limit: u16,
    base: u64,
}

impl Register {
    pub fn base(&self) -> *const Descriptor {
        self.base as *const Descriptor
    }

    #[allow(dead_code)]
    #[inline(never)]
    pub fn get() -> Self {
        let mut register = Register::default();
        unsafe {
            asm!(
                "sidt [{}]",
                in(reg) &mut register,
            );
        }
        register
    }

    pub fn length(&self) -> usize {
        (self.limit as usize + 1) / size_of::<Descriptor>()
    }

    #[inline(never)]
    pub fn set(&self) {
        unsafe {
            asm!(
                "lidt [{}]",
                in(reg) self,
            );
        }
    }
}

impl From<&Table> for Register {
    fn from(table: &Table) -> Self {
        let limit: u16 = table.limit();
        let base: u64 = table.base();
        Self { limit, base }
    }
}
