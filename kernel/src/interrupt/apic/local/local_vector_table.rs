use {
    bitfield_struct::bitfield,
    core::fmt,
    super::super::{
        DeliveryMode,
        TriggerMode,
    },
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    #[allow(dead_code)]
    reserved0: [u32; 3],
}

impl FatRegister {
    pub fn disable_periodic_interrupt(&mut self) {
        let register: Register = self.register.with_mask(Mask::InhibitInterrupt.into());
        *self.register_mut() = register.into();
    }

    pub fn set(&mut self, vector: u8, delivery_mode: DeliveryMode, interrupt_input_pin_polarity: InterruptInputPinPolarity, trigger_mode: TriggerMode, mask: Mask, timer_mode: TimerMode) {
        let register: Register = self.register;
        let register: Register = register.overwrite(vector, delivery_mode, interrupt_input_pin_polarity, trigger_mode, mask, timer_mode);
        *self.register_mut() = register.into();
    }

    fn register_mut(&mut self) -> &mut u32 {
        let address: *mut Self = self as *mut Self;
        let address: *mut u32 = address as *mut u32;
        unsafe {
            &mut *address
        }
    }
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

impl Register {
    fn overwrite(self, vector: u8, delivery_mode: DeliveryMode, interrupt_input_pin_polarity: InterruptInputPinPolarity, trigger_mode: TriggerMode, mask: Mask, timer_mode: TimerMode) -> Self {
        self.with_vector(vector)
            .with_delivery_mode(delivery_mode.into())
            .with_interrupt_input_pin_polarity(interrupt_input_pin_polarity.into())
            .with_trigger_mode(trigger_mode.into())
            .with_mask(mask.into())
            .with_timer_mode(timer_mode.into())
    }
}

pub enum InterruptInputPinPolarity {
    ActiveHigh,
    ActiveLow,
}

impl From<bool> for InterruptInputPinPolarity {
    fn from(interrupt_input_pin_polarity: bool) -> Self {
        if interrupt_input_pin_polarity {
            Self::ActiveLow
        } else {
            Self::ActiveHigh
        }
    }
}

impl From<InterruptInputPinPolarity> for bool {
    fn from(interrupt_input_pin_polarity: InterruptInputPinPolarity) -> Self {
        match interrupt_input_pin_polarity {
            InterruptInputPinPolarity::ActiveHigh => false,
            InterruptInputPinPolarity::ActiveLow => true,
        }
    }
}

pub enum Mask {
    EnableInterrupt,
    InhibitInterrupt,
}

impl From<bool> for Mask {
    fn from(mask: bool) -> Self {
        if mask {
            Self::InhibitInterrupt
        } else {
            Self::EnableInterrupt
        }
    }
}

impl From<Mask> for bool {
    fn from(mask: Mask) -> Self {
        match mask {
            Mask::EnableInterrupt => false,
            Mask::InhibitInterrupt => true,
        }
    }
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

