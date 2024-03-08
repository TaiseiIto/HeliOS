use {
    bitfield_struct::bitfield,
    core::arch::asm,
};

/// # Segment Selector
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 3.4.2 Segment Selectors, Figure 3-6. Segment Selector
#[bitfield(u16)]
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Selector {
    #[bits(2)]
    rpl: u8,
    ti: bool,
    #[bits(13)]
    index: u16,
}

impl Selector {
    pub fn create(index: u16, ti: bool, rpl: u8) -> Self {
        Self::default()
            .with_index(index)
            .with_ti(ti)
            .with_rpl(rpl)
    }

    #[inline(never)]
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

    #[inline(never)]
    pub fn ds() -> Self {
        let ds: u16;
        unsafe {
            asm!(
                "mov {0:x}, ds",
                out(reg) ds,
            );
        }
        ds.into()
    }

    #[inline(never)]
    pub fn es() -> Self {
        let es: u16;
        unsafe {
            asm!(
                "mov {0:x}, es",
                out(reg) es,
            );
        }
        es.into()
    }

    #[inline(never)]
    pub fn fs() -> Self {
        let fs: u16;
        unsafe {
            asm!(
                "mov {0:x}, fs",
                out(reg) fs,
            );
        }
        fs.into()
    }

    pub fn get_index(&self) -> u16 {
        self.index()
    }

    pub fn get_rpl(&self) -> u8 {
        self.rpl()
    }

    #[inline(never)]
    pub fn gs() -> Self {
        let gs: u16;
        unsafe {
            asm!(
                "mov {0:x}, gs",
                out(reg) gs,
            );
        }
        gs.into()
    }

    #[inline(never)]
    pub fn ss() -> Self {
        let ss: u16;
        unsafe {
            asm!(
                "mov {0:x}, ss",
                out(reg) ss,
            );
        }
        ss.into()
    }
}

