use core::arch::asm;

#[derive(Debug, Default)]
#[repr(packed)]
pub struct Register {
    limit: u16,
    base: u64,
}

impl Register {
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
}

