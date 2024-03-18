use {
    bitfield_struct::bitfield,
    super::super::{
        rdmsr,
        super::Cpuid,
        wrmsr,
    },
};

/// # IA32_LSTAR
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3 5.8.8 Figure 5-14
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63
#[bitfield(u64)]
pub struct Lstar {
    target_rip_for_64bit_mode_calling_program: u64,
}

impl Lstar {
    const ECX: u32 = 0xc0000082;

    pub fn get(cpuid: &Option<Cpuid>) -> Option<Self> {
        cpuid
            .as_ref()
            .and_then(|cpuid| cpuid
                .supports_intel64_architecture()
                .then(|| rdmsr(Self::ECX).into()))
    }

    pub fn set(self) {
        let lstar: u64 = self.into();
        wrmsr(Self::ECX, lstar);
    }

    pub fn set_handler(cpuid: &Option<Cpuid>, handler: unsafe extern "C" fn()) {
        let handler: usize = handler as usize;
        let handler: u64 = handler as u64;
        if let Some(lstar) = Self::get(cpuid) {
            lstar
                .with_target_rip_for_64bit_mode_calling_program(handler)
                .set();
        }
    }
}

