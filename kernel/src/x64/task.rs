//! Task Management
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 8 Task Management

pub mod state;

use {crate::memory, core::arch::asm};

/// Task Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 2.4.4 Task Register (TR)
#[derive(Debug)]
pub struct Register {
    segment_selector: memory::segment::Selector,
}

impl Register {
    #[allow(dead_code)]
    #[inline(never)]
    pub fn get() -> Self {
        let mut segment_selector: u16;
        unsafe {
            asm!(
                "str {0:x}",
                out(reg) segment_selector,
            );
        }
        let segment_selector: memory::segment::Selector = segment_selector.into();
        Self { segment_selector }
    }

    #[inline(never)]
    pub fn set(&self) {
        let segment_selector: u16 = self.segment_selector.into();
        unsafe {
            asm!(
                "ltr {0:x}",
                in(reg) segment_selector,
            );
        }
    }
}

impl From<memory::segment::Selector> for Register {
    fn from(segment_selector: memory::segment::Selector) -> Self {
        Self { segment_selector }
    }
}
