pub mod comparator;
pub mod configuration_and_capability;
pub mod fsb_interrupt_route;

use core::fmt;

/// # Timer Registers
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.1 Register Overview Table 2 Memory-Mapped Registers
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct Registers {
    #[allow(dead_code)]
    configuration_and_capability: configuration_and_capability::Register,
    #[allow(dead_code)]
    comparator: comparator::Register,
    #[allow(dead_code)]
    fsb_interrupt_route: fsb_interrupt_route::Register,
    #[allow(dead_code)]
    __: u64,
}

impl Registers {
    pub fn disable_periodic_interrupt(&mut self) {
        let configuration_and_capability: configuration_and_capability::Register = self.configuration_and_capability.disable_periodic_interrupt();
        self.configuration_and_capability = configuration_and_capability;
    }

    pub fn enable_periodic_interrupt(&mut self, period: u64) -> u8 {
        let configuration_and_capability: configuration_and_capability::Register = self.configuration_and_capability.enable_periodic_interrupt();
        self.configuration_and_capability = configuration_and_capability;
        self.comparator = comparator::Register::create(period);
        configuration_and_capability.irq()
    }

    pub fn is_enable(&self) -> bool {
        let configuration_and_capability: configuration_and_capability::Register = self.configuration_and_capability;
        configuration_and_capability.is_enable()
    }

    pub fn supports_periodic_interrupt(&self) -> bool {
        let configuration_and_capability: configuration_and_capability::Register = self.configuration_and_capability;
        configuration_and_capability.supports_periodic_interrupt()
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let configuration_and_capability: configuration_and_capability::Register = self.configuration_and_capability;
        let configuration_and_capability: configuration_and_capability::Controller = (&configuration_and_capability).into();
        let comparator: comparator::Register = self.comparator;
        let fsb_interrupt_route: fsb_interrupt_route::Register = self.fsb_interrupt_route;
        formatter
            .debug_struct("Registers")
            .field("configuration_and_capability", &configuration_and_capability)
            .field("comparator", &comparator)
            .field("fsb_interrupt_route", &fsb_interrupt_route)
            .finish()
    }
}

