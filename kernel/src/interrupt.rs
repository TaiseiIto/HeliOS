pub mod descriptor;

pub use descriptor::Descriptor;

use crate::{
    com2_print,
    com2_println,
};

#[derive(Debug)]
#[repr(C)]
pub struct StackFrame {
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

pub fn register_handlers(idt: &mut descriptor::Table) {
    HANDLERS
        .as_slice()
        .iter()
        .zip(idt.iter_mut())
        .for_each(|(handler, descriptor)| {
            let interrupt_stack_table: u8 = 1;
            let interface = descriptor::Interface::new(*handler, interrupt_stack_table);
            *descriptor = (&interface).into();
        });
}

const HANDLERS: [extern "x86-interrupt" fn(StackFrame); 0x100] = [
    handler_0x00,
    handler_0x01,
    handler_0x02,
    handler_0x03,
    handler_0x04,
    handler_0x05,
    handler_0x06,
    handler_0x07,
    handler_0x08,
    handler_0x09,
    handler_0x0a,
    handler_0x0b,
    handler_0x0c,
    handler_0x0d,
    handler_0x0e,
    handler_0x0f,
    handler_0x10,
    handler_0x11,
    handler_0x12,
    handler_0x13,
    handler_0x14,
    handler_0x15,
    handler_0x16,
    handler_0x17,
    handler_0x18,
    handler_0x19,
    handler_0x1a,
    handler_0x1b,
    handler_0x1c,
    handler_0x1d,
    handler_0x1e,
    handler_0x1f,
    handler_0x20,
    handler_0x21,
    handler_0x22,
    handler_0x23,
    handler_0x24,
    handler_0x25,
    handler_0x26,
    handler_0x27,
    handler_0x28,
    handler_0x29,
    handler_0x2a,
    handler_0x2b,
    handler_0x2c,
    handler_0x2d,
    handler_0x2e,
    handler_0x2f,
    handler_0x30,
    handler_0x31,
    handler_0x32,
    handler_0x33,
    handler_0x34,
    handler_0x35,
    handler_0x36,
    handler_0x37,
    handler_0x38,
    handler_0x39,
    handler_0x3a,
    handler_0x3b,
    handler_0x3c,
    handler_0x3d,
    handler_0x3e,
    handler_0x3f,
    handler_0x40,
    handler_0x41,
    handler_0x42,
    handler_0x43,
    handler_0x44,
    handler_0x45,
    handler_0x46,
    handler_0x47,
    handler_0x48,
    handler_0x49,
    handler_0x4a,
    handler_0x4b,
    handler_0x4c,
    handler_0x4d,
    handler_0x4e,
    handler_0x4f,
    handler_0x50,
    handler_0x51,
    handler_0x52,
    handler_0x53,
    handler_0x54,
    handler_0x55,
    handler_0x56,
    handler_0x57,
    handler_0x58,
    handler_0x59,
    handler_0x5a,
    handler_0x5b,
    handler_0x5c,
    handler_0x5d,
    handler_0x5e,
    handler_0x5f,
    handler_0x60,
    handler_0x61,
    handler_0x62,
    handler_0x63,
    handler_0x64,
    handler_0x65,
    handler_0x66,
    handler_0x67,
    handler_0x68,
    handler_0x69,
    handler_0x6a,
    handler_0x6b,
    handler_0x6c,
    handler_0x6d,
    handler_0x6e,
    handler_0x6f,
    handler_0x70,
    handler_0x71,
    handler_0x72,
    handler_0x73,
    handler_0x74,
    handler_0x75,
    handler_0x76,
    handler_0x77,
    handler_0x78,
    handler_0x79,
    handler_0x7a,
    handler_0x7b,
    handler_0x7c,
    handler_0x7d,
    handler_0x7e,
    handler_0x7f,
    handler_0x80,
    handler_0x81,
    handler_0x82,
    handler_0x83,
    handler_0x84,
    handler_0x85,
    handler_0x86,
    handler_0x87,
    handler_0x88,
    handler_0x89,
    handler_0x8a,
    handler_0x8b,
    handler_0x8c,
    handler_0x8d,
    handler_0x8e,
    handler_0x8f,
    handler_0x90,
    handler_0x91,
    handler_0x92,
    handler_0x93,
    handler_0x94,
    handler_0x95,
    handler_0x96,
    handler_0x97,
    handler_0x98,
    handler_0x99,
    handler_0x9a,
    handler_0x9b,
    handler_0x9c,
    handler_0x9d,
    handler_0x9e,
    handler_0x9f,
    handler_0xa0,
    handler_0xa1,
    handler_0xa2,
    handler_0xa3,
    handler_0xa4,
    handler_0xa5,
    handler_0xa6,
    handler_0xa7,
    handler_0xa8,
    handler_0xa9,
    handler_0xaa,
    handler_0xab,
    handler_0xac,
    handler_0xad,
    handler_0xae,
    handler_0xaf,
    handler_0xb0,
    handler_0xb1,
    handler_0xb2,
    handler_0xb3,
    handler_0xb4,
    handler_0xb5,
    handler_0xb6,
    handler_0xb7,
    handler_0xb8,
    handler_0xb9,
    handler_0xba,
    handler_0xbb,
    handler_0xbc,
    handler_0xbd,
    handler_0xbe,
    handler_0xbf,
    handler_0xc0,
    handler_0xc1,
    handler_0xc2,
    handler_0xc3,
    handler_0xc4,
    handler_0xc5,
    handler_0xc6,
    handler_0xc7,
    handler_0xc8,
    handler_0xc9,
    handler_0xca,
    handler_0xcb,
    handler_0xcc,
    handler_0xcd,
    handler_0xce,
    handler_0xcf,
    handler_0xd0,
    handler_0xd1,
    handler_0xd2,
    handler_0xd3,
    handler_0xd4,
    handler_0xd5,
    handler_0xd6,
    handler_0xd7,
    handler_0xd8,
    handler_0xd9,
    handler_0xda,
    handler_0xdb,
    handler_0xdc,
    handler_0xdd,
    handler_0xde,
    handler_0xdf,
    handler_0xe0,
    handler_0xe1,
    handler_0xe2,
    handler_0xe3,
    handler_0xe4,
    handler_0xe5,
    handler_0xe6,
    handler_0xe7,
    handler_0xe8,
    handler_0xe9,
    handler_0xea,
    handler_0xeb,
    handler_0xec,
    handler_0xed,
    handler_0xee,
    handler_0xef,
    handler_0xf0,
    handler_0xf1,
    handler_0xf2,
    handler_0xf3,
    handler_0xf4,
    handler_0xf5,
    handler_0xf6,
    handler_0xf7,
    handler_0xf8,
    handler_0xf9,
    handler_0xfa,
    handler_0xfb,
    handler_0xfc,
    handler_0xfd,
    handler_0xfe,
    handler_0xff,
];

extern "x86-interrupt" fn handler_0x00(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x01(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x02(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x03(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x04(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x05(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x06(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x07(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x08(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x09(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x0a(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x0b(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x0c(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x0d(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x0e(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x0f(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x10(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x11(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x12(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x13(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x14(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x15(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x16(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x17(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x18(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x19(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x1a(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x1b(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x1c(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x1d(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x1e(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x1f(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x20(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x21(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x22(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x23(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x24(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x25(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x26(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x27(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x28(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x29(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2a(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2b(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2c(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2d(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2e(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x2f(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x30(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x31(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x32(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x33(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x34(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x35(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x36(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x37(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x38(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x39(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3a(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3b(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3c(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3d(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3e(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x3f(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x40(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x41(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x42(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x43(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x44(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x45(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x46(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x47(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x48(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x49(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4a(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4b(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4c(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4d(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4e(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x4f(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x50(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x51(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x52(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x53(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x54(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x55(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x56(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x57(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x58(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x59(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5a(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5b(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5c(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5d(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5e(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x5f(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x60(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x61(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x62(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x63(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x64(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x65(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x66(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x67(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x68(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x69(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6a(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6b(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6c(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6d(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6e(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x6f(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x70(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x71(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x72(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x73(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x74(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x75(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x76(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x77(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x78(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x79(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7a(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7b(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7c(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7d(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7e(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x7f(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x80(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x81(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x82(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x83(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x84(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x85(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x86(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x87(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x88(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x89(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8a(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8b(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8c(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8d(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8e(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x8f(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x90(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x91(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x92(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x93(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x94(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x95(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x96(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x97(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x98(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x99(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9a(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9b(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9c(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9d(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9e(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0x9f(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa0(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa1(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa2(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa3(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa4(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa5(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa6(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa7(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa8(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xa9(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xaa(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xab(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xac(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xad(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xae(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xaf(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb0(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb1(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb2(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb3(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb4(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb5(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb6(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb7(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb8(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xb9(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xba(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xbb(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xbc(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xbd(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xbe(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xbf(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc0(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc1(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc2(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc3(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc4(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc5(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc6(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc7(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc8(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xc9(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xca(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xcb(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xcc(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xcd(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xce(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xcf(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd0(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd1(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd2(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd3(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd4(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd5(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd6(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd7(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd8(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xd9(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xda(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xdb(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xdc(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xdd(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xde(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xdf(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe0(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe1(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe2(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe3(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe4(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe5(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe6(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe7(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe8(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xe9(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xea(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xeb(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xec(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xed(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xee(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xef(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf0(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf1(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf2(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf3(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf4(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf5(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf6(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf7(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf8(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xf9(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xfa(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xfb(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xfc(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xfd(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xfe(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

extern "x86-interrupt" fn handler_0xff(stack_frame: StackFrame) {
    panic!("stack_frame = {:#x?}", stack_frame);
}

