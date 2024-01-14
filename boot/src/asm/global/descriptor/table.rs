//! # Wrapper functions of x64 instructions
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)

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

