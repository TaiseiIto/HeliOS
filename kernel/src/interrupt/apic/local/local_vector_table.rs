use {
    bitfield_struct::bitfield,
    core::fmt,
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    #[allow(dead_code)]
    reserved0: [u32; 3],
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let vector: u8 = register.vector();
        let delivery_mode: u8 = register.delivery_mode();
        let delivery_status: bool = register.delivery_status();
        let interrupt_input_pin_polarity: bool = register.interrupt_input_pin_polarity();
        let remote_irr: bool = register.remote_irr();
        let trigger_mode: bool = register.trigger_mode();
        let mask: bool = register.mask();
        let timer_mode: u8 = register.timer_mode();
        formatter
            .debug_struct("Register")
            .field("vector", &vector)
            .field("delivery_mode", &delivery_mode)
            .field("delivery_status", &delivery_status)
            .field("interrupt_input_pin_polarity", &interrupt_input_pin_polarity)
            .field("remote_irr", &remote_irr)
            .field("trigger_mode", &trigger_mode)
            .field("mask", &mask)
            .field("timer_mode", &timer_mode)
            .finish()
    }
}

/// # Local Vector Table (LVT)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.1 Figure 11-8. Local Vector Table (LVT)
#[bitfield(u32)]
struct Register {
    vector: u8,
    #[bits(3)]
    delivery_mode: u8,
    #[bits(access = RO)]
    reserved0: bool,
    delivery_status: bool,
    interrupt_input_pin_polarity: bool,
    remote_irr: bool,
    trigger_mode: bool,
    mask: bool,
    #[bits(2)]
    timer_mode: u8,
    #[bits(13, access = RO)]
    reserved1: u16,
}

pub enum TimerMode {
    OneShot,
    Periodic,
    TscDeadline,
}

impl From<u8> for TimerMode {
    fn from(timer_mode: u8) -> Self {
        match timer_mode {
            0b00 => Self::OneShot,
            0b01 => Self::Periodic,
            0b10 => Self::TscDeadline,
            _ => unreachable!(),
        }
    }
}

impl From<TimerMode> for u8 {
    fn from(timer_mode: TimerMode) -> Self {
        match timer_mode {
            TimerMode::OneShot => 0b00,
            TimerMode::Periodic => 0b01,
            TimerMode::TscDeadline => 0b10,
        }
    }
}

