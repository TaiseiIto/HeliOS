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
            let interrupt_stack_table: u8 = 1;
            let interface = descriptor::Interface::new(*handler, interrupt_stack_table);
            *descriptor = (&interface).into();
        });
}

const HANDLERS: [extern "x86-interrupt" fn(); 0x100] = [
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

extern "x86-interrupt" fn handler_0x00() {
    panic!("Interrupt 0x00 !!!");
}

extern "x86-interrupt" fn handler_0x01() {
    panic!("Interrupt 0x01 !!!");
}

extern "x86-interrupt" fn handler_0x02() {
    panic!("Interrupt 0x02 !!!");
}

extern "x86-interrupt" fn handler_0x03() {
    panic!("Interrupt 0x03 !!!");
}

extern "x86-interrupt" fn handler_0x04() {
    panic!("Interrupt 0x04 !!!");
}

extern "x86-interrupt" fn handler_0x05() {
    panic!("Interrupt 0x05 !!!");
}

extern "x86-interrupt" fn handler_0x06() {
    panic!("Interrupt 0x06 !!!");
}

extern "x86-interrupt" fn handler_0x07() {
    panic!("Interrupt 0x07 !!!");
}

extern "x86-interrupt" fn handler_0x08() {
    panic!("Interrupt 0x08 !!!");
}

extern "x86-interrupt" fn handler_0x09() {
    panic!("Interrupt 0x09 !!!");
}

extern "x86-interrupt" fn handler_0x0a() {
    panic!("Interrupt 0x0a !!!");
}

extern "x86-interrupt" fn handler_0x0b() {
    panic!("Interrupt 0x0b !!!");
}

extern "x86-interrupt" fn handler_0x0c() {
    panic!("Interrupt 0x0c !!!");
}

extern "x86-interrupt" fn handler_0x0d() {
    panic!("Interrupt 0x0d !!!");
}

extern "x86-interrupt" fn handler_0x0e() {
    panic!("Interrupt 0x0e !!!");
}

extern "x86-interrupt" fn handler_0x0f() {
    panic!("Interrupt 0x0f !!!");
}

extern "x86-interrupt" fn handler_0x10() {
    panic!("Interrupt 0x10 !!!");
}

extern "x86-interrupt" fn handler_0x11() {
    panic!("Interrupt 0x11 !!!");
}

extern "x86-interrupt" fn handler_0x12() {
    panic!("Interrupt 0x12 !!!");
}

extern "x86-interrupt" fn handler_0x13() {
    panic!("Interrupt 0x13 !!!");
}

extern "x86-interrupt" fn handler_0x14() {
    panic!("Interrupt 0x14 !!!");
}

extern "x86-interrupt" fn handler_0x15() {
    panic!("Interrupt 0x15 !!!");
}

extern "x86-interrupt" fn handler_0x16() {
    panic!("Interrupt 0x16 !!!");
}

extern "x86-interrupt" fn handler_0x17() {
    panic!("Interrupt 0x17 !!!");
}

extern "x86-interrupt" fn handler_0x18() {
    panic!("Interrupt 0x18 !!!");
}

extern "x86-interrupt" fn handler_0x19() {
    panic!("Interrupt 0x19 !!!");
}

extern "x86-interrupt" fn handler_0x1a() {
    panic!("Interrupt 0x1a !!!");
}

extern "x86-interrupt" fn handler_0x1b() {
    panic!("Interrupt 0x1b !!!");
}

extern "x86-interrupt" fn handler_0x1c() {
    panic!("Interrupt 0x1c !!!");
}

extern "x86-interrupt" fn handler_0x1d() {
    panic!("Interrupt 0x1d !!!");
}

extern "x86-interrupt" fn handler_0x1e() {
    panic!("Interrupt 0x1e !!!");
}

extern "x86-interrupt" fn handler_0x1f() {
    panic!("Interrupt 0x1f !!!");
}

extern "x86-interrupt" fn handler_0x20() {
    panic!("Interrupt 0x20 !!!");
}

extern "x86-interrupt" fn handler_0x21() {
    panic!("Interrupt 0x21 !!!");
}

extern "x86-interrupt" fn handler_0x22() {
    panic!("Interrupt 0x22 !!!");
}

extern "x86-interrupt" fn handler_0x23() {
    panic!("Interrupt 0x23 !!!");
}

extern "x86-interrupt" fn handler_0x24() {
    panic!("Interrupt 0x24 !!!");
}

extern "x86-interrupt" fn handler_0x25() {
    panic!("Interrupt 0x25 !!!");
}

extern "x86-interrupt" fn handler_0x26() {
    panic!("Interrupt 0x26 !!!");
}

extern "x86-interrupt" fn handler_0x27() {
    panic!("Interrupt 0x27 !!!");
}

extern "x86-interrupt" fn handler_0x28() {
    panic!("Interrupt 0x28 !!!");
}

extern "x86-interrupt" fn handler_0x29() {
    panic!("Interrupt 0x29 !!!");
}

extern "x86-interrupt" fn handler_0x2a() {
    panic!("Interrupt 0x2a !!!");
}

extern "x86-interrupt" fn handler_0x2b() {
    panic!("Interrupt 0x2b !!!");
}

extern "x86-interrupt" fn handler_0x2c() {
    panic!("Interrupt 0x2c !!!");
}

extern "x86-interrupt" fn handler_0x2d() {
    panic!("Interrupt 0x2d !!!");
}

extern "x86-interrupt" fn handler_0x2e() {
    panic!("Interrupt 0x2e !!!");
}

extern "x86-interrupt" fn handler_0x2f() {
    panic!("Interrupt 0x2f !!!");
}

extern "x86-interrupt" fn handler_0x30() {
    panic!("Interrupt 0x30 !!!");
}

extern "x86-interrupt" fn handler_0x31() {
    panic!("Interrupt 0x31 !!!");
}

extern "x86-interrupt" fn handler_0x32() {
    panic!("Interrupt 0x32 !!!");
}

extern "x86-interrupt" fn handler_0x33() {
    panic!("Interrupt 0x33 !!!");
}

extern "x86-interrupt" fn handler_0x34() {
    panic!("Interrupt 0x34 !!!");
}

extern "x86-interrupt" fn handler_0x35() {
    panic!("Interrupt 0x35 !!!");
}

extern "x86-interrupt" fn handler_0x36() {
    panic!("Interrupt 0x36 !!!");
}

extern "x86-interrupt" fn handler_0x37() {
    panic!("Interrupt 0x37 !!!");
}

extern "x86-interrupt" fn handler_0x38() {
    panic!("Interrupt 0x38 !!!");
}

extern "x86-interrupt" fn handler_0x39() {
    panic!("Interrupt 0x39 !!!");
}

extern "x86-interrupt" fn handler_0x3a() {
    panic!("Interrupt 0x3a !!!");
}

extern "x86-interrupt" fn handler_0x3b() {
    panic!("Interrupt 0x3b !!!");
}

extern "x86-interrupt" fn handler_0x3c() {
    panic!("Interrupt 0x3c !!!");
}

extern "x86-interrupt" fn handler_0x3d() {
    panic!("Interrupt 0x3d !!!");
}

extern "x86-interrupt" fn handler_0x3e() {
    panic!("Interrupt 0x3e !!!");
}

extern "x86-interrupt" fn handler_0x3f() {
    panic!("Interrupt 0x3f !!!");
}

extern "x86-interrupt" fn handler_0x40() {
    panic!("Interrupt 0x40 !!!");
}

extern "x86-interrupt" fn handler_0x41() {
    panic!("Interrupt 0x41 !!!");
}

extern "x86-interrupt" fn handler_0x42() {
    panic!("Interrupt 0x42 !!!");
}

extern "x86-interrupt" fn handler_0x43() {
    panic!("Interrupt 0x43 !!!");
}

extern "x86-interrupt" fn handler_0x44() {
    panic!("Interrupt 0x44 !!!");
}

extern "x86-interrupt" fn handler_0x45() {
    panic!("Interrupt 0x45 !!!");
}

extern "x86-interrupt" fn handler_0x46() {
    panic!("Interrupt 0x46 !!!");
}

extern "x86-interrupt" fn handler_0x47() {
    panic!("Interrupt 0x47 !!!");
}

extern "x86-interrupt" fn handler_0x48() {
    panic!("Interrupt 0x48 !!!");
}

extern "x86-interrupt" fn handler_0x49() {
    panic!("Interrupt 0x49 !!!");
}

extern "x86-interrupt" fn handler_0x4a() {
    panic!("Interrupt 0x4a !!!");
}

extern "x86-interrupt" fn handler_0x4b() {
    panic!("Interrupt 0x4b !!!");
}

extern "x86-interrupt" fn handler_0x4c() {
    panic!("Interrupt 0x4c !!!");
}

extern "x86-interrupt" fn handler_0x4d() {
    panic!("Interrupt 0x4d !!!");
}

extern "x86-interrupt" fn handler_0x4e() {
    panic!("Interrupt 0x4e !!!");
}

extern "x86-interrupt" fn handler_0x4f() {
    panic!("Interrupt 0x4f !!!");
}

extern "x86-interrupt" fn handler_0x50() {
    panic!("Interrupt 0x50 !!!");
}

extern "x86-interrupt" fn handler_0x51() {
    panic!("Interrupt 0x51 !!!");
}

extern "x86-interrupt" fn handler_0x52() {
    panic!("Interrupt 0x52 !!!");
}

extern "x86-interrupt" fn handler_0x53() {
    panic!("Interrupt 0x53 !!!");
}

extern "x86-interrupt" fn handler_0x54() {
    panic!("Interrupt 0x54 !!!");
}

extern "x86-interrupt" fn handler_0x55() {
    panic!("Interrupt 0x55 !!!");
}

extern "x86-interrupt" fn handler_0x56() {
    panic!("Interrupt 0x56 !!!");
}

extern "x86-interrupt" fn handler_0x57() {
    panic!("Interrupt 0x57 !!!");
}

extern "x86-interrupt" fn handler_0x58() {
    panic!("Interrupt 0x58 !!!");
}

extern "x86-interrupt" fn handler_0x59() {
    panic!("Interrupt 0x59 !!!");
}

extern "x86-interrupt" fn handler_0x5a() {
    panic!("Interrupt 0x5a !!!");
}

extern "x86-interrupt" fn handler_0x5b() {
    panic!("Interrupt 0x5b !!!");
}

extern "x86-interrupt" fn handler_0x5c() {
    panic!("Interrupt 0x5c !!!");
}

extern "x86-interrupt" fn handler_0x5d() {
    panic!("Interrupt 0x5d !!!");
}

extern "x86-interrupt" fn handler_0x5e() {
    panic!("Interrupt 0x5e !!!");
}

extern "x86-interrupt" fn handler_0x5f() {
    panic!("Interrupt 0x5f !!!");
}

extern "x86-interrupt" fn handler_0x60() {
    panic!("Interrupt 0x60 !!!");
}

extern "x86-interrupt" fn handler_0x61() {
    panic!("Interrupt 0x61 !!!");
}

extern "x86-interrupt" fn handler_0x62() {
    panic!("Interrupt 0x62 !!!");
}

extern "x86-interrupt" fn handler_0x63() {
    panic!("Interrupt 0x63 !!!");
}

extern "x86-interrupt" fn handler_0x64() {
    panic!("Interrupt 0x64 !!!");
}

extern "x86-interrupt" fn handler_0x65() {
    panic!("Interrupt 0x65 !!!");
}

extern "x86-interrupt" fn handler_0x66() {
    panic!("Interrupt 0x66 !!!");
}

extern "x86-interrupt" fn handler_0x67() {
    panic!("Interrupt 0x67 !!!");
}

extern "x86-interrupt" fn handler_0x68() {
    panic!("Interrupt 0x68 !!!");
}

extern "x86-interrupt" fn handler_0x69() {
    panic!("Interrupt 0x69 !!!");
}

extern "x86-interrupt" fn handler_0x6a() {
    panic!("Interrupt 0x6a !!!");
}

extern "x86-interrupt" fn handler_0x6b() {
    panic!("Interrupt 0x6b !!!");
}

extern "x86-interrupt" fn handler_0x6c() {
    panic!("Interrupt 0x6c !!!");
}

extern "x86-interrupt" fn handler_0x6d() {
    panic!("Interrupt 0x6d !!!");
}

extern "x86-interrupt" fn handler_0x6e() {
    panic!("Interrupt 0x6e !!!");
}

extern "x86-interrupt" fn handler_0x6f() {
    panic!("Interrupt 0x6f !!!");
}

extern "x86-interrupt" fn handler_0x70() {
    panic!("Interrupt 0x70 !!!");
}

extern "x86-interrupt" fn handler_0x71() {
    panic!("Interrupt 0x71 !!!");
}

extern "x86-interrupt" fn handler_0x72() {
    panic!("Interrupt 0x72 !!!");
}

extern "x86-interrupt" fn handler_0x73() {
    panic!("Interrupt 0x73 !!!");
}

extern "x86-interrupt" fn handler_0x74() {
    panic!("Interrupt 0x74 !!!");
}

extern "x86-interrupt" fn handler_0x75() {
    panic!("Interrupt 0x75 !!!");
}

extern "x86-interrupt" fn handler_0x76() {
    panic!("Interrupt 0x76 !!!");
}

extern "x86-interrupt" fn handler_0x77() {
    panic!("Interrupt 0x77 !!!");
}

extern "x86-interrupt" fn handler_0x78() {
    panic!("Interrupt 0x78 !!!");
}

extern "x86-interrupt" fn handler_0x79() {
    panic!("Interrupt 0x79 !!!");
}

extern "x86-interrupt" fn handler_0x7a() {
    panic!("Interrupt 0x7a !!!");
}

extern "x86-interrupt" fn handler_0x7b() {
    panic!("Interrupt 0x7b !!!");
}

extern "x86-interrupt" fn handler_0x7c() {
    panic!("Interrupt 0x7c !!!");
}

extern "x86-interrupt" fn handler_0x7d() {
    panic!("Interrupt 0x7d !!!");
}

extern "x86-interrupt" fn handler_0x7e() {
    panic!("Interrupt 0x7e !!!");
}

extern "x86-interrupt" fn handler_0x7f() {
    panic!("Interrupt 0x7f !!!");
}

extern "x86-interrupt" fn handler_0x80() {
    panic!("Interrupt 0x80 !!!");
}

extern "x86-interrupt" fn handler_0x81() {
    panic!("Interrupt 0x81 !!!");
}

extern "x86-interrupt" fn handler_0x82() {
    panic!("Interrupt 0x82 !!!");
}

extern "x86-interrupt" fn handler_0x83() {
    panic!("Interrupt 0x83 !!!");
}

extern "x86-interrupt" fn handler_0x84() {
    panic!("Interrupt 0x84 !!!");
}

extern "x86-interrupt" fn handler_0x85() {
    panic!("Interrupt 0x85 !!!");
}

extern "x86-interrupt" fn handler_0x86() {
    panic!("Interrupt 0x86 !!!");
}

extern "x86-interrupt" fn handler_0x87() {
    panic!("Interrupt 0x87 !!!");
}

extern "x86-interrupt" fn handler_0x88() {
    panic!("Interrupt 0x88 !!!");
}

extern "x86-interrupt" fn handler_0x89() {
    panic!("Interrupt 0x89 !!!");
}

extern "x86-interrupt" fn handler_0x8a() {
    panic!("Interrupt 0x8a !!!");
}

extern "x86-interrupt" fn handler_0x8b() {
    panic!("Interrupt 0x8b !!!");
}

extern "x86-interrupt" fn handler_0x8c() {
    panic!("Interrupt 0x8c !!!");
}

extern "x86-interrupt" fn handler_0x8d() {
    panic!("Interrupt 0x8d !!!");
}

extern "x86-interrupt" fn handler_0x8e() {
    panic!("Interrupt 0x8e !!!");
}

extern "x86-interrupt" fn handler_0x8f() {
    panic!("Interrupt 0x8f !!!");
}

extern "x86-interrupt" fn handler_0x90() {
    panic!("Interrupt 0x90 !!!");
}

extern "x86-interrupt" fn handler_0x91() {
    panic!("Interrupt 0x91 !!!");
}

extern "x86-interrupt" fn handler_0x92() {
    panic!("Interrupt 0x92 !!!");
}

extern "x86-interrupt" fn handler_0x93() {
    panic!("Interrupt 0x93 !!!");
}

extern "x86-interrupt" fn handler_0x94() {
    panic!("Interrupt 0x94 !!!");
}

extern "x86-interrupt" fn handler_0x95() {
    panic!("Interrupt 0x95 !!!");
}

extern "x86-interrupt" fn handler_0x96() {
    panic!("Interrupt 0x96 !!!");
}

extern "x86-interrupt" fn handler_0x97() {
    panic!("Interrupt 0x97 !!!");
}

extern "x86-interrupt" fn handler_0x98() {
    panic!("Interrupt 0x98 !!!");
}

extern "x86-interrupt" fn handler_0x99() {
    panic!("Interrupt 0x99 !!!");
}

extern "x86-interrupt" fn handler_0x9a() {
    panic!("Interrupt 0x9a !!!");
}

extern "x86-interrupt" fn handler_0x9b() {
    panic!("Interrupt 0x9b !!!");
}

extern "x86-interrupt" fn handler_0x9c() {
    panic!("Interrupt 0x9c !!!");
}

extern "x86-interrupt" fn handler_0x9d() {
    panic!("Interrupt 0x9d !!!");
}

extern "x86-interrupt" fn handler_0x9e() {
    panic!("Interrupt 0x9e !!!");
}

extern "x86-interrupt" fn handler_0x9f() {
    panic!("Interrupt 0x9f !!!");
}

extern "x86-interrupt" fn handler_0xa0() {
    panic!("Interrupt 0xa0 !!!");
}

extern "x86-interrupt" fn handler_0xa1() {
    panic!("Interrupt 0xa1 !!!");
}

extern "x86-interrupt" fn handler_0xa2() {
    panic!("Interrupt 0xa2 !!!");
}

extern "x86-interrupt" fn handler_0xa3() {
    panic!("Interrupt 0xa3 !!!");
}

extern "x86-interrupt" fn handler_0xa4() {
    panic!("Interrupt 0xa4 !!!");
}

extern "x86-interrupt" fn handler_0xa5() {
    panic!("Interrupt 0xa5 !!!");
}

extern "x86-interrupt" fn handler_0xa6() {
    panic!("Interrupt 0xa6 !!!");
}

extern "x86-interrupt" fn handler_0xa7() {
    panic!("Interrupt 0xa7 !!!");
}

extern "x86-interrupt" fn handler_0xa8() {
    panic!("Interrupt 0xa8 !!!");
}

extern "x86-interrupt" fn handler_0xa9() {
    panic!("Interrupt 0xa9 !!!");
}

extern "x86-interrupt" fn handler_0xaa() {
    panic!("Interrupt 0xaa !!!");
}

extern "x86-interrupt" fn handler_0xab() {
    panic!("Interrupt 0xab !!!");
}

extern "x86-interrupt" fn handler_0xac() {
    panic!("Interrupt 0xac !!!");
}

extern "x86-interrupt" fn handler_0xad() {
    panic!("Interrupt 0xad !!!");
}

extern "x86-interrupt" fn handler_0xae() {
    panic!("Interrupt 0xae !!!");
}

extern "x86-interrupt" fn handler_0xaf() {
    panic!("Interrupt 0xaf !!!");
}

extern "x86-interrupt" fn handler_0xb0() {
    panic!("Interrupt 0xb0 !!!");
}

extern "x86-interrupt" fn handler_0xb1() {
    panic!("Interrupt 0xb1 !!!");
}

extern "x86-interrupt" fn handler_0xb2() {
    panic!("Interrupt 0xb2 !!!");
}

extern "x86-interrupt" fn handler_0xb3() {
    panic!("Interrupt 0xb3 !!!");
}

extern "x86-interrupt" fn handler_0xb4() {
    panic!("Interrupt 0xb4 !!!");
}

extern "x86-interrupt" fn handler_0xb5() {
    panic!("Interrupt 0xb5 !!!");
}

extern "x86-interrupt" fn handler_0xb6() {
    panic!("Interrupt 0xb6 !!!");
}

extern "x86-interrupt" fn handler_0xb7() {
    panic!("Interrupt 0xb7 !!!");
}

extern "x86-interrupt" fn handler_0xb8() {
    panic!("Interrupt 0xb8 !!!");
}

extern "x86-interrupt" fn handler_0xb9() {
    panic!("Interrupt 0xb9 !!!");
}

extern "x86-interrupt" fn handler_0xba() {
    panic!("Interrupt 0xba !!!");
}

extern "x86-interrupt" fn handler_0xbb() {
    panic!("Interrupt 0xbb !!!");
}

extern "x86-interrupt" fn handler_0xbc() {
    panic!("Interrupt 0xbc !!!");
}

extern "x86-interrupt" fn handler_0xbd() {
    panic!("Interrupt 0xbd !!!");
}

extern "x86-interrupt" fn handler_0xbe() {
    panic!("Interrupt 0xbe !!!");
}

extern "x86-interrupt" fn handler_0xbf() {
    panic!("Interrupt 0xbf !!!");
}

extern "x86-interrupt" fn handler_0xc0() {
    panic!("Interrupt 0xc0 !!!");
}

extern "x86-interrupt" fn handler_0xc1() {
    panic!("Interrupt 0xc1 !!!");
}

extern "x86-interrupt" fn handler_0xc2() {
    panic!("Interrupt 0xc2 !!!");
}

extern "x86-interrupt" fn handler_0xc3() {
    panic!("Interrupt 0xc3 !!!");
}

extern "x86-interrupt" fn handler_0xc4() {
    panic!("Interrupt 0xc4 !!!");
}

extern "x86-interrupt" fn handler_0xc5() {
    panic!("Interrupt 0xc5 !!!");
}

extern "x86-interrupt" fn handler_0xc6() {
    panic!("Interrupt 0xc6 !!!");
}

extern "x86-interrupt" fn handler_0xc7() {
    panic!("Interrupt 0xc7 !!!");
}

extern "x86-interrupt" fn handler_0xc8() {
    panic!("Interrupt 0xc8 !!!");
}

extern "x86-interrupt" fn handler_0xc9() {
    panic!("Interrupt 0xc9 !!!");
}

extern "x86-interrupt" fn handler_0xca() {
    panic!("Interrupt 0xca !!!");
}

extern "x86-interrupt" fn handler_0xcb() {
    panic!("Interrupt 0xcb !!!");
}

extern "x86-interrupt" fn handler_0xcc() {
    panic!("Interrupt 0xcc !!!");
}

extern "x86-interrupt" fn handler_0xcd() {
    panic!("Interrupt 0xcd !!!");
}

extern "x86-interrupt" fn handler_0xce() {
    panic!("Interrupt 0xce !!!");
}

extern "x86-interrupt" fn handler_0xcf() {
    panic!("Interrupt 0xcf !!!");
}

extern "x86-interrupt" fn handler_0xd0() {
    panic!("Interrupt 0xd0 !!!");
}

extern "x86-interrupt" fn handler_0xd1() {
    panic!("Interrupt 0xd1 !!!");
}

extern "x86-interrupt" fn handler_0xd2() {
    panic!("Interrupt 0xd2 !!!");
}

extern "x86-interrupt" fn handler_0xd3() {
    panic!("Interrupt 0xd3 !!!");
}

extern "x86-interrupt" fn handler_0xd4() {
    panic!("Interrupt 0xd4 !!!");
}

extern "x86-interrupt" fn handler_0xd5() {
    panic!("Interrupt 0xd5 !!!");
}

extern "x86-interrupt" fn handler_0xd6() {
    panic!("Interrupt 0xd6 !!!");
}

extern "x86-interrupt" fn handler_0xd7() {
    panic!("Interrupt 0xd7 !!!");
}

extern "x86-interrupt" fn handler_0xd8() {
    panic!("Interrupt 0xd8 !!!");
}

extern "x86-interrupt" fn handler_0xd9() {
    panic!("Interrupt 0xd9 !!!");
}

extern "x86-interrupt" fn handler_0xda() {
    panic!("Interrupt 0xda !!!");
}

extern "x86-interrupt" fn handler_0xdb() {
    panic!("Interrupt 0xdb !!!");
}

extern "x86-interrupt" fn handler_0xdc() {
    panic!("Interrupt 0xdc !!!");
}

extern "x86-interrupt" fn handler_0xdd() {
    panic!("Interrupt 0xdd !!!");
}

extern "x86-interrupt" fn handler_0xde() {
    panic!("Interrupt 0xde !!!");
}

extern "x86-interrupt" fn handler_0xdf() {
    panic!("Interrupt 0xdf !!!");
}

extern "x86-interrupt" fn handler_0xe0() {
    panic!("Interrupt 0xe0 !!!");
}

extern "x86-interrupt" fn handler_0xe1() {
    panic!("Interrupt 0xe1 !!!");
}

extern "x86-interrupt" fn handler_0xe2() {
    panic!("Interrupt 0xe2 !!!");
}

extern "x86-interrupt" fn handler_0xe3() {
    panic!("Interrupt 0xe3 !!!");
}

extern "x86-interrupt" fn handler_0xe4() {
    panic!("Interrupt 0xe4 !!!");
}

extern "x86-interrupt" fn handler_0xe5() {
    panic!("Interrupt 0xe5 !!!");
}

extern "x86-interrupt" fn handler_0xe6() {
    panic!("Interrupt 0xe6 !!!");
}

extern "x86-interrupt" fn handler_0xe7() {
    panic!("Interrupt 0xe7 !!!");
}

extern "x86-interrupt" fn handler_0xe8() {
    panic!("Interrupt 0xe8 !!!");
}

extern "x86-interrupt" fn handler_0xe9() {
    panic!("Interrupt 0xe9 !!!");
}

extern "x86-interrupt" fn handler_0xea() {
    panic!("Interrupt 0xea !!!");
}

extern "x86-interrupt" fn handler_0xeb() {
    panic!("Interrupt 0xeb !!!");
}

extern "x86-interrupt" fn handler_0xec() {
    panic!("Interrupt 0xec !!!");
}

extern "x86-interrupt" fn handler_0xed() {
    panic!("Interrupt 0xed !!!");
}

extern "x86-interrupt" fn handler_0xee() {
    panic!("Interrupt 0xee !!!");
}

extern "x86-interrupt" fn handler_0xef() {
    panic!("Interrupt 0xef !!!");
}

extern "x86-interrupt" fn handler_0xf0() {
    panic!("Interrupt 0xf0 !!!");
}

extern "x86-interrupt" fn handler_0xf1() {
    panic!("Interrupt 0xf1 !!!");
}

extern "x86-interrupt" fn handler_0xf2() {
    panic!("Interrupt 0xf2 !!!");
}

extern "x86-interrupt" fn handler_0xf3() {
    panic!("Interrupt 0xf3 !!!");
}

extern "x86-interrupt" fn handler_0xf4() {
    panic!("Interrupt 0xf4 !!!");
}

extern "x86-interrupt" fn handler_0xf5() {
    panic!("Interrupt 0xf5 !!!");
}

extern "x86-interrupt" fn handler_0xf6() {
    panic!("Interrupt 0xf6 !!!");
}

extern "x86-interrupt" fn handler_0xf7() {
    panic!("Interrupt 0xf7 !!!");
}

extern "x86-interrupt" fn handler_0xf8() {
    panic!("Interrupt 0xf8 !!!");
}

extern "x86-interrupt" fn handler_0xf9() {
    panic!("Interrupt 0xf9 !!!");
}

extern "x86-interrupt" fn handler_0xfa() {
    panic!("Interrupt 0xfa !!!");
}

extern "x86-interrupt" fn handler_0xfb() {
    panic!("Interrupt 0xfb !!!");
}

extern "x86-interrupt" fn handler_0xfc() {
    panic!("Interrupt 0xfc !!!");
}

extern "x86-interrupt" fn handler_0xfd() {
    panic!("Interrupt 0xfd !!!");
}

extern "x86-interrupt" fn handler_0xfe() {
    panic!("Interrupt 0xfe !!!");
}

extern "x86-interrupt" fn handler_0xff() {
    panic!("Interrupt 0xff !!!");
}

