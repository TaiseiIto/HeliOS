use {
    bitfield_struct::bitfield,
    core::arch::asm,
};

/// RFLAGS
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.1A 3.4.3.4 RFLAGS Register in 64-Bit Mode
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.1A 3.4.3 EFLAGS Register
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2A 4-521 PUSHF/PUSHFD/PUSHFQ Push EFLAGS Register Onto the Stack
#[bitfield(u64)]
pub struct Rflags {
    cf: bool,
    #[bits(access = RO)]
    reserved0: bool,
    pf: bool,
    #[bits(access = RO)]
    reserved1: bool,
    af: bool,
    #[bits(access = RO)]
    reserved2: bool,
    zf: bool,
    sf: bool,
    tf: bool,
    interrupt_enable: bool,
    df: bool,
    of: bool,
    #[bits(2)]
    iopl: u8,
    nt: bool,
    #[bits(access = RO)]
    reserved3: bool,
    rf: bool,
    vm: bool,
    ac: bool,
    vif: bool,
    vip: bool,
    id: bool,
    #[bits(42, access = RO)]
    reserved4: u64,
}

impl Rflags {
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
}

