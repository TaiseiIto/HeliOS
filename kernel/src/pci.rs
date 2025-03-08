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
pub mod msi;
pub mod secondary_status;
pub mod status;
pub mod xhci;

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
        assert_eq!(register % 4, 0);
        Self::new()
            .with_enable(true)
            .with_bus(bus)
            .with_device(device)
            .with_function(function)
            .with_register(register >> 2)
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

    pub fn write_u8(self, data: u8) {
        let address: u32 = self.into();
        x64::port::outl(Self::ADDRESS_PORT, address);
        x64::port::outb(Self::DATA_PORT, data)
    }

    pub fn write_u16(self, data: u16) {
        let address: u32 = self.into();
        x64::port::outl(Self::ADDRESS_PORT, address);
        x64::port::outw(Self::DATA_PORT, data)
    }

    pub fn write_u32(self, data: u32) {
        let address: u32 = self.into();
        x64::port::outl(Self::ADDRESS_PORT, address);
        x64::port::outl(Self::DATA_PORT, data)
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
#[derive(Debug)]
pub struct Function {
    space: [u32; Self::LENGTH],
}

impl Function {
    const LENGTH: usize = 0x40;

    pub fn read(bus: u8, device: u8, function: u8) -> Option<Self> {
        let space: Vec<u32> = (u8::MIN..=u8::MAX)
            .filter(|register| register % 4 == 0)
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
}

pub enum Header<'a> {
    Type0(&'a Type0),
    Type1(&'a Type1),
}

impl Header<'_> {
    fn base_address_registers(&self) -> Vec<base_address::Register> {
        match self {
            Self::Type0(type0) => type0.base_address_registers(),
            Self::Type1(type1) => type1.base_address_registers(),
        }
    }

    fn capabilities_pointer(&self) -> u8 {
        match self {
            Self::Type0(type0) => type0.capabilities_pointer,
            Self::Type1(type1) => type1.capabilities_pointer,
        }
    }

    fn class_code(&self) -> class::Code {
        match self {
            Self::Type0(type0) => type0.class_code,
            Self::Type1(type1) => type1.class_code,
        }.into()
    }
}

impl<'a> From<&'a Function> for Header<'a> {
    fn from(function: &'a Function) -> Self {
        match (function.try_into(), function.try_into()) {
            (Ok(type0), Err(_)) => Self::Type0(type0),
            (Err(_), Ok(type1)) => Self::Type1(type1),
            _ => unreachable!(),
        }
    }
}

/// # Type 0 Configuration Space Header
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2 Type 0 Configuration Space Header Figure 7-10 Type 0 Configuration Space Header
#[repr(packed)]
pub struct Type0 {
    vendor_id: u16,
    device_id: u16,
    command: command::Register,
    status: status::Register,
    revision_id: u8,
    class_code: class::Register,
    cash_line_size: u8,
    latency_timer: u8,
    header_type: header_type::Register,
    bist: bist::Register,
    base_address_registers: [u32; 6],
    cardbus_cis_pointer: u32,
    subsystem_vendor_id: u16,
    subsystem_id: u16,
    expansion_rom_base_address: expansion_rom_base_address::Register,
    capabilities_pointer: u8,
    __: [u8; 7],
    interrupt_line: u8,
    interrupt_pin: u8,
    min_gnt: u8,
    min_lat: u8,
}

impl Type0 {
    fn base_address_registers(&self) -> Vec<base_address::Register> {
        self.base_address_registers
            .as_slice()
            .iter()
            .map(|base_address_register| base_address_register.into())
            .collect()
    }
}

impl<'a> TryFrom<&'a Function> for &'a Type0 {
    type Error = ();

    fn try_from(function: &'a Function) -> Result<Self, Self::Error> {
        let function: *const Function = function as *const Function;
        let type0: *const Type0 = function as *const Type0;
        let type0: &Type0 = unsafe {
            &*type0
        };
        let header_type: header_type::Register = type0.header_type;
        match header_type.into() {
            header_type::Type::Zero => Ok(type0),
            header_type::Type::One => Err(()),
        }
    }
}

/// # Type 1 Configuration Space Header
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.3 Type 1 Configuration Space Header Figure 7-14 Type 1 Configuration Space Header
#[repr(packed)]
pub struct Type1 {
    vendor_id: u16,
    device_id: u16,
    command: command::Register,
    status: status::Register,
    revision_id: u8,
    class_code: class::Register,
    cash_line_size: u8,
    latency_timer: u8,
    header_type: header_type::Register,
    bist: bist::Register,
    base_address_registers: [u32; 2],
    primary_bus_number: u8,
    secondary_bus_number: u8,
    subordinate_bus_number: u8,
    secondary_latency_timer: u8,
    io_base: u8,
    io_limit: u8,
    secondary_status: secondary_status::Register,
    mamory_base: u16,
    memory_limit: u16,
    prefetchable_memory_base: u16,
    prefetchable_memory_limit: u16,
    prefetchable_memory_base_upper_32bits: u32,
    prefetchable_memory_limit_upper_32bits: u32,
    io_base_upper_16bits: u16,
    io_limit_upper_16bits: u16,
    capabilities_pointer: u8,
    __: [u8; 3],
    expantion_rom_base_address: expansion_rom_base_address::Register,
    interrupt_line: u8,
    interrupt_pin: u8,
    bridge_control: bridge_control::Register,
}

impl Type1 {
    fn base_address_registers(&self) -> Vec<base_address::Register> {
        self.base_address_registers
            .as_slice()
            .iter()
            .map(|base_address_register| base_address_register.into())
            .collect()
    }
}

impl<'a> TryFrom<&'a Function> for &'a Type1 {
    type Error = ();

    fn try_from(function: &'a Function) -> Result<Self, Self::Error> {
        let function: *const Function = function as *const Function;
        let type1: *const Type1 = function as *const Type1;
        let type1: &Type1 = unsafe {
            &*type1
        };
        let header_type: header_type::Register = type1.header_type;
        match header_type.into() {
            header_type::Type::Zero => Err(()),
            header_type::Type::One => Ok(type1),
        }
    }
}

