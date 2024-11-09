//! # PCI (Peripheral Component Interconnect)
//! ## References
//! * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf)

pub mod base_address;
pub mod bist;
pub mod bridge_control;
pub mod class;
pub mod command;
pub mod expansion_rom_base_address;
pub mod header_type;
pub mod secondary_status;
pub mod status;

use {
    alloc::{
        collections::{
            btree_map::BTreeMap,
            btree_set::BTreeSet,
        },
        vec::Vec,
    },
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
        ops,
    },
    crate::{
        com2_println,
        x64,
    },
};

/// # CFGADR - Configuration Address Register
/// ## References
/// * [Intel E8500 Chipset North Bridge (NB)](https://www.intel.co.jp/content/dam/doc/datasheet/e8500-chipset-north-bridge-datasheet.pdf) 4.6.1 CFGADR - Configuration Address Register
#[bitfield(u32)]
pub struct Address {
    #[bits(2)]
    __: u8,
    #[bits(6)]
    register: u8,
    #[bits(3)]
    function: u8,
    #[bits(5)]
    device: u8,
    bus: u8,
    #[bits(7)]
    __: u8,
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

    pub fn read_u8(self) -> u8 {
        let address: u32 = self.into();
        x64::port::outl(Self::ADDRESS_PORT, address);
        x64::port::inb(Self::DATA_PORT)
    }

    pub fn read_u16(self) -> u16 {
        let address: u32 = self.into();
        x64::port::outl(Self::ADDRESS_PORT, address);
        x64::port::inw(Self::DATA_PORT)
    }

    pub fn read_u32(self) -> u32 {
        let address: u32 = self.into();
        x64::port::outl(Self::ADDRESS_PORT, address);
        x64::port::inl(Self::DATA_PORT)
    }

    fn device_range() -> ops::RangeInclusive<u8> {
        0..=(1 << Self::DEVICE_BITS) - 1
    }

    fn function_range() -> ops::RangeInclusive<u8> {
        0..=(1 << Self::FUNCTION_BITS) - 1
    }
}

/// # PCI
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf)
#[derive(Debug, Default)]
pub struct Configuration {
    buses: BTreeMap<u8, Bus>,
}

impl Configuration {
    pub fn read() -> Self {
        let buses: BTreeMap<u8, Bus> = BTreeMap::new();
        let mut pci = Self {
            buses,
        };
        let bus_number: u8 = 0;
        let device_number: u8 = 0;
        let function_number: u8 = 0;
        pci.scan(bus_number, device_number, function_number);
        pci
    }

    fn add(&mut self, bus_number: u8, device_number: u8, function_number: u8, function: Function) {
        self.buses
            .entry(bus_number)
            .or_default()
            .add(device_number, function_number, function);
    }

    fn has(&self, bus_number: u8, device_number: u8, function_number: u8) -> bool {
        self.buses
            .get(&bus_number)
            .map_or(false, |bus| bus.has(device_number, function_number))
    }

    fn scan(&mut self, bus_number: u8, device_number: u8, function_number: u8) {
        if !self.has(bus_number, device_number, function_number) {
            if let Some(function) = Function::read(bus_number, device_number, function_number) {
                com2_println!("Scan PCI ({:#x?}, {:#x?}, {:#x?})", bus_number, device_number, function_number);
                let mut next_addresses: BTreeSet<(u8, u8, u8)> = BTreeSet::new();
                match function.class_code() {
                    class::Code::HostBridge => {
                        let bus_number: u8 = function_number;
                        let function_number: u8 = 0;
                        next_addresses
                            .extend(Address::device_range()
                                .map(|device_number| (bus_number, device_number, function_number)));
                    },
                    class::Code::Pci2PciBridge | class::Code::SubtractiveDecodePci2PciBridge => if let Some(secondary_bus_number) = function.secondary_bus_number() {
                        let bus_number: u8 = secondary_bus_number;
                        let function_number: u8 = 0;
                        next_addresses
                            .extend(Address::device_range()
                                .map(|device_number| (bus_number, device_number, function_number)));
                    },
                    _ => {},
                }
                if function_number == 0 && function.is_multi_function_device() {
                    next_addresses
                        .extend(Address::function_range()
                            .map(|function_number| (bus_number, device_number, function_number)));
                }
                self.add(bus_number, device_number, function_number, function);
                next_addresses
                    .into_iter()
                    .for_each(|(bus_number, device_number, function_number)| self.scan(bus_number, device_number, function_number));
            }
        }
    }
}

