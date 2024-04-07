pub mod general_capabilities_and_id;
pub mod general_configuration;
pub mod general_interrupt_status;
pub mod main_counter_value;
pub mod timer;

use core::fmt;

/// # Register Overview
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.1 Register Overview Table 2 Memory-Mapped Registers
#[repr(packed)]
pub struct Registers {
    general_capabilities_and_id: general_capabilities_and_id::Register,
    reserved0: u64,
    general_configuration: general_configuration::Register,
    reserved1: u64,
    general_interrupt_status: general_interrupt_status::Register,
    reserved2: [u64; 0x19],
    main_counter_value: main_counter_value::Register,
    reserved3: u64,
    timer: [timer::Registers; 0x18],
}

impl Registers {
    pub fn start_counting(&mut self) {
        self.general_configuration = self.general_configuration.start_counting();
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let general_capabilities_and_id: general_capabilities_and_id::Register = self.general_capabilities_and_id;
        let general_configuration: general_configuration::Register = self.general_configuration;
        let general_interrupt_status: general_interrupt_status::Register = self.general_interrupt_status;
        let main_counter_value: main_counter_value::Register = self.main_counter_value;
        let timer: [timer::Registers; 0x18] = self.timer;
        formatter
            .debug_struct("Registers")
            .field("general_capabilities_and_id", &general_capabilities_and_id)
            .field("general_configuration", &general_configuration)
            .field("general_interrupt_status", &general_interrupt_status)
            .field("main_counter_value", &main_counter_value)
            .field("timer", &timer)
            .finish()
    }
}

