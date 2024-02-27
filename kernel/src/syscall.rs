use crate::x64;

pub fn initialize(cpuid: &Option<x64::Cpuid>) {
    let system_call_enable: bool = x64::msr::ia32::Efer::enable_system_call_enable_bit(cpuid);
    assert!(system_call_enable);
}

