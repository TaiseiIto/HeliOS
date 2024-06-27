use {
    bitfield_struct::bitfield,
    crate::x64,
};

/// # Control Register
/// ## References
/// * [Programmable Interval Timer](https://wiki.osdev.org/Programmable_Interval_Timer)
/// * [8254 PROGRAMMABLE INTERVAL TIMER](https://www.scs.stanford.edu/10wi-cs140/pintos/specs/8254.pdf)
#[bitfield(u8)]
pub struct Register {
    bcd: bool,
    #[bits(3)]
    mode: u8,
    #[bits(2)]
    access: u8,
    #[bits(2)]
    selector: u8,
}

impl Register {
    const PORT: u16 = 0x43;

    pub fn create(bcd: bool, mode: Mode, access: Access, selector: Selector) -> Self {
        Self::new()
            .with_bcd(bcd)
            .with_mode(mode.into())
            .with_access(access.into())
            .with_selector(selector.into())
    }

    pub fn set(self) {
        x64::port::outb(Self::PORT, self.into());
    }
}

pub enum Mode {
    InterruptOnTerminalCount,
    HardwareRetriggerableOneShot,
    RateGenerator,
    SquareWaveMode,
    SoftwareTriggeredStrobe,
    HardwareTriggeredStrobe,
}

impl From<u8> for Mode {
    fn from(mode: u8) -> Self {
        match mode {
            0 => Self::InterruptOnTerminalCount,
            1 => Self::HardwareRetriggerableOneShot,
            2 => Self::RateGenerator,
            3 => Self::SquareWaveMode,
            4 => Self::SoftwareTriggeredStrobe,
            5 => Self::HardwareTriggeredStrobe,
            _ => unreachable!(),
        }
    }
}

impl From<Mode> for u8 {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::InterruptOnTerminalCount => 0,
            Mode::HardwareRetriggerableOneShot => 1,
            Mode::RateGenerator => 2,
            Mode::SquareWaveMode => 3,
            Mode::SoftwareTriggeredStrobe => 4,
            Mode::HardwareTriggeredStrobe => 5,
        }
    }
}

pub enum Access {
    Latch,
    Low,
    High,
    LowAndHigh,
}

impl From<u8> for Access {
    fn from(access: u8) -> Self {
        match access {
            0 => Self::Latch,
            1 => Self::Low,
            2 => Self::High,
            3 => Self::LowAndHigh,
            _ => unreachable!(),
        }
    }
}

impl From<Access> for u8 {
    fn from(access: Access) -> Self {
        match access {
            Access::Latch => 0,
            Access::Low => 1,
            Access::High => 2,
            Access::LowAndHigh => 3,
        }
    }
}

pub enum Selector {
    Counter(u8),
    ReadBack,
}

impl From<u8> for Selector {
    fn from(selector: u8) -> Self {
        match selector {
            counter @ (0 | 1 | 2) => Self::Counter(counter),
            3 => Self::ReadBack,
            _ => unreachable!(),
        }
    }
}

impl From<Selector> for u8 {
    fn from(selector: Selector) -> Self {
        match selector {
            Selector::Counter(counter) => {
                assert!((0..=2).contains(&counter));
                counter
            },
            Selector::ReadBack => 3,
        }
    }
}

