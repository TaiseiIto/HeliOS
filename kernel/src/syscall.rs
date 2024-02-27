use crate::{
    com2_print,
    com2_println,
    x64,
};

pub fn initialize(cpuid: &Option<x64::Cpuid>) {
    let ia32_star = x64::msr::ia32::Star::get(cpuid);
    com2_println!("ia32_star = {:#x?}", ia32_star);
    let system_call_enable: bool = x64::msr::ia32::Efer::enable_system_call_enable_bit(cpuid);
    assert!(system_call_enable);
}

