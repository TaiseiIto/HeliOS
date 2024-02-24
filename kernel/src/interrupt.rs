pub mod descriptor;

pub use descriptor::Descriptor;

use crate::{
    com2_print,
    com2_println,
};

pub fn register_handlers(idt: &mut descriptor::Table) {
    HANDLERS
        .as_slice()
        .iter()
        .zip(idt.iter_mut())
        .for_each(|(handler, descriptor)| {
            let interrupt_stack_table: u8 = 0;
            let interface = descriptor::Interface::new(*handler, interrupt_stack_table);
            *descriptor = (&interface).into();
        });
}

const HANDLERS: [extern "x86-interrupt" fn(u64); 0x100] = [
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

extern "x86-interrupt" fn handler_0x00(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x00 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x01(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x01 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x02(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x02 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x03(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x03 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x04(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x04 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x05(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x05 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x06(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x06 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x07(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x07 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x08(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x08 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x09(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x09 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x0a(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x0a RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x0b(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x0b RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x0c(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x0c RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x0d(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x0d RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x0e(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x0e RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x0f(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x0f RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x10(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x10 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x11(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x11 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x12(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x12 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x13(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x13 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x14(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x14 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x15(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x15 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x16(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x16 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x17(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x17 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x18(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x18 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x19(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x19 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x1a(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x1a RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x1b(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x1b RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x1c(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x1c RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x1d(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x1d RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x1e(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x1e RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x1f(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x1f RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x20(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x20 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x21(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x21 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x22(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x22 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x23(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x23 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x24(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x24 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x25(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x25 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x26(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x26 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x27(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x27 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x28(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x28 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x29(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x29 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x2a(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x2a RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x2b(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x2b RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x2c(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x2c RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x2d(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x2d RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x2e(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x2e RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x2f(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x2f RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x30(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x30 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x31(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x31 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x32(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x32 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x33(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x33 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x34(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x34 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x35(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x35 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x36(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x36 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x37(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x37 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x38(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x38 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x39(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x39 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x3a(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x3a RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x3b(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x3b RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x3c(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x3c RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x3d(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x3d RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x3e(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x3e RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x3f(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x3f RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x40(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x40 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x41(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x41 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x42(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x42 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x43(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x43 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x44(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x44 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x45(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x45 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x46(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x46 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x47(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x47 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x48(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x48 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x49(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x49 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x4a(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x4a RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x4b(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x4b RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x4c(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x4c RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x4d(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x4d RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x4e(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x4e RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x4f(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x4f RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x50(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x50 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x51(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x51 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x52(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x52 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x53(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x53 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x54(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x54 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x55(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x55 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x56(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x56 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x57(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x57 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x58(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x58 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x59(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x59 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x5a(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x5a RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x5b(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x5b RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x5c(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x5c RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x5d(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x5d RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x5e(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x5e RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x5f(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x5f RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x60(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x60 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x61(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x61 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x62(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x62 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x63(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x63 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x64(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x64 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x65(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x65 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x66(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x66 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x67(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x67 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x68(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x68 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x69(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x69 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x6a(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x6a RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x6b(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x6b RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x6c(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x6c RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x6d(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x6d RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x6e(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x6e RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x6f(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x6f RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x70(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x70 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x71(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x71 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x72(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x72 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x73(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x73 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x74(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x74 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x75(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x75 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x76(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x76 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x77(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x77 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x78(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x78 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x79(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x79 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x7a(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x7a RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x7b(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x7b RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x7c(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x7c RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x7d(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x7d RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x7e(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x7e RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x7f(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x7f RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x80(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x80 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x81(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x81 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x82(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x82 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x83(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x83 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x84(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x84 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x85(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x85 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x86(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x86 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x87(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x87 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x88(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x88 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x89(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x89 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x8a(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x8a RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x8b(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x8b RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x8c(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x8c RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x8d(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x8d RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x8e(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x8e RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x8f(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x8f RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x90(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x90 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x91(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x91 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x92(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x92 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x93(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x93 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x94(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x94 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x95(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x95 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x96(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x96 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x97(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x97 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x98(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x98 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x99(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x99 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x9a(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x9a RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x9b(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x9b RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x9c(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x9c RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x9d(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x9d RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x9e(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x9e RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0x9f(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0x9f RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xa0(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xa0 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xa1(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xa1 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xa2(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xa2 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xa3(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xa3 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xa4(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xa4 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xa5(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xa5 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xa6(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xa6 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xa7(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xa7 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xa8(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xa8 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xa9(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xa9 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xaa(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xaa RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xab(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xab RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xac(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xac RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xad(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xad RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xae(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xae RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xaf(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xaf RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xb0(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xb0 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xb1(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xb1 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xb2(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xb2 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xb3(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xb3 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xb4(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xb4 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xb5(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xb5 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xb6(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xb6 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xb7(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xb7 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xb8(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xb8 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xb9(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xb9 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xba(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xba RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xbb(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xbb RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xbc(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xbc RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xbd(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xbd RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xbe(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xbe RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xbf(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xbf RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xc0(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xc0 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xc1(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xc1 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xc2(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xc2 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xc3(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xc3 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xc4(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xc4 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xc5(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xc5 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xc6(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xc6 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xc7(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xc7 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xc8(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xc8 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xc9(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xc9 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xca(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xca RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xcb(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xcb RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xcc(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xcc RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xcd(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xcd RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xce(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xce RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xcf(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xcf RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xd0(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xd0 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xd1(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xd1 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xd2(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xd2 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xd3(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xd3 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xd4(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xd4 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xd5(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xd5 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xd6(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xd6 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xd7(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xd7 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xd8(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xd8 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xd9(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xd9 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xda(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xda RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xdb(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xdb RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xdc(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xdc RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xdd(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xdd RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xde(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xde RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xdf(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xdf RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xe0(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xe0 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xe1(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xe1 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xe2(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xe2 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xe3(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xe3 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xe4(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xe4 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xe5(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xe5 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xe6(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xe6 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xe7(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xe7 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xe8(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xe8 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xe9(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xe9 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xea(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xea RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xeb(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xeb RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xec(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xec RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xed(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xed RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xee(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xee RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xef(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xef RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xf0(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xf0 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xf1(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xf1 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xf2(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xf2 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xf3(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xf3 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xf4(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xf4 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xf5(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xf5 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xf6(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xf6 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xf7(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xf7 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xf8(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xf8 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xf9(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xf9 RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xfa(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xfa RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xfb(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xfb RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xfc(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xfc RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xfd(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xfd RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xfe(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xfe RSP = {:#x?}", stack);
}

extern "x86-interrupt" fn handler_0xff(stack: u64) {
    let stack: &u64 = &stack;
    let stack: *const u64 = stack as *const u64;
    panic!("Interrupt 0xff RSP = {:#x?}", stack);
}

