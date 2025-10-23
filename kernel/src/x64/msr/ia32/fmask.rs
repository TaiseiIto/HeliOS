use {
    super::super::{super::Cpuid, rdmsr, wrmsr},
    bitfield_struct::bitfield,
};

/// # IA32_FMASK
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3 5.8.8 Figure 5-14
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-64
#[bitfield(u64)]
pub struct Fmask {
    syscall_eflags_mask: u32,
    __: u32,
}

impl Fmask {
    const ECX: u32 = 0xc0000084;

    pub fn get(cpuid: &Cpuid) -> Option<Self> {
        cpuid
            .supports_intel64_architecture()
            .then(|| rdmsr(Self::ECX).into())
    }

    pub fn set_all_flags(cpuid: &Cpuid) {
        if let Some(fmask) = Self::get(cpuid) {
            fmask.with_syscall_eflags_mask(u32::MAX).set();
        }
    }

    pub fn set(self) {
        let fmask: u64 = self.into();
        wrmsr(Self::ECX, fmask);
    }
}
