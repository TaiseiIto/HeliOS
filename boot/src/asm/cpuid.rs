//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    core::arch::asm,
    super::Rflags,
};

mod eax0x00000000;
mod eax0x00000001;
mod eax0x00000002;
mod eax0x00000003;
mod eax0x00000004;
mod eax0x00000005;
mod eax0x00000006;
mod eax0x00000007;
mod eax0x00000009;
mod eax0x0000000a;
mod eax0x0000000b;
mod eax0x0000000d;
mod eax0x0000000f;
mod eax0x00000010;
mod eax0x00000012;

pub use {
    eax0x00000000::Eax0x00000000,
    eax0x00000001::Eax0x00000001,
    eax0x00000002::Eax0x00000002,
    eax0x00000003::Eax0x00000003,
    eax0x00000004::Eax0x00000004,
    eax0x00000005::Eax0x00000005,
    eax0x00000006::Eax0x00000006,
    eax0x00000007::Eax0x00000007,
    eax0x00000009::Eax0x00000009,
    eax0x0000000a::Eax0x0000000a,
    eax0x0000000b::Eax0x0000000b,
    eax0x0000000d::Eax0x0000000d,
    eax0x0000000f::Eax0x0000000f,
    eax0x00000010::Eax0x00000010,
    eax0x00000012::Eax0x00000012,
};

/// # CPUID
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217
#[derive(Debug)]
pub struct Cpuid {
    #[allow(dead_code)]
    eax0x00000000: Eax0x00000000,
    #[allow(dead_code)]
    eax0x00000001: Option<Eax0x00000001>,
    #[allow(dead_code)]
    eax0x00000002: Option<Eax0x00000002>,
    #[allow(dead_code)]
    eax0x00000003: Option<Eax0x00000003>,
    #[allow(dead_code)]
    eax0x00000004: Option<Eax0x00000004>,
    #[allow(dead_code)]
    eax0x00000005: Option<Eax0x00000005>,
    #[allow(dead_code)]
    eax0x00000006: Option<Eax0x00000006>,
    #[allow(dead_code)]
    eax0x00000007: Option<Eax0x00000007>,
    #[allow(dead_code)]
    eax0x00000009: Option<Eax0x00000009>,
    #[allow(dead_code)]
    eax0x0000000a: Option<Eax0x0000000a>,
    #[allow(dead_code)]
    eax0x0000000b: Option<Eax0x0000000b>,
    #[allow(dead_code)]
    eax0x0000000d: Option<Eax0x0000000d>,
    #[allow(dead_code)]
    eax0x0000000f: Option<Eax0x0000000f>,
    #[allow(dead_code)]
    eax0x00000010: Option<Eax0x00000010>,
    #[allow(dead_code)]
    eax0x00000012: Option<Eax0x00000012>,
}

impl Cpuid {
    pub fn get() -> Option<Self> {
        if Rflags::cpuid_is_supported() {
            let eax0x00000000: Eax0x00000000 = Eax0x00000000::get();
            let eax0x00000001: Option<Eax0x00000001> = Eax0x00000001::get(&eax0x00000000);
            let eax0x00000002: Option<Eax0x00000002> = Eax0x00000002::get(&eax0x00000000);
            let eax0x00000003: Option<Eax0x00000003> = eax0x00000001
                .as_ref()
                .and_then(|eax0x00000001| Eax0x00000003::get(&eax0x00000000, eax0x00000001));
            let eax0x00000004: Option<Eax0x00000004> = Eax0x00000004::get(&eax0x00000000);
            let eax0x00000005: Option<Eax0x00000005> = Eax0x00000005::get(&eax0x00000000);
            let eax0x00000006: Option<Eax0x00000006> = Eax0x00000006::get(&eax0x00000000);
            let eax0x00000007: Option<Eax0x00000007> = Eax0x00000007::get(&eax0x00000000);
            let eax0x00000009: Option<Eax0x00000009> = Eax0x00000009::get(&eax0x00000000);
            let eax0x0000000a: Option<Eax0x0000000a> = Eax0x0000000a::get(&eax0x00000000);
            let eax0x0000000b: Option<Eax0x0000000b> = Eax0x0000000b::get(&eax0x00000000);
            let eax0x0000000d: Option<Eax0x0000000d> = Eax0x0000000d::get(&eax0x00000000);
            let eax0x0000000f: Option<Eax0x0000000f> = Eax0x0000000f::get(&eax0x00000000);
            let eax0x00000010: Option<Eax0x00000010> = Eax0x00000010::get(&eax0x00000000);
            let eax0x00000012: Option<Eax0x00000012> = Eax0x00000012::get(&eax0x00000000);
            Some(Self {
                eax0x00000000,
                eax0x00000001,
                eax0x00000002,
                eax0x00000003,
                eax0x00000004,
                eax0x00000005,
                eax0x00000006,
                eax0x00000007,
                eax0x00000009,
                eax0x0000000a,
                eax0x0000000b,
                eax0x0000000d,
                eax0x0000000f,
                eax0x00000010,
                eax0x00000012,
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

