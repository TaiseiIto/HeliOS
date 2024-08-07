use bitfield_struct::bitfield;

/// # Main Counter Value Register
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.6 General Interrupt Status Register
#[bitfield(u64)]
pub struct Register {
    counter_value: u64,
}

impl Register {
    pub fn create(counter_value: u64) -> Self {
        Self::default().with_counter_value(counter_value)
    }

    pub fn get(&self) -> u64 {
        self.counter_value()
    }
}

