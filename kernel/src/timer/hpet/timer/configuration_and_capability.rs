use bitfield_struct::bitfield;

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
        self.set_tn_int_type_cnf(InterruptType::Edge.into());
        self.set_tn_int_enb_cnf(true);
        self.set_tn_type_cnf(Type::Periodic.into());         // Periodic interrupt
        self.set_tn_32mode_cnf(false);      // 64 bit mode
        self.set_tn_int_route_cnf(irq);
        self.set_tn_fsb_en_cnf(false);      // I/O APIC
        irq
    }

    pub fn supports_periodic_interrupt(&self) -> bool {
        self.tn_per_int_cap()
    }
}

#[derive(Debug)]
pub struct Controller {
    interrupt_type: InterruptType,
    interrupt_enable: bool,
    timer_type: Type,
}

impl From<&Register> for Controller {
    fn from(register: &Register) -> Self {
        let interrupt_type: InterruptType = register.tn_int_type_cnf().into();
        let interrupt_enable: bool = register.tn_int_enb_cnf();
        let timer_type: Type = register.tn_type_cnf().into();
        Self {
            interrupt_type,
            interrupt_enable,
            timer_type,
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

