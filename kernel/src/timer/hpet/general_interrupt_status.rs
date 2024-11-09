/// # General Interrupt Status Register
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.6 General Interrupt Status Register
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct Register {
    int_sts: u32,
    reserved0: u32
}

impl Register {
    pub fn timer_interactive_active(&self, timer: usize) -> bool {
        assert!(timer < (u32::BITS as usize));
        (self.int_sts & (1 << timer)) != 0
    }
}

