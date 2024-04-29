//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

mod eax0x00000000;

pub use {
    eax0x00000000::Eax0x00000000,
    super::Rflags,
};

use core::arch::asm;

/// # CPUID
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2A 3-217
#[derive(Debug)]
pub struct Cpuid {
    eax0x00000000: Eax0x00000000,
}

impl Cpuid {
    pub fn get() -> Option<Self> {
        Rflags::cpuid_is_supported().then(|| {
            let eax0x00000000: Eax0x00000000 = Eax0x00000000::get();
            Self {
                eax0x00000000,
            }
        })
    }

    pub fn max_eax(&self) -> u32 {
        self.eax0x00000000.max_eax()
    }
}

pub struct Return {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl Return {
    #[inline(never)]
    pub fn get(mut eax: u32, mut ecx: u32) -> Self {
        let mut ebx: u32;
        let mut edx: u32;
        unsafe {
            asm!(
                "cpuid",
                "mov {0:e}, ebx",
                out(reg) ebx,
                inout("eax") eax,
                inout("ecx") ecx,
                out("edx") edx,
            );
        }
        Self {
            eax,
            ebx,
            ecx,
            edx,
        }
    }

    pub fn eax_ebx_ecx_edx(self) -> [u32; 4] {
        self.into()
    }

    pub fn eax(&self) -> u32 {
        self.eax
    }

    pub fn ebx(&self) -> u32 {
        self.ebx
    }

    pub fn ecx(&self) -> u32 {
        self.ecx
    }

    pub fn edx(&self) -> u32 {
        self.edx
    }
}

impl From<Return> for [u32; 4] {
    fn from(cpuid_return: Return) -> Self {
        let eax: u32 = cpuid_return.eax();
        let ebx: u32 = cpuid_return.ebx();
        let ecx: u32 = cpuid_return.ecx();
        let edx: u32 = cpuid_return.edx();
        [eax, ebx, ecx, edx]
    }
}

