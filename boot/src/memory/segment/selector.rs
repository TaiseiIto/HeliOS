use {
    bitfield_struct::bitfield,
    core::arch::asm,
};

/// # Segment Selector
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 3.4.2 Segment Selectors, Figure 3-6. Segment Selector
#[bitfield(u16)]
pub struct Selector {
    #[bits(2)]
    rpl: u8,
    ti: bool,
    #[bits(13)]
    index: u16,
}

impl Selector {
    pub fn cs() -> Self {
        let cs: u16;
        unsafe {
            asm!(
                "mov {0:x}, cs",
                out(reg) cs,
            );
        }
        cs.into()
    }
}

