use bitfield_struct::bitfield;

/// # Timer N FSB Interrupt Route Register
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.10 Timer N FSB Interrupt Route Register
#[bitfield(u64)]
pub struct Register {
    tn_fsb_int_val: u32,
    tn_fsb_int_addr: u32,
}
