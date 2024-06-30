//! # Advanced Programmable Interrupt Controller (APIC)
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A Chapter 11 Advanced Programmable Interrupt Controller (APIC)

pub mod io;
pub mod local;

/// # Trigger Mode
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.1 Figure 11-8. Local Vector Table (LVT)
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.1 Figure 11-12. Interrupt Command Register (ICR)
#[derive(Debug)]
pub enum DeliveryMode {
    Fixed,
    LowestPriority,
    Smi,
    Nmi,
    Init,
    StartUp,
    ExtInt,
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
            0b111 => Ok(Self::ExtInt),
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
            DeliveryMode::ExtInt => 0b111,
        }
    }
}

/// # Trigger Mode
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.1 Figure 11-8. Local Vector Table (LVT)
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.1 Figure 11-12. Interrupt Command Register (ICR)
#[derive(Debug, Eq, PartialEq)]
pub enum DeliveryStatus {
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

/// # Trigger Mode
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.1 Figure 11-8. Local Vector Table (LVT)
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.1 Figure 11-12. Interrupt Command Register (ICR)
#[derive(Debug)]
pub enum TriggerMode {
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


