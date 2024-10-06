//! # PCI Configuration
//! ## References
//! * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf)

use {
    bitfield_struct::bitfield,
    crate::x64,
};

/// # CFGADR - Configuration Address Register
/// ## References
/// * [Intel E8500 Chipset North Bridge (NB)](https://www.intel.co.jp/content/dam/doc/datasheet/e8500-chipset-north-bridge-datasheet.pdf) 4.6.1 CFGADR - Configuration Address Register
#[bitfield(u32)]
pub struct Address {
    #[bits(2, access = RO)]
    reserved0: u8,
    #[bits(6)]
    register: u8,
    #[bits(3)]
    function: u8,
    #[bits(5)]
    device: u8,
    bus: u8,
    #[bits(7)]
    reserved1: u8,
    enable: bool,
}

impl Address {
    const ADDRESS_PORT: u16 = 0x0cf8;
    const DATA_PORT: u16 = 0x0cfc;

    pub fn create(bus: u8, device: u8, function: u8, register: u8) -> Self {
        Self::new()
            .with_enable(true)
            .with_bus(bus)
            .with_device(device)
            .with_function(function)
            .with_register(register)
    }

    pub fn read(self) -> u32 {
        let address: u32 = self.into();
        x64::port::outl(Self::ADDRESS_PORT, address);
        x64::port::inl(Self::DATA_PORT)
    }
}

