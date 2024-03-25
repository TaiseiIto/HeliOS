use bitfield_struct::bitfield;

/// # Timer N Comparator Register
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.9 Timer N Comparator Register
#[bitfield(u64)]
pub struct Register {
    tn_comparator_value: u32,
    #[bits(access = RO)]
    reserved0: u32,
}

