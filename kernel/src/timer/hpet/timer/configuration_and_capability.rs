use {
    bitfield_struct::bitfield,
    core::fmt,
};

/// # Timer N Configuration and Capabilities Register
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.8 Timer N Configuration and Capabilities Register
#[bitfield(u64)]
pub struct Register {
    #[bits(access = RO)]
    reserved0: bool,
    tn_int_type_cnf: bool,
    tn_int_enb_cnf: bool,
    tn_type_cnf: bool,
    #[bits(access = RO)]
    tn_per_int_cap: bool,
    #[bits(access = RO)]
    tn_size_cap: bool,
    tn_val_set_cnf: bool,
    #[bits(access = RO)]
    reserved1: bool,
    tn_32mode_cnf: bool,
    #[bits(5)]
    tn_int_route_cnf: u8,
    tn_fsb_en_cnf: bool,
    #[bits(access = RO)]
    tn_fsb_int_del_cap: bool,
    #[bits(access = RO)]
    reserved2: u16,
    #[bits(access = RO)]
    tn_int_route_cap: u32,
}

impl Register {
    pub fn is_enable(&self) -> bool {
        self.tn_int_enb_cnf()
    }

    pub fn set_periodic_interrupt(&mut self) -> u8 {
        assert!(self.supports_periodic_interrupt());
        let tn_int_route_cap: u32 = self.tn_int_route_cap();
        let irq: u8 = (0..u32::BITS)
            .find(|irq| tn_int_route_cap & (1 << irq) != 0)
            .unwrap() as u8;
        let interrupt_destination = InterruptDestination::IoApic {
            irq,
        };
        self.set_tn_int_type_cnf(InterruptType::Edge.into());
        self.set_tn_int_enb_cnf(true);
        self.set_tn_type_cnf(Type::Periodic.into());
        self.set_tn_32mode_cnf(Mode::Bit64.into());
        self.set_interrupt_destination(&interrupt_destination);
        irq
    }

    pub fn supports_periodic_interrupt(&self) -> bool {
        self.tn_per_int_cap()
    }

    fn set_interrupt_destination(&mut self, interrupt_destination: &InterruptDestination) {
        match interrupt_destination {
            InterruptDestination::Fsb => {
                self.set_tn_fsb_en_cnf(true);
            },
            InterruptDestination::IoApic {
                irq,
            } => {
                self.set_tn_fsb_en_cnf(false);
                self.set_tn_int_route_cnf(*irq);
            },
        }
    }
}

pub struct Controller {
    interrupt_type: InterruptType,
    interrupt_enable: bool,
    timer_type: Type,
    periodic_interrupt_capable: bool,
    size: Size,
    resetting_comparator_value: bool,
    mode: Mode,
    interrupt_destination: InterruptDestination,
}

impl From<&Register> for Controller {
    fn from(register: &Register) -> Self {
        let interrupt_type: InterruptType = register.tn_int_type_cnf().into();
        let interrupt_enable: bool = register.tn_int_enb_cnf();
        let timer_type: Type = register.tn_type_cnf().into();
        let periodic_interrupt_capable: bool = register.tn_per_int_cap();
        let size: Size = register.tn_size_cap().into();
        let resetting_comparator_value: bool = register.tn_val_set_cnf();
        let mode: Mode = register.tn_32mode_cnf().into();
        let interrupt_destination: InterruptDestination = register.into();
        Self {
            interrupt_type,
            interrupt_enable,
            timer_type,
            periodic_interrupt_capable,
            size,
            resetting_comparator_value,
            mode,
            interrupt_destination,
        }
    }
}

impl fmt::Debug for Controller {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct: fmt::DebugStruct = formatter.debug_struct("Controller");
        debug_struct.field("interrupt_type", &self.interrupt_type);
        debug_struct.field("interrupt_enable", &self.interrupt_enable);
        if self.periodic_interrupt_capable {
            debug_struct.field("timer_type", &self.timer_type);
        }
        debug_struct.field("periodic_interrupt_capable", &self.periodic_interrupt_capable);
        debug_struct.field("size", &self.size);
        debug_struct.field("resetting_comparator_value", &self.resetting_comparator_value);
        if matches!(self.size, Size::Bit64) {
            debug_struct.field("mode", &self.mode);
        }
        debug_struct.field("interrupt_destination", &self.interrupt_destination);
        debug_struct.finish()
    }
}

#[derive(Debug)]
enum InterruptType {
    Edge,
    Level,
}

impl From<bool> for InterruptType {
    fn from(interrupt_type: bool) -> Self {
        if interrupt_type {
            Self::Level
        } else {
            Self::Edge
        }
    }
}

impl From<InterruptType> for bool {
    fn from(interrupt_type: InterruptType) -> Self {
        match interrupt_type {
            InterruptType::Edge => false,
            InterruptType::Level => true,
        }
    }
}

#[derive(Debug)]
enum Type {
    OneShot,
    Periodic,
}

impl From<bool> for Type {
    fn from(timer_type: bool) -> Self {
        if timer_type {
            Self::Periodic
        } else {
            Self::OneShot
        }
    }
}

impl From<Type> for bool {
    fn from(timer_type: Type) -> Self {
        match timer_type {
            Type::OneShot => false,
            Type::Periodic => true,
        }
    }
}

#[derive(Debug)]
enum Size {
    Bit32,
    Bit64,
}

impl From<bool> for Size {
    fn from(size: bool) -> Self {
        if size {
            Self::Bit64
        } else {
            Self::Bit32
        }
    }
}

impl From<Size> for bool {
    fn from(size: Size) -> Self {
        match size {
            Size::Bit32 => false,
            Size::Bit64 => true,
        }
    }
}

#[derive(Debug)]
enum Mode {
    Bit32,
    Bit64,
}

impl From<bool> for Mode {
    fn from(mode: bool) -> Self {
        if mode {
            Self::Bit32
        } else {
            Self::Bit64
        }
    }
}

impl From<Mode> for bool {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Bit32 => true,
            Mode::Bit64 => false,
        }
    }
}

#[derive(Debug)]
enum InterruptDestination {
    Fsb,
    IoApic {
        irq: u8,
    },
}

impl From<&Register> for InterruptDestination {
    fn from(register: &Register) -> Self {
        if register.tn_fsb_int_del_cap() && register.tn_fsb_en_cnf() {
            Self::Fsb
        } else {
            let irq: u8 = register.tn_int_route_cnf();
            Self::IoApic {
                irq,
            }
        }
    }
}

