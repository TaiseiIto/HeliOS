use {
    bitfield_struct::bitfield,
    super::super::{
        rdmsr,
        super::Cpuid,
        wrmsr,
    },
};

/// # IA32_EFER
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63
#[bitfield(u64)]
pub struct Efer {
    sce: bool,
    #[bits(7, access = RO)]
    reserved0: u8,
    lme: bool,
    #[bits(access = RO)]
    reserved1: bool,
    lma: bool,
    nxe: bool,
    #[bits(52, access = RO)]
    reserved2: u64,
}

impl Efer {
    const ECX: u32 = 0xc0000080;

    pub fn enable_execute_disable_bit(cpuid: &Cpuid) -> bool {
        cpuid
            .supports_execute_disable_bit()
            .then(|| Self::get(cpuid)
                .map_or(false, |efer| {
                    efer
                        .with_nxe(true)
                        .set();
                    true
                }))
            .unwrap_or(false)
    }

    pub fn enable_system_call_enable_bit(cpuid: &Cpuid) -> bool {
        Self::get(cpuid)
            .map_or(false, |efer| {
                efer
                    .with_sce(true)
                    .with_lma(true)
                    .set();
                true
            })
    }

    pub fn get(cpuid: &Cpuid) -> Option<Self> {
        cpuid
            .supports_ia32_efer()
            .then(|| rdmsr(Self::ECX).into())
    }

    pub fn pae_paging_is_used(&self) -> bool {
        !self.lme()
    }

    pub fn set(self) {
        let efer: u64 = self.into();
        wrmsr(Self::ECX, efer);
    }
}

