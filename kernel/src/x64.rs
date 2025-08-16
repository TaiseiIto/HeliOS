//! # Wrapper functions of x64 instructions
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)

pub mod cmos;
pub mod control;
pub mod cpuid;
pub mod descriptor;
pub mod msr;
pub mod port;
pub mod rflags;
pub mod task;

pub use {
    cpuid::Cpuid,
    rflags::Rflags,
};

use {
    core::arch::asm,
    crate::memory,
};

/// # Clear Interrupt Flag
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2A 3-166
#[inline(never)]
pub fn cli() {
    unsafe {
        asm!("cli");
    }
    assert!(!Rflags::get().interrupt_is_enabled());
}

/// # Halt
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2A 3-489
#[inline(never)]
pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

/// # Pause
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2B 4-230
#[inline(never)]
pub fn pause() {
    unsafe {
        asm!("pause");
    }
}

/// # Read Timer Stamp Counter
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2B 4-550
#[allow(dead_code)]
#[inline(never)]
pub fn rdtsc() -> u64 {
    let eax: u32;
    let edx: u32;
    unsafe {
        asm!(
            "rdtsc",
            out("eax") eax,
            out("edx") edx,
        );
    }
    ((edx as u64) << u32::BITS) + (eax as u64)
}

/// # Set Interrupt Flag
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2B 4-662
#[inline(never)]
pub fn sti() {
    unsafe {
        asm!("sti");
    }
    assert!(Rflags::get().interrupt_is_enabled());
}

pub fn set_segment_registers(code_segment_selector: &memory::segment::Selector, data_segment_selector: &memory::segment::Selector) {
    let code_segment_selector: u16 = (*code_segment_selector).into();
    let data_segment_selector: u16 = (*data_segment_selector).into();
    unsafe {
        asm!(
            "mov ds, {data_segment_selector:x}",
            "mov es, {data_segment_selector:x}",
            "mov fs, {data_segment_selector:x}",
            "mov gs, {data_segment_selector:x}",
            "mov ss, {data_segment_selector:x}",
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