/// # PCI Bus
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf)
#[derive(Debug, Default)]
pub struct Bus {
    devices: BTreeMap<u8, Device>,
}

impl Bus {
    fn add(&mut self, device_number: u8, function_number: u8, function: Function) {
        self.devices
            .entry(device_number)
            .or_default()
            .add(function_number, function);
    }

    fn has(&self, device_number: u8, function_number: u8) -> bool {
        self.devices
            .get(&device_number)
            .map_or(false, |device| device.has(function_number))
    }
}

/// # PCI Device
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf)
#[derive(Debug, Default)]
pub struct Device {
    functions: BTreeMap<u8, Function>,
}

impl Device {
    fn add(&mut self, function_number: u8, function: Function) {
        self.functions
            .insert(function_number, function);
    }

    fn has(&self, function_number: u8) -> bool {
        self.functions
            .get(&function_number)
            .is_some()
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

    pub fn read(bus: u8, device: u8, function: u8) -> Option<Self> {
        let space: Vec<u32> = (0u8..Self::LENGTH as u8)
            .map(|register| Address::create(bus, device, function, register).read_u32())
            .collect();
        let space: [u32; Self::LENGTH] = space
            .try_into()
            .unwrap();
        let function = Self {
            space
        };
        (function.vendor_id() != 0xffff).then_some(function)
    }

    pub fn is_multi_function_device(&self) -> bool {
        self.header_type()
            .is_multi_function_device()
    }

    fn vendor_id(&self) -> u16 {
        let vendor_id_and_device_id: u32 = self.space[0];
        let vendor_id_and_device_id: [u16; 2] = unsafe {
            mem::transmute(vendor_id_and_device_id)
        };
        vendor_id_and_device_id[0]
    }

    fn device_id(&self) -> u16 {
        let vendor_id_and_device_id: u32 = self.space[0];
        let vendor_id_and_device_id: [u16; 2] = unsafe {
            mem::transmute(vendor_id_and_device_id)
        };
        vendor_id_and_device_id[1]
    }

    fn command(&self) -> command::Register {
        let command_and_status: u32 = self.space[1];
        let command_and_status: [u16; 2] = unsafe {
            mem::transmute(command_and_status)
        };
        let command: u16 = command_and_status[0];
        command.into()
    }

    fn status(&self) -> status::Register {
        let command_and_status: u32 = self.space[1];
        let command_and_status: [u16; 2] = unsafe {
            mem::transmute(command_and_status)
        };
        let status: u16 = command_and_status[1];
        status.into()
    }

    fn revision_id(&self) -> u8 {
        self.space[2].to_le_bytes()[0]
    }

	fn class_code(&self) -> class::Code {
		let base_class: u8 = self.base_class();
		let sub_class: u8 = self.sub_class();
		let programming_interface: u8 = self.programming_interface();
		class::Code::new(base_class, sub_class, programming_interface)
	}

    fn programming_interface(&self) -> u8 {
        self.space[2].to_le_bytes()[1]
    }

    fn sub_class(&self) -> u8 {
        self.space[2].to_le_bytes()[2]
    }

    fn base_class(&self) -> u8 {
        self.space[2].to_le_bytes()[3]
    }

    fn cache_line_size(&self) -> u8 {
        self.space[3].to_le_bytes()[0]
    }

    fn latency_timer(&self) -> u8 {
        self.space[3].to_le_bytes()[1]
    }

    fn header_type(&self) -> header_type::Register {
        let header_type: u8 = self.space[3].to_le_bytes()[2];
        header_type.into()
    }

    fn bist(&self) -> bist::Register {
        let bist: u8 = self.space[3].to_le_bytes()[3];
        bist.into()
    }

    fn base_address_registers(&self) -> Vec<base_address::Register> {
        let header_type: header_type::Type = self.into();
        let end_index: usize = match header_type {
            header_type::Type::Zero => 10,
            header_type::Type::One => 6,
        };
        (4..end_index)
            .map(|index| self.space[index].into())
            .collect()
    }

    fn cardbus_cis_pointer(&self) -> Option<u32> {
        match self.into() {
            header_type::Type::Zero => Some(self.space[10]),
            header_type::Type::One => None,
        }
    }

    fn subsystem_vendor_id(&self) -> Option<u16> {
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

    fn subsystem_id(&self) -> Option<u16> {
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

    fn expansion_rom_base_address(&self) -> expansion_rom_base_address::Register {
        let index: usize = match self.into() {
            header_type::Type::Zero => 12,
            header_type::Type::One => 14,
        };
        self.space[index].into()
    }

    fn primary_bus_number(&self) -> Option<u8> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => Some(self.space[6].to_le_bytes()[0]),
        }
    }

    fn secondary_bus_number(&self) -> Option<u8> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => Some(self.space[6].to_le_bytes()[1]),
        }
    }

    fn subordinate_bus_number(&self) -> Option<u8> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => Some(self.space[6].to_le_bytes()[2]),
        }
    }

    fn secondary_latency_timer(&self) -> Option<u8> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => Some(self.space[6].to_le_bytes()[3]),
        }
    }

    fn io_base(&self) -> Option<u8> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => Some(self.space[7].to_le_bytes()[0]),
        }
    }

    fn io_limit(&self) -> Option<u8> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => Some(self.space[7].to_le_bytes()[1]),
        }
    }

    fn secondary_status(&self) -> Option<secondary_status::Register> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => {
                let io_base_and_io_limit_and_secondary_status: u32 = self.space[7];
                let io_base_and_io_limit_and_secondary_status: [u16; 2] = unsafe {
                    mem::transmute(io_base_and_io_limit_and_secondary_status)
                };
                Some(io_base_and_io_limit_and_secondary_status[1].into())
            },
        }
    }

    fn memory_base(&self) -> Option<u16> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => {
                let memory_base_and_memory_limit: u32 = self.space[8];
                let memory_base_and_memory_limit: [u16; 2] = unsafe {
                    mem::transmute(memory_base_and_memory_limit)
                };
                Some(memory_base_and_memory_limit[0])
            },
        }
    }

    fn memory_limit(&self) -> Option<u16> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => {
                let memory_base_and_memory_limit: u32 = self.space[8];
                let memory_base_and_memory_limit: [u16; 2] = unsafe {
                    mem::transmute(memory_base_and_memory_limit)
                };
                Some(memory_base_and_memory_limit[1])
            },
        }
    }

    fn prefetchable_memory_base(&self) -> Option<u16> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => {
                let prefetchable_memory_base_and_prefetchable_memory_limit: u32 = self.space[9];
                let prefetchable_memory_base_and_prefetchable_memory_limit: [u16; 2] = unsafe {
                    mem::transmute(prefetchable_memory_base_and_prefetchable_memory_limit)
                };
                Some(prefetchable_memory_base_and_prefetchable_memory_limit[0])
            },
        }
    }

    fn prefetchable_memory_limit(&self) -> Option<u16> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => {
                let prefetchable_memory_base_and_prefetchable_memory_limit: u32 = self.space[9];
                let prefetchable_memory_base_and_prefetchable_memory_limit: [u16; 2] = unsafe {
                    mem::transmute(prefetchable_memory_base_and_prefetchable_memory_limit)
                };
                Some(prefetchable_memory_base_and_prefetchable_memory_limit[1])
            },
        }
    }

    fn prefetchable_memory_base_upper_32bits(&self) -> Option<u32> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => Some(self.space[10]),
        }
    }

    fn prefetchable_memory_limit_upper_32bits(&self) -> Option<u32> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => Some(self.space[11]),
        }
    }

    fn io_base_upper_16bits(&self) -> Option<u16> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => {
                let io_base_upper_16bits_and_io_limit_upper_16bits: u32 = self.space[12];
                let io_base_upper_16bits_and_io_limit_upper_16bits: [u16; 2] = unsafe {
                    mem::transmute(io_base_upper_16bits_and_io_limit_upper_16bits)
                };
                Some(io_base_upper_16bits_and_io_limit_upper_16bits[0])
            },
        }
    }

    fn io_limit_upper_16bits(&self) -> Option<u16> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => {
                let io_base_upper_16bits_and_io_limit_upper_16bits: u32 = self.space[12];
                let io_base_upper_16bits_and_io_limit_upper_16bits: [u16; 2] = unsafe {
                    mem::transmute(io_base_upper_16bits_and_io_limit_upper_16bits)
                };
                Some(io_base_upper_16bits_and_io_limit_upper_16bits[1])
            },
        }
    }

    fn capabilities_pointer(&self) -> u8 {
        self.space[13].to_le_bytes()[0]
    }

    fn interrupt_line(&self) -> u8 {
        self.space[15].to_le_bytes()[0]
    }

    fn interrupt_pin(&self) -> u8 {
        self.space[15].to_le_bytes()[1]
    }

    fn min_gnt(&self) -> Option<u8> {
        match self.into() {
            header_type::Type::Zero => Some(self.space[15].to_le_bytes()[2]),
            header_type::Type::One => None,
        }
    }

    fn min_lat(&self) -> Option<u8> {
        match self.into() {
            header_type::Type::Zero => Some(self.space[15].to_le_bytes()[3]),
            header_type::Type::One => None,
        }
    }

    fn bridge_control(&self) -> Option<bridge_control::Register> {
        match self.into() {
            header_type::Type::Zero => None,
            header_type::Type::One => {
                let interrupt_line_and_interrupt_pin_and_bridge_control: u32 = self.space[15];
                let interrupt_line_and_interrupt_pin_and_bridge_control: [u16; 2] = unsafe {
                    mem::transmute(interrupt_line_and_interrupt_pin_and_bridge_control)
                };
                Some(interrupt_line_and_interrupt_pin_and_bridge_control[1].into())
            },
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
            .field("class_code", &self.class_code())
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
        if let header_type::Type::Zero = self.into() {
            debug.field("expansion_rom_base_address", &self.expansion_rom_base_address());
        }
        if let Some(primary_bus_number) = self.primary_bus_number() {
            debug.field("primary_bus_number", &primary_bus_number);
        }
        if let Some(secondary_bus_number) = self.secondary_bus_number() {
            debug.field("secondary_bus_number", &secondary_bus_number);
        }
        if let Some(subordinate_bus_number) = self.subordinate_bus_number() {
            debug.field("subordinate_bus_number", &subordinate_bus_number);
        }
        if let Some(secondary_latency_timer) = self.secondary_latency_timer() {
            debug.field("secondary_latency_timer", &secondary_latency_timer);
        }
        if let Some(io_base) = self.io_base() {
            debug.field("io_base", &io_base);
        }
        if let Some(io_limit) = self.io_limit() {
            debug.field("io_limit", &io_limit);
        }
        if let Some(secondary_status) = self.secondary_status() {
            debug.field("secondary_status", &secondary_status);
        }
        if let Some(memory_base) = self.memory_base() {
            debug.field("memory_base", &memory_base);
        }
        if let Some(memory_limit) = self.memory_limit() {
            debug.field("memory_limit", &memory_limit);
        }
        if let Some(prefetchable_memory_base) = self.prefetchable_memory_base() {
            debug.field("prefetchable_memory_base", &prefetchable_memory_base);
        }
        if let Some(prefetchable_memory_limit) = self.prefetchable_memory_limit() {
            debug.field("prefetchable_memory_limit", &prefetchable_memory_limit);
        }
        if let Some(prefetchable_memory_base_upper_32bits) = self.prefetchable_memory_base_upper_32bits() {
            debug.field("prefetchable_memory_base_upper_32bits", &prefetchable_memory_base_upper_32bits);
        }
        if let Some(prefetchable_memory_limit_upper_32bits) = self.prefetchable_memory_limit_upper_32bits() {
            debug.field("prefetchable_memory_limit_upper_32bits", &prefetchable_memory_limit_upper_32bits);
        }
        if let Some(io_base_upper_16bits) = self.io_base_upper_16bits() {
            debug.field("io_base_upper_16bits", &io_base_upper_16bits);
        }
        if let Some(io_limit_upper_16bits) = self.io_limit_upper_16bits() {
            debug.field("io_limit_upper_16bits", &io_limit_upper_16bits);
        }
        debug.field("capabilities_pointer", &self.capabilities_pointer());
        if let header_type::Type::One = self.into() {
            debug.field("expansion_rom_base_address", &self.expansion_rom_base_address());
        }
        debug
            .field("interrupt_line", &self.interrupt_line())
            .field("interrupt_pin", &self.interrupt_pin());
        if let Some(min_gnt) = self.min_gnt() {
            debug.field("min_gnt", &min_gnt);
        }
        if let Some(min_lat) = self.min_lat() {
            debug.field("min_lat", &min_lat);
        }
        if let Some(bridge_control) = self.bridge_control() {
            debug.field("bridge_control", &bridge_control);
        }
        debug.finish()
    }
}

