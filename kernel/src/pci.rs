//! # PCI (Peripheral Component Interconnect)
//! ## References
//! * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf)

pub mod base;
pub mod bist;
pub mod bridge_control;
pub mod capability;
pub mod class;
pub mod command;
pub mod expansion_rom_base_address;
pub mod header_type;
pub mod secondary_status;
pub mod status;
pub mod xhc;

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
    crate::x64,
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

    pub fn add(self, offset: usize) -> Self {
        assert_eq!(offset % mem::size_of::<u32>(), 0);
        let address: u32 = self.0;
        let address: u32 = address + (offset as u32);
        address.into()
    }

    pub fn create(bus: u8, device: u8, function: u8, register: u8) -> Self {
        assert_eq!((register as usize) % mem::size_of::<u32>(), 0);
        Self::new()
            .with_enable(true)
            .with_bus(bus)
            .with_device(device)
            .with_function(function)
            .with_register(register >> 2)
    }

    pub fn read(self) -> u32 {
        let address: u32 = self.into();
        x64::port::outl(Self::ADDRESS_PORT, address);
        x64::port::inl(Self::DATA_PORT)
    }

    pub fn write(self, data: u32) {
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
#[derive(Default)]
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

    pub fn reset(&mut self) {
        self.buses
            .values_mut()
            .for_each(|bus| bus.reset())
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
                let mut next_addresses: BTreeSet<(u8, u8, u8)> = BTreeSet::new();
                match function.header().class_code() {
                    class::Code::HostBridge => {
                        let bus_number: u8 = function_number;
                        let function_number: u8 = 0;
                        next_addresses
                            .extend(Address::device_range()
                                .map(|device_number| (bus_number, device_number, function_number)));
                    },
                    class::Code::Pci2PciBridge | class::Code::SubtractiveDecodePci2PciBridge => if let Header::Type1(type1) = function.header() {
                        let bus_number: u8 = type1.secondary_bus_number;
                        let function_number: u8 = 0;
                        next_addresses
                            .extend(Address::device_range()
                                .map(|device_number| (bus_number, device_number, function_number)));
                    },
                    _ => {},
                }
                if function_number == 0 && function.header().header_type().is_multi_function_device() {
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

impl fmt::Debug for Configuration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries(self
                .buses
                .iter()
                .map(|(bus_number, bus)| (*bus_number, BusWithAddress::new(*bus_number, bus))))
            .finish()
    }
}

/// # PCI Bus
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf)
#[derive(Default)]
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

    fn reset(&mut self) {
        self.devices
            .values_mut()
            .for_each(|device| device.reset());
    }
}

pub struct BusWithAddress<'a> {
    bus_number: u8,
    bus: &'a Bus,
}

impl<'a> BusWithAddress<'a> {
    fn new(bus_number: u8, bus: &'a Bus) -> Self {
        Self {
            bus_number,
            bus,
        }
    }
}

impl fmt::Debug for BusWithAddress<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries({
                let Self {
                    bus_number,
                    bus: Bus {
                        devices,
                    },
                } = self;
                devices
                    .iter()
                    .map(|(device_number, device)| (*device_number, DeviceWithAddress::new(*bus_number, *device_number, device)))
            })
            .finish()
    }
}

/// # PCI Device
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf)
#[derive(Default)]
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

    fn reset(&mut self) {
        self.functions
            .values_mut()
            .for_each(|device| device.reset());
    }
}

pub struct DeviceWithAddress<'a> {
    bus_number: u8,
    device_number: u8,
    device: &'a Device,
}

impl<'a> DeviceWithAddress<'a> {
    fn new(bus_number: u8, device_number: u8, device: &'a Device) -> Self {
        Self {
            bus_number,
            device_number,
            device,
        }
    }
}

impl fmt::Debug for DeviceWithAddress<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries({
                let Self {
                    bus_number,
                    device_number,
                    device: Device {
                        functions,
                    },
                } = self;
                functions
                    .iter()
                    .map(|(function_number, function)| (*function_number, FunctionWithAddress::new(*bus_number, *device_number, *function_number, function)))
            })
            .finish()
    }
}

/// # PCI Function
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.1 Type 0/1 Common Configuration Space Figure 7-4 Common Configuration Space Header
pub struct Function {
    #[allow(dead_code)]
    space: [u32; Self::LENGTH],
}

impl Function {
    const LENGTH: usize = 0x40;

