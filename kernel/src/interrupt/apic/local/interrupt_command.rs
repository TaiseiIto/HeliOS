use {
    bitfield_struct::bitfield,
    core::fmt,
    crate::{
        com2_print,
        com2_println,
        memory,
        x64,
    },
};

/// # Interrupt Command Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.1 Figure 11-12. Interrupt Command Register (ICR)
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Register {
    low: FatLow,
    high: FatHigh,
}

impl Register {
    pub fn assert_init(&mut self, processor_local_apic_id: u8) {
        let address: *mut FatHigh = &mut self.high as *mut FatHigh;
        let address: usize = address as usize;
        com2_println!("interrupt command high address = {:#x?}", address);
        self.high = self.high.select_processor(processor_local_apic_id);
        let address: *mut FatLow = &mut self.low as *mut FatLow;
        let address: usize = address as usize;
        com2_println!("interrupt command low address = {:#x?}", address);
        self.low = self.low.assert_init();
    }

    pub fn deassert_init(&mut self, processor_local_apic_id: u8) {
        let address: *mut FatHigh = &mut self.high as *mut FatHigh;
        let address: usize = address as usize;
        com2_println!("interrupt command high address = {:#x?}", address);
        self.high = self.high.select_processor(processor_local_apic_id);
        let address: *mut FatLow = &mut self.low as *mut FatLow;
        let address: usize = address as usize;
        com2_println!("interrupt command low address = {:#x?}", address);
        self.low = self.low.deassert_init();
    }

    pub fn send_sipi(&mut self, processor_local_apic_id: u8, entry_point: usize) {
        let address: *mut FatHigh = &mut self.high as *mut FatHigh;
        let address: usize = address as usize;
        com2_println!("interrupt command high address = {:#x?}", address);
        self.high = self.high.select_processor(processor_local_apic_id);
        let address: *mut FatLow = &mut self.low as *mut FatLow;
        let address: usize = address as usize;
        com2_println!("interrupt command low address = {:#x?}", address);
        self.low = self.low.send_sipi(entry_point);
    }

    pub fn wait_to_send(&self) {
        while self.is_sending() {
            x64::pause();
        }
    }

    fn is_sending(&self) -> bool {
        let low: FatLow = self.low;
        low.is_sending()
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct FatLow {
    register: Low,
    reserved0: [u32; 3],
}

impl FatLow {
    fn assert_init(self) -> Self {
        let Self {
            register,
            reserved0,
        } = self;
        let register: Low = register.assert_init();
        com2_println!("interrupt command low = {:#x?}", register.0);
        Self {
            register,
            reserved0,
        }
    }

    fn deassert_init(self) -> Self {
        let Self {
            register,
            reserved0,
        } = self;
        let register: Low = register.deassert_init();
        com2_println!("interrupt command low = {:#x?}", register.0);
        Self {
            register,
            reserved0,
        }
    }

    fn is_sending(&self) -> bool {
        let register: Low = self.register;
        register.is_sending()
    }

    fn send_sipi(self, entry_point: usize) -> Self {
        let Self {
            register,
            reserved0,
        } = self;
        let register: Low = register.send_sipi(entry_point);
        com2_println!("interrupt command low = {:#x?}", register.0);
        Self {
            register,
            reserved0,
        }
    }
}

impl fmt::Debug for FatLow {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Low = self.register;
        let vector: u8 = register.vector();
        let delivery_mode: Result<DeliveryMode, ()> = register.delivery_mode().try_into();
        let delivery_mode: DeliveryMode = delivery_mode.unwrap();
        let destination_mode: DestinationMode = register.destination_mode().into();
        let delivery_status: DeliveryStatus = register.delivery_status().into();
        let level: Level = register.level().into();
        let trigger_mode: bool = register.trigger_mode();
        let destination_shorthand: u8 = register.destination_shorthand();
        formatter
            .debug_struct("Low")
            .field("vector", &vector)
            .field("delivery_mode", &delivery_mode)
            .field("destination_mode", &destination_mode)
            .field("delivery_status", &delivery_status)
            .field("level", &level)
            .field("trigger_mode", &trigger_mode)
            .field("destination_shorthand", &destination_shorthand)
            .finish()
    }
}

#[bitfield(u32)]
struct Low {
    vector: u8,
    #[bits(3)]
    delivery_mode: u8,
    destination_mode: bool,
    delivery_status: bool,
    #[bits(access = RO)]
    reserved0: bool,
    level: bool,
    trigger_mode: bool,
    #[bits(2, access = RO)]
    reserved1: u8,
    #[bits(2)]
    destination_shorthand: u8,
    #[bits(12, access = RO)]
    reserved2: u16,
}

impl Low {
    fn assert_init(self) -> Self {
        self.with_vector(0)
            .with_delivery_mode(DeliveryMode::Init.into())
            .with_destination_mode(DestinationMode::Physical.into())
            .with_level(Level::Assert.into())
            .with_trigger_mode(TriggerMode::Level.into())
            .with_destination_shorthand(DestinationShorthand::NoShorthand.into())
    }

    fn deassert_init(self) -> Self {
        self.with_vector(0)
            .with_delivery_mode(DeliveryMode::Init.into())
            .with_destination_mode(DestinationMode::Physical.into())
            .with_level(Level::Deassert.into())
            .with_trigger_mode(TriggerMode::Level.into())
            .with_destination_shorthand(DestinationShorthand::NoShorthand.into())
    }

