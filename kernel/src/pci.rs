//! # PCI (Peripheral Component Interconnect)
//! ## References
//! * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf)

pub mod bist;
pub mod command;
pub mod header_type;
pub mod status;

use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::fmt,
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

/// # PCI Function
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.1 Type 0/1 Common Configuration Space Figure 7-4 Common Configuration Space Header
pub struct Function {
    space: [u32; Self::LENGTH],
}

impl Function {
    const LENGTH: usize = 0x40;

    pub fn read(bus: u8, device: u8, function: u8) -> Self {
        let space: Vec<u32> = (0u8..Self::LENGTH as u8)
            .map(|register| Address::create(bus, device, function, register).read())
            .collect();
        let space: [u32; Self::LENGTH] = space
            .try_into()
            .unwrap();
        Self {
            space
        }
    }

    pub fn vendor_id(&self) -> u16 {
        (self.space[0] & (u16::MAX as u32)) as u16
    }

    pub fn device_id(&self) -> u16 {
        (self.space[0] >> u16::BITS) as u16
    }

    pub fn command(&self) -> command::Register {
        let command: u16 = (self.space[1] & (u16::MAX as u32)) as u16;
        command.into()
    }

    pub fn status(&self) -> status::Register {
        let status: u16 = (self.space[1] >> u16::BITS) as u16;
        status.into()
    }

    pub fn revision_id(&self) -> u8 {
        (self.space[2] & (u8::MAX as u32)) as u8
    }

    pub fn programming_interface(&self) -> u8 {
        ((self.space[2] >> u8::BITS) & (u8::MAX as u32)) as u8
    }

    pub fn sub_class_code(&self) -> u8 {
        ((self.space[2] >> (2 * u8::BITS)) & (u8::MAX as u32)) as u8
    }

    pub fn base_class_code(&self) -> u8 {
        ((self.space[2] >> (3 * u8::BITS)) & (u8::MAX as u32)) as u8
    }

    pub fn cache_line_size(&self) -> u8 {
        (self.space[3] & (u8::MAX as u32)) as u8
    }

    pub fn latency_timer(&self) -> u8 {
        ((self.space[3] >> u8::BITS) & (u8::MAX as u32)) as u8
    }

    pub fn header_type(&self) -> header_type::Register {
        let header_type: u8 = ((self.space[3] >> (2 * u8::BITS)) & (u8::MAX as u32)) as u8;
        header_type.into()
    }

    pub fn bist(&self) -> bist::Register {
        let bist: u8 = ((self.space[3] >> (3 * u8::BITS)) & (u8::MAX as u32)) as u8;
        bist.into()
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Function")
            .field("vendor_id", &self.vendor_id())
            .field("device_id", &self.device_id())
            .field("command", &self.command())
            .field("status", &self.status())
            .field("revision_id", &self.revision_id())
            .field("programming_interface", &self.programming_interface())
            .field("sub_class_code", &self.sub_class_code())
            .field("base_class_code", &self.base_class_code())
            .field("cache_line_size", &self.cache_line_size())
            .field("latency_timer", &self.latency_timer())
            .field("header_type", &self.header_type())
            .field("bist", &self.bist())
            .finish()
    }
}

