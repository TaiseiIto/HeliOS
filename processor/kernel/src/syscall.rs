use crate::{bsp_println, memory, x64};

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
pub extern "C" fn syscall_handler(
    rdi: usize,
    rsi: usize,
    rdx: usize,
    r10: usize,
    r8: usize,
    r9: usize,
    rax: usize,
) {
    bsp_println!("Syscall");
    bsp_println!("rax = {:#x?}", rax);
    bsp_println!("rdi = {:#x?}", rdi);
    bsp_println!("rsi = {:#x?}", rsi);
    bsp_println!("rdx = {:#x?}", rdx);
    bsp_println!("r10 = {:#x?}", r10);
    bsp_println!("r8 = {:#x?}", r8);
    bsp_println!("r9 = {:#x?}", r9);
    panic!("Syscall handler is not implemented!");
}

pub fn initialize(
    cpuid: &x64::Cpuid,
    kernel_code_segment_selector: &memory::segment::Selector,
    kernel_data_segment_selector: &memory::segment::Selector,
    application_code_segment_selector: &memory::segment::Selector,
    application_data_segment_selector: &memory::segment::Selector,
) {
    x64::msr::ia32::Star::set_segment_selectors(
        cpuid,
        kernel_code_segment_selector,
        kernel_data_segment_selector,
        application_code_segment_selector,
        application_data_segment_selector,
    );
    let ia32_star = x64::msr::ia32::Star::get(cpuid);
    bsp_println!("ia32_star = {:#x?}", ia32_star);
    x64::msr::ia32::Lstar::set_handler(cpuid, handler);
    let ia32_lstar = x64::msr::ia32::Lstar::get(cpuid);
    bsp_println!("ia32_lstar = {:#x?}", ia32_lstar);
    x64::msr::ia32::Fmask::set_all_flags(cpuid);
    let ia32_fmask = x64::msr::ia32::Fmask::get(cpuid);
    bsp_println!("ia32_fmask = {:#x?}", ia32_fmask);
    let system_call_enable: bool = x64::msr::ia32::Efer::enable_system_call_enable_bit(cpuid);
    assert!(system_call_enable);
}
