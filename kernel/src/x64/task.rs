//! Task Management
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 8 Task Management

pub mod state;

use {
    core::arch::asm,
    crate::memory,
};

/// Task Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 2.4.4 Task Register (TR)
#[derive(Debug)]
pub struct Register {
    #[allow(dead_code)]
    segment_selector: memory::segment::Selector,
}

impl Register {
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
        Self {
            segment_selector,
        }
    }
}

