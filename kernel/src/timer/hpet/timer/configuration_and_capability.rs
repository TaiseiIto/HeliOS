use {
    alloc::collections::BTreeSet,
    bitfield_struct::bitfield,
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

    pub fn irq(&self) -> u8 {
        self.tn_int_route_cnf()
    }

    pub fn set_periodic_interrupt(self) -> Self {
        assert!(self.supports_periodic_interrupt());
        let tn_int_route_cap: u32 = self.tn_int_route_cap();
        let irq: u8 = 2;
        assert!(tn_int_route_cap & (1 << irq) != 0);
        let interrupt_destination = InterruptDestination::IoApic {
            irq,
        };
        self.with_tn_int_type_cnf(InterruptType::Edge.into())
            .with_tn_int_enb_cnf(true)
            .with_tn_type_cnf(Type::Periodic.into())
            .with_tn_val_set_cnf(true)
            .with_tn_32mode_cnf(Mode::Bit64.into())
            .with_interrupt_destination(&interrupt_destination)
    }

    pub fn supports_periodic_interrupt(&self) -> bool {
        self.tn_per_int_cap()
    }

    fn with_interrupt_destination(self, interrupt_destination: &InterruptDestination) -> Self {
        match interrupt_destination {
            InterruptDestination::Fsb => {
                self.with_tn_fsb_en_cnf(true)
            },
            InterruptDestination::IoApic {
                irq,
            } => {
                self.with_tn_fsb_en_cnf(false)
                    .with_tn_int_route_cnf(*irq)
            },
        }
    }
}

#[derive(Debug)]
pub struct Controller {
    periodic_interrupt_capable: bool,
    size: Size,
    resetting_comparator_value: bool,
    available_irq_numbers: BTreeSet<u8>,
    interrupt: Interrupt,
}

impl From<&Register> for Controller {
    fn from(register: &Register) -> Self {
        let periodic_interrupt_capable: bool = register.tn_per_int_cap();
        let size: Size = register.tn_size_cap().into();
        let resetting_comparator_value: bool = register.tn_val_set_cnf();
        let available_irq_numbers: BTreeSet<u8> = (0..u32::BITS)
            .filter_map(|irq| (register.tn_int_route_cap() & (1 << irq) != 0).then(|| irq as u8))
            .collect();
        let interrupt: Interrupt = register.into();
        Self {
            periodic_interrupt_capable,
            size,
            resetting_comparator_value,
            available_irq_numbers,
            interrupt,
        }
    }
}

#[derive(Debug)]
enum Interrupt {
    Disable,
    Enable {
        interrupt_type: InterruptType,
        timer_type: Type,
        mode: Mode,
        interrupt_destination: InterruptDestination,
    },
}

impl From<&Register> for Interrupt {
    fn from(register: &Register) -> Self {
        if register.tn_int_enb_cnf() {
            let interrupt_type: InterruptType = register.tn_int_type_cnf().into();
            let timer_type: Type = register.tn_type_cnf().into();
            let mode: Mode = register.tn_32mode_cnf().into();
            let interrupt_destination: InterruptDestination = register.into();
            Self::Enable {
                interrupt_type,
                timer_type,
                mode,
                interrupt_destination,
            }
        } else {
            Self::Disable
        }
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

