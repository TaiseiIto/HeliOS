use bitfield_struct::bitfield;

/// # General Configuration Register
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.5 General Configuration Register
#[bitfield(u64)]
pub struct Register {
    enable_cnf: bool,
    leg_rt_cnf: bool,
    #[bits(62, access = RO)]
    reserved0: u64,
}

impl Register {
    pub fn start_counting(self) -> Self {
        self.with_enable_cnf(true)
    }
}

