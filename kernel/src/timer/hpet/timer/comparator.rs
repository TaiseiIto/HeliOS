use {
    bitfield_struct::bitfield,
    crate::{
        com2_print,
        com2_println,
    },
};

/// # Timer N Comparator Register
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.9 Timer N Comparator Register
#[bitfield(u64)]
pub struct Register {
    tn_comparator_value: u64,
}

impl Register {
    pub fn create(tn_comparator_value: u64) -> Self {
        com2_println!("tn_comparator_value = {:#x?}", tn_comparator_value);
        Self::default().with_tn_comparator_value(tn_comparator_value)
    }
}

