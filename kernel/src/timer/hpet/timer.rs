/// # Timer Registers
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.1 Register Overview Table 2 Memory-Mapped Registers
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Registers {
    configuration_and_capability: u64,
    comparator_value: u64,
    fsb_interrupt_route: u64,
    reserved0: u64,
}

