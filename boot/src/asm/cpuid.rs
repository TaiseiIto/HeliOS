
use {
    core::arch::asm,
    super::Rflags,
};

mod eax0x00000000;
mod eax0x00000001;
mod eax0x00000002;
mod eax0x00000003;

pub use eax0x00000000::Eax0x00000000;
pub use eax0x00000001::Eax0x00000001;
pub use eax0x00000002::Eax0x00000002;
pub use eax0x00000003::Eax0x00000003;

/// # CPUID
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217
#[derive(Debug)]
pub struct Cpuid {
    eax0x00000000: Eax0x00000000,
    eax0x00000001: Option<Eax0x00000001>,
    eax0x00000002: Option<Eax0x00000002>,
    eax0x00000003: Option<Eax0x00000003>,
}

impl Cpuid {
    pub fn get() -> Option<Self> {
        if Rflags::cpuid_is_supported() {
            let eax0x00000000: Eax0x00000000 = Eax0x00000000::get();
            let eax0x00000001: Option<Eax0x00000001> = Eax0x00000001::get(&eax0x00000000);
            let eax0x00000002: Option<Eax0x00000002> = Eax0x00000002::get(&eax0x00000000);
            let eax0x00000003: Option<Eax0x00000003> = eax0x00000001
                .as_ref()
                .map(|eax0x00000001| Eax0x00000003::get(&eax0x00000000, &eax0x00000001))
                .flatten();
            Some(Self {
                eax0x00000000,
                eax0x00000001,
                eax0x00000002,
                eax0x00000003,
            })
        } else {
            None
        }
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

