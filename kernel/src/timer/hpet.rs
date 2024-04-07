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
        let general_configuration: general_configuration::Register = self.general_configuration;
        if !general_configuration.is_counting() {
            self.general_configuration = general_configuration.start_counting();
        }
    }

    pub fn wait_femtoseconds(&self, femtoseconds: u64) {
        let mut current_counter_value: u64 = self.get_counter_value();
        let increments: u64 = femtoseconds / self.get_femtoseconds_per_increment();
        let minimum_counter_value: u64 = current_counter_value.wrapping_add(increments);
        let maximum_counter_value: u64 = minimum_counter_value.wrapping_add(1 << (u64::BITS - 1));
        while if minimum_counter_value < maximum_counter_value {
            !(minimum_counter_value..maximum_counter_value).contains(&current_counter_value)
        } else {
            (maximum_counter_value..minimum_counter_value).contains(&current_counter_value)
        } {
            current_counter_value = self.get_counter_value();
        }
    }

    pub fn wait_microseconds(&self, microseconds: u64) {
        self.wait_nanoseconds(1000 * microseconds)
    }

    pub fn wait_milliseconds(&self, milliseconds: u64) {
        self.wait_microseconds(1000 * milliseconds)
    }

    pub fn wait_nanoseconds(&self, nanoseconds: u64) {
        self.wait_picoseconds(1000 * nanoseconds)
    }

    pub fn wait_picoseconds(&self, picoseconds: u64) {
        self.wait_femtoseconds(1000 * picoseconds)
    }

    pub fn wait_seconds(&self, seconds: u64) {
        self.wait_milliseconds(1000 * seconds)
    }

    fn get_counter_value(&self) -> u64 {
        let main_counter_value: main_counter_value::Register = self.main_counter_value;
        main_counter_value.get()
    }

    fn get_femtoseconds_per_increment(&self) -> u64 {
        let general_capabilities_and_id: general_capabilities_and_id::Register = self.general_capabilities_and_id;
        general_capabilities_and_id.get_femtoseconds_per_increment()
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

