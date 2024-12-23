use {
    bitfield_struct::bitfield,
    core::arch::asm,
};

/// # RFLAGS
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.1A 3.4.3.4 RFLAGS Register in 64-Bit Mode
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.1A 3.4.3 EFLAGS Register
#[bitfield(u64)]
pub struct Rflags {
    cf: bool,
    __: bool,
    pf: bool,
    __: bool,
    af: bool,
    __: bool,
    zf: bool,
    sf: bool,
    tf: bool,
    interrupt_enable: bool,
    df: bool,
    of: bool,
    #[bits(2)]
    iopl: u8,
    nt: bool,
    __: bool,
    rf: bool,
    vm: bool,
    ac: bool,
    vif: bool,
    vip: bool,
    id: bool,
    #[bits(42)]
    __: u64,
}

impl Rflags {
    /// # Get RFLAGS
    /// ## References
    /// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2A 4-521 PUSHF/PUSHFD/PUSHFQ Push EFLAGS Register Onto the Stack
    #[inline(never)]
    pub fn get() -> Self {
        let mut rflags: u64;
        unsafe {
            asm!(
                "pushfq",
                "pop {0}",
                out(reg) rflags,
            );
        }
        rflags.into()
    }

    /// # Set RFLAGS
    /// ## References
    /// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2B 4-401 POPF/POPFD/POPFQ Pop Stack Into EFLAGS Register
    #[inline(never)]
    pub fn set(self) {
        let rflags: u64 = self.into();
        unsafe {
            asm!(
                "push {0}",
                "popfq",
                in(reg) rflags,
            );
        }
    }

    pub fn cpuid_is_supported() -> bool {
        Self::get().with_id(true).set();
        let rflags = Self::get();
        if rflags.id() {
            rflags.with_id(false).set();
            !Self::get().id()
        } else {
            false
        }
    }
}

