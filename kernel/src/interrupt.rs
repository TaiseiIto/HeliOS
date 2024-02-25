pub mod descriptor;

pub use descriptor::Descriptor;

use crate::{
    com2_print,
    com2_println,
    memory,
    x64,
};

pub enum Handler {
    WithErrorCode(extern "x86-interrupt" fn(StackFrameAndErrorCode)),
    WithoutErrorCode(extern "x86-interrupt" fn(StackFrame)),
}

impl From<&Handler> for usize {
    fn from(handler: &Handler) -> Self {
        match handler {
            Handler::WithErrorCode(handler) => *handler as Self,
            Handler::WithoutErrorCode(handler) => *handler as Self,
        }
    }
}

impl From<extern "x86-interrupt" fn(StackFrameAndErrorCode)> for Handler {
    fn from(handler: extern "x86-interrupt" fn(StackFrameAndErrorCode)) -> Self {
        Self::WithErrorCode(handler)
    }
}

impl From<extern "x86-interrupt" fn(StackFrame)> for Handler {
    fn from(handler: extern "x86-interrupt" fn(StackFrame)) -> Self {
        Self::WithoutErrorCode(handler)
    }
}

/// # Interrupt Stack Frame
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.14.4 Figure 6-9. IA-32e Mode Stack Usage After Privilege Level Change
#[derive(Debug)]
#[repr(C)]
pub struct StackFrame {
    rip: u64,
    cs: memory::segment::Selector,
    rflags: x64::Rflags,
    rsp: u64,
    ss: memory::segment::Selector,
}

/// # Interrupt Stack Frame and Error Code
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.14.4 Figure 6-9. IA-32e Mode Stack Usage After Privilege Level Change
#[derive(Debug)]
#[repr(C)]
pub struct StackFrameAndErrorCode {
    error_code: u64,
    stack_frame: StackFrame,
}

pub fn register_handlers(idt: &mut descriptor::Table) {
    let handlers: [Handler; 0x100] = [
        (handler_0x00 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x01 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x02 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x03 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x04 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x05 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x06 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x07 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x08 as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x09 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x0a as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x0b as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x0c as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x0d as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x0e as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x0f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x10 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x11 as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x12 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x13 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x14 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x15 as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x16 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x17 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x18 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x19 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x1a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x1b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x1c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x1d as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x1e as extern "x86-interrupt" fn(StackFrameAndErrorCode)).into(),
        (handler_0x1f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x20 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x21 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x22 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x23 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x24 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x25 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x26 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x27 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x28 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x29 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x2f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x30 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x31 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x32 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x33 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x34 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x35 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x36 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x37 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x38 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x39 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x3f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x40 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x41 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x42 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x43 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x44 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x45 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x46 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x47 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x48 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x49 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x4f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x50 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x51 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x52 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x53 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x54 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x55 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x56 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x57 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x58 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x59 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x5f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x60 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x61 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x62 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x63 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x64 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x65 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x66 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x67 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x68 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x69 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x6f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x70 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x71 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x72 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x73 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x74 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x75 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x76 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x77 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x78 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x79 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x7f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x80 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x81 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x82 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x83 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x84 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x85 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x86 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x87 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x88 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x89 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x8f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x90 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x91 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x92 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x93 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x94 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x95 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x96 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x97 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x98 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x99 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9a as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9b as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9c as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9d as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9e as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0x9f as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xa9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xaa as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xab as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xac as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xad as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xae as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xaf as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xb9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xba as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xbb as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xbc as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xbd as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xbe as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xbf as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xc9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xca as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xcb as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xcc as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xcd as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xce as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xcf as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xd9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xda as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xdb as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xdc as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xdd as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xde as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xdf as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xe9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xea as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xeb as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xec as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xed as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xee as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xef as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf0 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf1 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf2 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf3 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf4 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf5 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf6 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf7 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf8 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xf9 as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xfa as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xfb as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xfc as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xfd as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xfe as extern "x86-interrupt" fn(StackFrame)).into(),
        (handler_0xff as extern "x86-interrupt" fn(StackFrame)).into(),
    ];
    handlers
        .as_slice()
        .iter()
        .zip(idt.iter_mut())
        .for_each(|(handler, descriptor)| {
            let interrupt_stack_table: u8 = 1;
            let interface = descriptor::Interface::new(handler, interrupt_stack_table);
            *descriptor = (&interface).into();
        });
}

