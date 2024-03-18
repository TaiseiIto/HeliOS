use crate::{
    com2_print,
    com2_println,
    memory,
    x64,
};

#[naked_function::naked]
pub unsafe extern "C" fn handler() {
    asm!(
        "enter 0, 0",
        "push rcx",     // Caller rip
        "push r11",     // Caller rflags
        "push rax",     // System call number
        "mov rcx, r10", // 4th argument
        "call syscall_handler",
        "pop rax",
        "pop r11",
        "pop rcx",
        "leave",
        "sysretq",
    );
}

#[no_mangle]
pub extern "C" fn syscall_handler(rdi: usize, rsi: usize, rdx: usize, r10: usize, r8: usize, r9: usize, rax: usize) {
    com2_println!("Syscall");
    com2_println!("rax = {:#x?}", rax);
    com2_println!("rdi = {:#x?}", rdi);
    com2_println!("rsi = {:#x?}", rsi);
    com2_println!("rdx = {:#x?}", rdx);
    com2_println!("r10 = {:#x?}", r10);
    com2_println!("r8 = {:#x?}", r8);
    com2_println!("r9 = {:#x?}", r9);
    panic!("Syscall handler is not implemented!");
}

pub fn initialize(
    cpuid: &x64::Cpuid,
    kernel_code_segment_selector: &memory::segment::Selector,
    kernel_data_segment_selector: &memory::segment::Selector,
    application_code_segment_selector: &memory::segment::Selector,
    application_data_segment_selector: &memory::segment::Selector,
) {
    x64::msr::ia32::Star::set_segment_selectors(cpuid, kernel_code_segment_selector, kernel_data_segment_selector, application_code_segment_selector, application_data_segment_selector);
    let ia32_star = x64::msr::ia32::Star::get(cpuid);
    com2_println!("ia32_star = {:#x?}", ia32_star);
    x64::msr::ia32::Lstar::set_handler(cpuid, handler);
    let ia32_lstar = x64::msr::ia32::Lstar::get(cpuid);
    com2_println!("ia32_lstar = {:#x?}", ia32_lstar);
    x64::msr::ia32::Fmask::set_all_flags(cpuid);
    let ia32_fmask = x64::msr::ia32::Fmask::get(cpuid);
    com2_println!("ia32_fmask = {:#x?}", ia32_fmask);
    let system_call_enable: bool = x64::msr::ia32::Efer::enable_system_call_enable_bit(cpuid);
    assert!(system_call_enable);
}

