pub mod comparator;
pub mod configuration_and_capability;
pub mod fsb_interrupt_route;

/// # Timer Registers
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.1 Register Overview Table 2 Memory-Mapped Registers
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Registers {
    #[allow(dead_code)]
    configuration_and_capability: configuration_and_capability::Register,
    #[allow(dead_code)]
    comparator_value: comparator::Register,
    #[allow(dead_code)]
    fsb_interrupt_route: fsb_interrupt_route::Register,
    #[allow(dead_code)]
    reserved0: u64,
}

impl Registers {
    pub fn is_enable(&self) -> bool {
        let configuration_and_capability: configuration_and_capability::Register = self.configuration_and_capability;
        configuration_and_capability.is_enable()
    }

    pub fn set_periodic_interrupt(&mut self, comparator: u64) -> u8 {
        let mut configuration_and_capability: configuration_and_capability::Register = self.configuration_and_capability;
        let irq: u8 = configuration_and_capability.set_periodic_interrupt();
        self.configuration_and_capability = configuration_and_capability;
        self.comparator_value = comparator::Register::create(comparator);
        irq
    }

    pub fn supports_periodic_interrupt(&self) -> bool {
        let configuration_and_capability: configuration_and_capability::Register = self.configuration_and_capability;
        configuration_and_capability.supports_periodic_interrupt()
    }
}