/// # Divide Error Exception (\#DE)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x00(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x00;
    com2_println!("Divide Error Exception (#DE)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Debug Exception (\#DB)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x01(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x01;
    com2_println!("Debug Exception (#DB)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # NMI Interrupt
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x02(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x02;
    com2_println!("NMI Interrupt");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Breakpoint Exception (\#BP)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x03(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x03;
    com2_println!("Breakpoint Exception (#BP)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Overflow Exception (\#OF)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x04(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x04;
    com2_println!("Overflow Exception (#OF)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # BOUND Range Exceeded Exception (\#BR)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x05(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x05;
    com2_println!("BOUND Range Exceeded Exception (#BR)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Invalid Opcode Exception (\#UD)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x06(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x06;
    com2_println!("Invalid Opcode Exception (#UD)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Device Not Available Exception (\#NM)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x07(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x07;
    com2_println!("Device Not Available Exception (#NM)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Double Fault Exception (\#DF)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x08(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x08;
    com2_println!("Double Fault Exception (#DF)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame_and_error_code = {:#x?}", stack_frame_and_error_code);
}

/// # Coprocessor Segment Overrun
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x09(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x09;
    com2_println!("Coprocessor Segment Overrun");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Invalid TSS Exception (\#TS)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x0a(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x0a;
    com2_println!("Invalid TSS Exception (#TS)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame_and_error_code = {:#x?}", stack_frame_and_error_code);
}

/// # Segment Not Present (\#NP)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x0b(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x0b;
    com2_println!("Segment Not Present (#NP)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame_and_error_code = {:#x?}", stack_frame_and_error_code);
}

/// # Stack Fault Exception (\#SS)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x0c(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x0c;
    com2_println!("Stack Fault Exception (#SS)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame_and_error_code = {:#x?}", stack_frame_and_error_code);
}

/// # General Protection Exception (\#GP)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x0d(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x0d;
    com2_println!("General Protection Exception (#GP)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame_and_error_code = {:#x?}", stack_frame_and_error_code);
}

/// # Page-Fault Exception (\#PF)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x0e(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x0e;
    com2_println!("Page-Fault Exception (#PF)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame_and_error_code = {:#x?}", stack_frame_and_error_code);
}

/// # Reserved Exception 0
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x0f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x0f;
    com2_println!("Reserved Exception 0");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # x87 Floating-Point Error (\#MF)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x10(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x10;
    com2_println!("x87 Floating-Point Error (#MF)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Alignment Check Exception (\#AC)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x11(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x11;
    com2_println!("Alignment Check Exception (#AC)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame_and_error_code = {:#x?}", stack_frame_and_error_code);
}

/// # Machine Check Exception (\#MC)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x12(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x12;
    com2_println!("Machine Check Exception (#MC)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # SIMD Floating-Point Exception (\#XM)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x13(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x13;
    com2_println!("SIMD Floating-Point Exception (#XM)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Virtualization Exception (\#VE)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x14(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x14;
    com2_println!("Virtualization Exception (#VE)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Control Protection Exception (\#CP)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.15 Exception and Interrupt Reference
extern "x86-interrupt" fn handler_0x15(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x15;
    com2_println!("Control Protection Exception (#CP)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame_and_error_code = {:#x?}", stack_frame_and_error_code);
}

/// # Reserved Exception 1
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x16(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x16;
    com2_println!("Reserved Exception 1");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Reserved Exception 2
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x17(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x17;
    com2_println!("Reserved Exception 2");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Reserved Exception 3
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x18(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x18;
    com2_println!("Reserved Exception 3");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Reserved Exception 4
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x19(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x19;
    com2_println!("Reserved Exception 4");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Reserved Exception 5
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x1a;
    com2_println!("Reserved Exception 5");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Reserved Exception 6
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x1b;
    com2_println!("Reserved Exception 6");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # Hypervisor Injection Exception (\#HV)
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x1c;
    com2_println!("Hypervisor Injection Exception (#HV)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

/// # VMM Communication Exception (\#VC)
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1d(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x1d;
    com2_println!("VMM Communication Exception (#VC)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame_and_error_code = {:#x?}", stack_frame_and_error_code);
}

/// # Security Exception (\#SX)
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1e(stack_frame_and_error_code: StackFrameAndErrorCode) {
    let interrupt_number: u8 = 0x1e;
    com2_println!("Security Exception (#SX)");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame_and_error_code = {:#x?}", stack_frame_and_error_code);
}

/// # Reserved Exception 7
/// ## References
/// * [Exceptions - OSDev Wiki](https://wiki.osdev.org/Exceptions)
extern "x86-interrupt" fn handler_0x1f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x1f;
    com2_println!("Reserved Exception 7");
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x20(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x20;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x21(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x21;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x22(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x22;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x23(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x23;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x24(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x24;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x25(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x25;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x26(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x26;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x27(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x27;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x28(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x28;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x29(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x29;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2a;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2b;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2c;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2d;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2e;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x2f;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x30(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x30;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x31(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x31;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x32(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x32;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x33(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x33;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x34(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x34;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x35(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x35;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x36(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x36;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x37(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x37;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x38(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x38;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x39(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x39;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3a;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3b;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3c;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3d;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3e;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x3f;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x40(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x40;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x41(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x41;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x42(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x42;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x43(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x43;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x44(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x44;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x45(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x45;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x46(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x46;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x47(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x47;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x48(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x48;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x49(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x49;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4a;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4b;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4c;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4d;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4e;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x4f;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x50(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x50;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x51(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x51;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x52(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x52;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x53(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x53;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x54(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x54;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x55(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x55;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x56(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x56;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x57(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x57;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x58(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x58;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x59(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x59;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5a;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5b;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5c;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5d;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5e;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x5f;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x60(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x60;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x61(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x61;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x62(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x62;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x63(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x63;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x64(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x64;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x65(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x65;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x66(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x66;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x67(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x67;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x68(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x68;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x69(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x69;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6a;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6b;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6c;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6d;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6e;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x6f;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x70(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x70;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x71(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x71;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x72(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x72;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x73(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x73;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x74(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x74;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x75(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x75;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x76(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x76;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x77(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x77;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x78(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x78;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x79(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x79;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7a;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7b;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7c;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7d;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7e;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x7f;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x80(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x80;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x81(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x81;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x82(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x82;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x83(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x83;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x84(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x84;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x85(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x85;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x86(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x86;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x87(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x87;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x88(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x88;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x89(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x89;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8a;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8b;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8c;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8d;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8e;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x8f;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x90(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x90;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x91(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x91;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x92(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x92;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x93(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x93;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x94(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x94;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x95(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x95;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x96(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x96;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x97(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x97;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x98(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x98;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x99(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x99;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9a(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9a;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9b(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9b;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9c(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9c;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9d(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9d;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9e(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9e;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9f(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0x9f;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa0;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa1;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa2;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa3;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa4;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa5;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa6;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa7;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa8;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xa9;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xaa(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xaa;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xab(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xab;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xac(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xac;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xad(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xad;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xae(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xae;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xaf(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xaf;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb0;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb1;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb2;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb3;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb4;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb5;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb6;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb7;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb8;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xb9;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xba(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xba;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xbb(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xbb;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xbc(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xbc;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xbd(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xbd;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xbe(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xbe;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xbf(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xbf;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc0;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc1;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc2;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc3;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc4;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc5;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc6;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc7;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc8;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xc9;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xca(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xca;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xcb(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xcb;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xcc(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xcc;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xcd(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xcd;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xce(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xce;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xcf(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xcf;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd0;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd1;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd2;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd3;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd4;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd5;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd6;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd7;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd8;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xd9;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xda(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xda;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xdb(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xdb;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xdc(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xdc;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xdd(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xdd;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xde(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xde;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xdf(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xdf;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe0;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe1;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe2;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe3;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe4;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe5;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe6;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe7;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe8;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xe9;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xea(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xea;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xeb(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xeb;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xec(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xec;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xed(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xed;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xee(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xee;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xef(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xef;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf0(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf0;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf1(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf1;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf2(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf2;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf3(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf3;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf4(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf4;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf5(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf5;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf6(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf6;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf7(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf7;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf8(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf8;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf9(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xf9;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xfa(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xfa;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xfb(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xfb;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xfc(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xfc;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xfd(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xfd;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xfe(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xfe;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xff(stack_frame: StackFrame) {
    let interrupt_number: u8 = 0xff;
    com2_println!("interrupt_number = {:#x?}", interrupt_number);
    com2_println!("stack_frame = {:#x?}", stack_frame);
}

