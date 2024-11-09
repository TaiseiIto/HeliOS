use bitfield_struct::bitfield;

/// # General Capabilities and ID Register
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.4 General Capabilities and ID Register
#[bitfield(u64)]
pub struct Register {
    rev_id: u8,
    #[bits(5)]
    num_tim_cap: u8,
    count_size_cap: bool,
    __: bool,
    leg_rt_cap: bool,
    vendor_id: u16,
    counter_clk_period: u32,
}

impl Register {
    pub fn get_femtoseconds_per_increment(&self) -> u64 {
        self.counter_clk_period() as u64
    }

    pub fn number_of_timers(&self) -> usize {
        (self.num_tim_cap() as usize) + 1
    }
}