    pub fn header(&self) -> Header<'_> {
        self.into()
    }

    pub fn read(bus: u8, device: u8, function: u8) -> Option<Self> {
        let space: Vec<u32> = (u8::MIN..=u8::MAX)
            .filter(|register| register % 4 == 0)
            .map(|register| Address::create(bus, device, function, register).read())
            .collect();
        let space: [u32; Self::LENGTH] = space
            .try_into()
            .unwrap();
        let vendor_id: u16 = (space[0] & 0x0000ffff) as u16;
        (vendor_id != 0xffff)
            .then_some(Self {
                space
            })
    }

    pub fn reset(&self) {
        if self.header().class_code() == class::Code::UsbXhc {
            let xhc: Result<xhc::Registers, ()> = self.try_into();
            if let Ok(mut xhc) = xhc {
                xhc.reset();
            }
        }
    }
}

pub struct FunctionWithAddress<'a> {
    bus_number: u8,
    device_number: u8,
    function_number: u8,
    function: &'a Function,
}

impl<'a> FunctionWithAddress<'a> {
    pub fn function(&'a self) -> &'a Function {
        self.function
    }

    pub fn bus_number(&self) -> u8 {
        self.bus_number
    }

    pub fn device_number(&self) -> u8 {
        self.device_number
    }

    pub fn function_number(&self) -> u8 {
        self.function_number
    }

    fn new(bus_number: u8, device_number: u8, function_number: u8, function: &'a Function) -> Self {
        Self {
            bus_number,
            device_number,
            function_number,
            function,
        }
    }

    fn msi_capabilities(&self) -> capability::Headers<'_> {
        self.into()
    }
}

impl fmt::Debug for FunctionWithAddress<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct: fmt::DebugStruct = formatter.debug_struct("Function");
        let capabilities: capability::Headers = self.msi_capabilities();
        match self.function.header() {
            Header::Type0(type0) => {
                let vendor_id: u16 = type0.vendor_id;
                let device_id: u16 = type0.device_id;
                let command: command::Register = type0.command;
                let status: status::Register = type0.status;
                let revision_id: u8 = type0.revision_id;
                let class_code: class::Register = type0.class_code.clone();
                let cash_line_size: u8 = type0.cash_line_size;
                let latency_timer: u8 = type0.latency_timer;
                let header_type: header_type::Register = type0.header_type;
                let bist: bist::Register = type0.bist;
                let index2address: base::Index2Address = type0.index2address();
                let cardbus_cis_pointer: u32 = type0.cardbus_cis_pointer;
                let subsystem_vendor_id: u16 = type0.subsystem_vendor_id;
                let subsystem_id: u16 = type0.subsystem_id;
                let expansion_rom_base_address: expansion_rom_base_address::Register = type0.expansion_rom_base_address;
                let interrupt_line: u8 = type0.interrupt_line;
                let interrupt_pin: u8 = type0.interrupt_pin;
                let min_gnt: u8 = type0.min_gnt;
                let min_lat: u8 = type0.min_lat;
                let class_code: class::Code = class_code.clone().into();
                debug_struct
                    .field("vendor_id", &vendor_id)
                    .field("device_id", &device_id)
                    .field("command", &command)
                    .field("status", &status)
                    .field("revision_id", &revision_id)
                    .field("class_code", &class_code)
                    .field("cash_line_size", &cash_line_size)
                    .field("latency_timer", &latency_timer)
                    .field("header_type", &header_type)
                    .field("bist", &bist)
                    .field("index2address", &index2address)
                    .field("cardbus_cis_pointer", &cardbus_cis_pointer)
                    .field("subsystem_vendor_id", &subsystem_vendor_id)
                    .field("subsystem_id", &subsystem_id)
                    .field("expansion_rom_base_address", &expansion_rom_base_address)
                    .field("capabilities", &capabilities)
                    .field("interrupt_line", &interrupt_line)
                    .field("interrupt_pin", &interrupt_pin)
                    .field("min_gnt", &min_gnt)
                    .field("min_lat", &min_lat)
            },
            Header::Type1(type1) => {
                let vendor_id: u16 = type1.vendor_id;
                let device_id: u16 = type1.device_id;
                let command: command::Register = type1.command;
                let status: status::Register = type1.status;
                let revision_id: u8 = type1.revision_id;
                let class_code: class::Register = type1.class_code.clone();
                let cash_line_size: u8 = type1.cash_line_size;
                let latency_timer: u8 = type1.latency_timer;
                let header_type: header_type::Register = type1.header_type;
                let bist: bist::Register = type1.bist;
                let index2address: base::Index2Address = type1.index2address();
                let primary_bus_number: u8 = type1.primary_bus_number;
                let secondary_bus_number: u8 = type1.secondary_bus_number;
                let subordinate_bus_number: u8 = type1.subordinate_bus_number;
                let secondary_latency_timer: u8 = type1.secondary_latency_timer;
                let io_base: u8 = type1.io_base;
                let io_limit: u8 = type1.io_limit;
                let secondary_status: secondary_status::Register = type1.secondary_status;
                let mamory_base: u16 = type1.mamory_base;
                let memory_limit: u16 = type1.memory_limit;
                let prefetchable_memory_base: u16 = type1.prefetchable_memory_base;
                let prefetchable_memory_limit: u16 = type1.prefetchable_memory_limit;
                let prefetchable_memory_base_upper_32bits: u32 = type1.prefetchable_memory_base_upper_32bits;
                let prefetchable_memory_limit_upper_32bits: u32 = type1.prefetchable_memory_limit_upper_32bits;
                let io_base_upper_16bits: u16 = type1.io_base_upper_16bits;
                let io_limit_upper_16bits: u16 = type1.io_limit_upper_16bits;
                let expantion_rom_base_address: expansion_rom_base_address::Register = type1.expantion_rom_base_address;
                let interrupt_line: u8 = type1.interrupt_line;
                let interrupt_pin: u8 = type1.interrupt_pin;
                let bridge_control: bridge_control::Register = type1.bridge_control;
                let class_code: class::Code = class_code.clone().into();
                debug_struct
                    .field("vendor_id", &vendor_id)
                    .field("device_id", &device_id)
                    .field("command", &command)
                    .field("status", &status)
                    .field("revision_id", &revision_id)
                    .field("class_code", &class_code)
                    .field("cash_line_size", &cash_line_size)
                    .field("latency_timer", &latency_timer)
                    .field("header_type", &header_type)
                    .field("bist", &bist)
                    .field("index2address", &index2address)
                    .field("primary_bus_number", &primary_bus_number)
                    .field("secondary_bus_number", &secondary_bus_number)
                    .field("subordinate_bus_number", &subordinate_bus_number)
                    .field("secondary_latency_timer", &secondary_latency_timer)
                    .field("io_base", &io_base)
                    .field("io_limit", &io_limit)
                    .field("secondary_status", &secondary_status)
                    .field("mamory_base", &mamory_base)
                    .field("memory_limit", &memory_limit)
                    .field("prefetchable_memory_base", &prefetchable_memory_base)
                    .field("prefetchable_memory_limit", &prefetchable_memory_limit)
                    .field("prefetchable_memory_base_upper_32bits", &prefetchable_memory_base_upper_32bits)
                    .field("prefetchable_memory_limit_upper_32bits", &prefetchable_memory_limit_upper_32bits)
                    .field("io_base_upper_16bits", &io_base_upper_16bits)
                    .field("io_limit_upper_16bits", &io_limit_upper_16bits)
                    .field("capabilities", &capabilities)
                    .field("expantion_rom_base_address", &expantion_rom_base_address)
                    .field("interrupt_line", &interrupt_line)
                    .field("interrupt_pin", &interrupt_pin)
                    .field("bridge_control", &bridge_control)
            },
        };
        if self.function.header().class_code() == class::Code::UsbXhc {
            let xhc: Result<xhc::Registers, ()> = self.function.try_into();
            if let Ok(xhc) = xhc {
                debug_struct.field("xhc", &xhc);
            }
        }
        debug_struct.finish()
    }
}

