/// # Register Overview
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.1 Register Overview Table 2 Memory-Mapped Registers
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    general_capabilities_and_id: u64,
    reserved0: u64,
    general_configuration: u64,
    reserved1: u64,
    general_interrupt_status: u64,
    reserved2: [u64; 0x19],
    main_counter_value: u64,
    reserved3: u64,
}

