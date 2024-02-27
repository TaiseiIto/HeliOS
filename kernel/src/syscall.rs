use crate::{
    com2_print,
    com2_println,
    memory,
    x64,
};

pub fn initialize(
    cpuid: &Option<x64::Cpuid>,
    kernel_code_segment_selector: &memory::segment::Selector,
    kernel_data_segment_selector: &memory::segment::Selector,
    application_code_segment_selector: &memory::segment::Selector,
    application_data_segment_selector: &memory::segment::Selector,
) {
    x64::msr::ia32::Star::set_segment_selectors(cpuid, kernel_code_segment_selector, kernel_data_segment_selector, application_code_segment_selector, application_data_segment_selector);
    let ia32_star = x64::msr::ia32::Star::get(cpuid);
    com2_println!("ia32_star = {:#x?}", ia32_star);
    let ia32_lstar = x64::msr::ia32::Lstar::get(cpuid);
    com2_println!("ia32_lstar = {:#x?}", ia32_lstar);
    let ia32_fmask = x64::msr::ia32::Fmask::get(cpuid);
    com2_println!("ia32_fmask = {:#x?}", ia32_fmask);
    let system_call_enable: bool = x64::msr::ia32::Efer::enable_system_call_enable_bit(cpuid);
    assert!(system_call_enable);
}