pub enum Header<'a> {
    Type0(&'a Type0),
    Type1(&'a Type1),
}

impl Header<'_> {
    pub fn index2address(&self) -> base::Index2Address {
        match self {
            Self::Type0(type0) => type0.index2address(),
            Self::Type1(type1) => type1.index2address(),
        }
    }

    pub fn capabilities_pointer(&self) -> u8 {
        let capabilities_pointer: u8 = match self {
            Self::Type0(type0) => type0.capabilities_pointer,
            Self::Type1(type1) => type1.capabilities_pointer,
        };
        assert_eq!((capabilities_pointer as usize) % mem::size_of::<u32>(), 0);
        capabilities_pointer
    }

    pub fn class_code(&self) -> class::Code {
        match self {
            Self::Type0(type0) => type0.class_code.clone(),
            Self::Type1(type1) => type1.class_code.clone(),
        }.into()
    }

    pub fn header_type(&self) -> header_type::Register {
        match self {
            Self::Type0(type0) => type0.header_type,
            Self::Type1(type1) => type1.header_type,
        }
    }

    pub fn vendor_id(&self) -> u16 {
        match self {
            Self::Type0(type0) => type0.vendor_id,
            Self::Type1(type1) => type1.vendor_id,
        }
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
    fn index2address(&self) -> base::Index2Address {
        let base_address_registers: [u32; 6] = self.base_address_registers;
        base_address_registers
            .as_slice()
            .into()
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
    fn index2address(&self) -> base::Index2Address {
        let base_address_registers: [u32; 2] = self.base_address_registers;
        base_address_registers
            .as_slice()
            .into()
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