    fn is_sending(&self) -> bool {
        let delivery_status: DeliveryStatus = self.delivery_status().into();
        delivery_status == DeliveryStatus::SendPending
    }

    fn send_sipi(self, entry_point: usize) -> Self {
        self.with_vector((entry_point / memory::page::SIZE) as u8)
            .with_delivery_mode(DeliveryMode::StartUp.into())
            .with_destination_shorthand(DestinationShorthand::NoShorthand.into())
    }
}

#[derive(Debug)]
enum DeliveryMode {
    Fixed,
    LowestPriority,
    Smi,
    Nmi,
    Init,
    StartUp,
}

impl TryFrom<u8> for DeliveryMode {
    type Error = ();

    fn try_from(delivery_mode: u8) -> Result<Self, Self::Error> {
        match delivery_mode {
            0b000 => Ok(Self::Fixed),
            0b001 => Ok(Self::LowestPriority),
            0b010 => Ok(Self::Smi),
            0b100 => Ok(Self::Nmi),
            0b101 => Ok(Self::Init),
            0b110 => Ok(Self::StartUp),
            _ => Err(()),
        }
    }
}

impl From<DeliveryMode> for u8 {
    fn from(delivery_mode: DeliveryMode) -> Self {
        match delivery_mode {
            DeliveryMode::Fixed => 0b000,
            DeliveryMode::LowestPriority => 0b001,
            DeliveryMode::Smi => 0b010,
            DeliveryMode::Nmi => 0b100,
            DeliveryMode::Init => 0b101,
            DeliveryMode::StartUp => 0b110,
        }
    }
}

#[derive(Debug)]
enum DestinationMode {
    Physical,
    Logical,
}

impl From<bool> for DestinationMode {
    fn from(destination_mode: bool) -> Self {
        match destination_mode {
            false => Self::Physical,
            true => Self::Logical,
        }
    }
}

impl From<DestinationMode> for bool {
    fn from(destination_mode: DestinationMode) -> Self {
        match destination_mode {
            DestinationMode::Physical => false,
            DestinationMode::Logical => true,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum DeliveryStatus {
    Idle,
    SendPending,
}

impl From<bool> for DeliveryStatus {
    fn from(delivery_status: bool) -> Self {
        match delivery_status {
            false => Self::Idle,
            true => Self::SendPending,
        }
    }
}

impl From<DeliveryStatus> for bool {
    fn from(delivery_status: DeliveryStatus) -> Self {
        match delivery_status {
            DeliveryStatus::Idle => false,
            DeliveryStatus::SendPending => true,
        }
    }
}

#[derive(Debug)]
enum Level {
    Deassert,
    Assert,
}

impl From<bool> for Level {
    fn from(level: bool) -> Self {
        match level {
            false => Self::Deassert,
            true => Self::Assert,
        }
    }
}

impl From<Level> for bool {
    fn from(level: Level) -> Self {
        match level {
            Level::Deassert => false,
            Level::Assert => true,
        }
    }
}

#[derive(Debug)]
enum TriggerMode {
    Edge,
    Level,
}

impl From<bool> for TriggerMode {
    fn from(trigger_mode: bool) -> Self {
        match trigger_mode {
            false => Self::Edge,
            true => Self::Level,
        }
    }
}

impl From<TriggerMode> for bool {
    fn from(trigger_mode: TriggerMode) -> Self {
        match trigger_mode {
            TriggerMode::Edge => false,
            TriggerMode::Level => true,
        }
    }
}

#[derive(Debug)]
enum DestinationShorthand {
    NoShorthand,
    SelfShorthand,
    AllIncludingSelf,
    AllExcludingSelf,
}

impl TryFrom<u8> for DestinationShorthand {
    type Error = ();

    fn try_from(destination_shorthand: u8) -> Result<Self, Self::Error> {
        match destination_shorthand {
            0b00 => Ok(Self::NoShorthand),
            0b01 => Ok(Self::SelfShorthand),
            0b10 => Ok(Self::AllIncludingSelf),
            0b11 => Ok(Self::AllExcludingSelf),
            _ => Err(()),
        }
    }
}

impl From<DestinationShorthand> for u8 {
    fn from(destination_shorthand: DestinationShorthand) -> Self {
        match destination_shorthand {
            DestinationShorthand::NoShorthand => 0b00,
            DestinationShorthand::SelfShorthand => 0b01,
            DestinationShorthand::AllIncludingSelf => 0b10,
            DestinationShorthand::AllExcludingSelf => 0b11,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct FatHigh {
    register: High,
    reserved0: [u32; 3],
}

impl FatHigh {
    fn select_processor(self, processor_local_apic_id: u8) -> Self {
        let Self {
            register,
            reserved0,
        } = self;
        let register: High = register.select_processor(processor_local_apic_id);
        com2_println!("interrupt command high = {:#x?}", register.0);
        Self {
            register,
            reserved0,
        }
    }
}

impl fmt::Debug for FatHigh {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: High = self.register;
        let destination_field: u8 = register.destination_field();
        formatter
            .debug_struct("High")
            .field("destination_field", &destination_field)
            .finish()
    }
}

#[bitfield(u32)]
struct High {
    #[bits(24, access = RO)]
    reserved0: u32,
    destination_field: u8,
}

impl High {
    fn select_processor(self, processor_local_apic_id: u8) -> Self {
        self.with_destination_field(processor_local_apic_id)
    }
}

