use core::fmt;

/// # General Interrupt Status Register
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.6 General Interrupt Status Register
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct Register {
    int_sts: u32,
    reserved0: u32
}

impl fmt::Debug for Register {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries((0..u32::BITS)
                .map(|bit| (bit, (self.int_sts & (1 << bit)) != 0)))
            .finish()
    }
}

