use {
    core::{
        arch::asm,
        mem,
    },
    super::super::{
        super::short,
        Table,
    },
};

#[derive(Debug, Default)]
#[repr(packed)]
pub struct Register {
    limit: u16,
    base: u64,
}

impl Register {
    pub fn base(&self) -> *const u64 {
        self.base as *const u64
    }

    #[allow(dead_code)]
    #[inline(never)]
    pub fn get() -> Self {
        let mut register = Register::default();
        unsafe {
            asm!(
                "sgdt [{}]",
                in(reg) &mut register,
            );
        }
        register
    }

    pub fn length(&self) -> usize {
        (self.limit as usize + 1) / mem::size_of::<short::Descriptor>()
    }

    #[allow(dead_code)]
    #[inline(never)]
    pub fn set(&self) {
        unsafe {
            asm!(
                "lgdt [{}]",
                in(reg) self,
            );
        }
    }
}

impl From<&Table> for Register {
    fn from(table: &Table) -> Self {
        let limit: u16 = table.limit();
        let base: u64 = table.base();
        Self {
            limit,
            base,
        }
    }
}

