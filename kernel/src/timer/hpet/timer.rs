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

