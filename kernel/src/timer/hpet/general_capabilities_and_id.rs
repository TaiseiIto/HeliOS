use bitfield_struct::bitfield;

/// # General Capabilities and ID Register
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.4 General Capabilities and ID Register
#[bitfield(u64)]
pub struct Register {
    ref_id: u8,
    #[bits(5)]
    num_tim_cap: u8,
    count_size_cap: bool,
    #[bits(access = RO)]
    reserved0: bool,
    leg_route_cap: bool,
    vendor_id: u16,
    counter_clk_period: u32,
}

