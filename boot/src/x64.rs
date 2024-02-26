//! # Wrapper functions of x64 instructions
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)

use core::arch::asm;

pub mod control;
pub mod cpuid;
pub mod descriptor;
pub mod msr;
pub mod port;
pub mod rflags;

pub use {
    cpuid::Cpuid,
    rflags::Rflags,
};

use crate::memory;

/// # Halt
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2A 3-489
#[inline(never)]
pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

pub fn set_segment_registers(code_segment_selector: &memory::segment::Selector, data_segment_selector: &memory::segment::Selector) {
    let code_segment_selector: u16 = code_segment_selector.clone().into();
    let data_segment_selector: u16 = data_segment_selector.clone().into();
    unsafe {
        asm!(
            "mov ds, {data_segment_selector:x}",
            "mov es, {data_segment_selector:x}",
            "mov fs, {data_segment_selector:x}",
            "mov gs, {data_segment_selector:x}",
            "movzx {extended_code_segment_selector}, {code_segment_selector:x}",
            "lea {destination}, [rip + 0f]",
            "push {extended_code_segment_selector}",
            "push {destination}",
            "retfq",
            "0:",
            code_segment_selector = in(reg) code_segment_selector,
            data_segment_selector = in(reg) data_segment_selector,
            extended_code_segment_selector = out(reg) _,
            destination = out(reg) _,
        );
    }
}
