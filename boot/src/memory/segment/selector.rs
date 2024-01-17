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

#[derive(Debug)]
pub struct Interface {
    #[allow(dead_code)]
    rpl: u8,
    #[allow(dead_code)]
    ti: bool,
    #[allow(dead_code)]
    index: u16,
}

impl From<Selector> for Interface {
    fn from(selector: Selector) -> Self {
        let rpl: u8 = selector.rpl();
        let ti: bool = selector.ti();
        let index: u16 = selector.index() << Selector::INDEX_OFFSET;
        Self {
            rpl,
            ti,
            index,
        }
    }
}

