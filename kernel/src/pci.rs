//! # PCI (Peripheral Component Interconnect)
//! ## References
//! * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf)

pub mod base_address;
pub mod bist;
pub mod command;
pub mod expansion_rom_base_address;
pub mod header_type;
pub mod status;

use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
    },
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
        let vendor_id_and_device_id: u32 = self.space[0];
        let vendor_id_and_device_id: [u16; 2] = unsafe {
            mem::transmute(vendor_id_and_device_id)
        };
        vendor_id_and_device_id[0]
    }

    pub fn device_id(&self) -> u16 {
        let vendor_id_and_device_id: u32 = self.space[0];
        let vendor_id_and_device_id: [u16; 2] = unsafe {
            mem::transmute(vendor_id_and_device_id)
        };
        vendor_id_and_device_id[1]
    }

    pub fn command(&self) -> command::Register {
        let command_and_status: u32 = self.space[1];
        let command_and_status: [u16; 2] = unsafe {
            mem::transmute(command_and_status)
        };
        let command: u16 = command_and_status[0];
        command.into()
    }

    pub fn status(&self) -> status::Register {
        let command_and_status: u32 = self.space[1];
        let command_and_status: [u16; 2] = unsafe {
            mem::transmute(command_and_status)
        };
        let status: u16 = command_and_status[1];
        status.into()
    }

    pub fn revision_id(&self) -> u8 {
        self.space[2].to_le_bytes()[0]
    }

    pub fn programming_interface(&self) -> u8 {
        self.space[2].to_le_bytes()[1]
    }

    pub fn sub_class_code(&self) -> u8 {
        self.space[2].to_le_bytes()[2]
    }

    pub fn base_class_code(&self) -> u8 {
        self.space[2].to_le_bytes()[3]
    }

    pub fn cache_line_size(&self) -> u8 {
        self.space[3].to_le_bytes()[0]
    }

    pub fn latency_timer(&self) -> u8 {
        self.space[3].to_le_bytes()[1]
    }

    pub fn header_type(&self) -> header_type::Register {
        let header_type: u8 = self.space[3].to_le_bytes()[2];
        header_type.into()
    }

    pub fn bist(&self) -> bist::Register {
        let bist: u8 = self.space[3].to_le_bytes()[3];
        bist.into()
    }

    pub fn base_address_registers(&self) -> Vec<base_address::Register> {
        let header_type: header_type::Type = self.into();
        let end_index: usize = match header_type {
            header_type::Type::Zero => 10,
            header_type::Type::One => 6,
        };
        (4..end_index)
            .map(|index| self.space[index].into())
            .collect()
    }

    pub fn cardbus_cis_pointer(&self) -> Option<u32> {
        match self.into() {
            header_type::Type::Zero => Some(self.space[10]),
            header_type::Type::One => None,
        }
    }

    pub fn subsystem_vendor_id(&self) -> Option<u16> {
        match self.into() {
            header_type::Type::Zero => {
                let subsystem: u32 = self.space[11];
                let subsystem: [u16; 2] = unsafe {
                    mem::transmute(subsystem)
                };
                Some(subsystem[0])
            },
            header_type::Type::One => None,
        }
    }

    pub fn subsystem_id(&self) -> Option<u16> {
        match self.into() {
            header_type::Type::Zero => {
                let subsystem: u32 = self.space[11];
                let subsystem: [u16; 2] = unsafe {
                    mem::transmute(subsystem)
                };
                Some(subsystem[1])
            },
            header_type::Type::One => None,
        }
    }

    pub fn expansion_rom_base_address(&self) -> Option<expansion_rom_base_address::Register> {
        match self.into() {
            header_type::Type::Zero => Some(self.space[12].into()),
            header_type::Type::One => None,
        }
    }

    pub fn capabilities_pointer(&self) -> u8 {
        self.space[13].to_le_bytes()[0]
    }

    pub fn interrupt_line(&self) -> u8 {
        self.space[15].to_le_bytes()[0]
    }

    pub fn interrupt_pin(&self) -> u8 {
        self.space[15].to_le_bytes()[1]
    }

    pub fn min_gnt(&self) -> Option<u8> {
        match self.into() {
            header_type::Type::Zero => Some(self.space[15].to_le_bytes()[2]),
            header_type::Type::One => None,
        }
    }

    pub fn min_lat(&self) -> Option<u8> {
        match self.into() {
            header_type::Type::Zero => Some(self.space[15].to_le_bytes()[3]),
            header_type::Type::One => None,
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug: fmt::DebugStruct = formatter.debug_struct("Function");
        let header_type: header_type::Register = self.header_type();
        debug
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
            .field("header_type", &header_type)
            .field("bist", &self.bist())
            .field("base_address_registers", &self.base_address_registers());
        if let Some(cardbus_cis_pointer) = self.cardbus_cis_pointer() {
            debug.field("cardbus_cis_pointer", &cardbus_cis_pointer);
        }
        if let Some(subsystem_vendor_id) = self.subsystem_vendor_id() {
            debug.field("subsystem_vendor_id", &subsystem_vendor_id);
        }
        if let Some(subsystem_id) = self.subsystem_id() {
            debug.field("subsystem_id", &subsystem_id);
        }
        if let Some(expansion_rom_base_address) = self.expansion_rom_base_address() {
            debug.field("expansion_rom_base_address", &expansion_rom_base_address);
        }
        debug.field("capabilities_pointer", &self.capabilities_pointer());
        debug
            .field("interrupt_line", &self.interrupt_line())
            .field("interrupt_pin", &self.interrupt_pin());
        if let Some(min_gnt) = self.min_gnt() {
            debug.field("min_gnt", &min_gnt);
        }
        if let Some(min_lat) = self.min_lat() {
            debug.field("min_lat", &min_lat);
        }
        debug.finish()
    }
}

