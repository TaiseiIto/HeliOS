pub mod comparator;
pub mod configuration_and_capability;
pub mod fsb_interrupt_route;

/// # Timer Registers
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.1 Register Overview Table 2 Memory-Mapped Registers
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Registers {
    configuration_and_capability: configuration_and_capability::Register,
    comparator_value: comparator::Register,
    fsb_interrupt_route: fsb_interrupt_route::Register,
    reserved0: u64,
}

