//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    alloc::string::String,
    core::{
        arch::asm,
        cell::OnceCell,
    },
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
mod eax0x00000014;
mod eax0x00000015;
mod eax0x00000016;
mod eax0x00000017;
mod eax0x00000018;
mod eax0x00000019;
mod eax0x0000001a;
mod eax0x0000001b;
mod eax0x0000001c;
mod eax0x0000001d;
mod eax0x0000001e;
mod eax0x0000001f;
mod eax0x00000020;
mod eax0x80000000;
mod eax0x80000001;
mod eax0x80000006;
mod eax0x80000007;
mod eax0x80000008;

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
    eax0x00000014::Eax0x00000014,
    eax0x00000015::Eax0x00000015,
    eax0x00000016::Eax0x00000016,
    eax0x00000017::Eax0x00000017,
    eax0x00000018::Eax0x00000018,
    eax0x00000019::Eax0x00000019,
    eax0x0000001a::Eax0x0000001a,
    eax0x0000001b::Eax0x0000001b,
    eax0x0000001c::Eax0x0000001c,
    eax0x0000001d::Eax0x0000001d,
    eax0x0000001e::Eax0x0000001e,
    eax0x0000001f::Eax0x0000001f,
    eax0x00000020::Eax0x00000020,
    eax0x80000000::Eax0x80000000,
    eax0x80000001::Eax0x80000001,
    eax0x80000006::Eax0x80000006,
    eax0x80000007::Eax0x80000007,
    eax0x80000008::Eax0x80000008,
};

static mut CPUID: OnceCell<Cpuid> = OnceCell::new();

/// # CPUID
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2A 3-217
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
    #[allow(dead_code)]
    eax0x00000014: Option<Eax0x00000014>,
    #[allow(dead_code)]
    eax0x00000015: Option<Eax0x00000015>,
    #[allow(dead_code)]
    eax0x00000016: Option<Eax0x00000016>,
    #[allow(dead_code)]
    eax0x00000017: Option<Eax0x00000017>,
    #[allow(dead_code)]
    eax0x00000018: Option<Eax0x00000018>,
    #[allow(dead_code)]
    eax0x00000019: Option<Eax0x00000019>,
    #[allow(dead_code)]
    eax0x0000001a: Option<Eax0x0000001a>,
    #[allow(dead_code)]
    eax0x0000001b: Option<Eax0x0000001b>,
    #[allow(dead_code)]
    eax0x0000001c: Option<Eax0x0000001c>,
    #[allow(dead_code)]
    eax0x0000001d: Option<Eax0x0000001d>,
    #[allow(dead_code)]
    eax0x0000001e: Option<Eax0x0000001e>,
    #[allow(dead_code)]
    eax0x0000001f: Option<Eax0x0000001f>,
    #[allow(dead_code)]
    eax0x00000020: Option<Eax0x00000020>,
    #[allow(dead_code)]
    eax0x80000000: Eax0x80000000,
    eax0x80000001: Option<Eax0x80000001>,
    #[allow(dead_code)]
    processor_brand_string: Option<String>,
    #[allow(dead_code)]
    eax0x80000006: Option<Eax0x80000006>,
    #[allow(dead_code)]
    eax0x80000007: Option<Eax0x80000007>,
    #[allow(dead_code)]
    eax0x80000008: Option<Eax0x80000008>,
}

impl Cpuid {
    pub fn get() -> &'static Self {
        unsafe {
            CPUID.get()
        }.unwrap()
    }

    pub fn set() {
        assert!(Rflags::cpuid_is_supported());
        let eax0x00000000: Eax0x00000000 = Eax0x00000000::get();
        let eax0x00000001: Option<Eax0x00000001> = Eax0x00000001::get(&eax0x00000000);
        let eax0x00000002: Option<Eax0x00000002> = Eax0x00000002::get(&eax0x00000000);
        let eax0x00000003: Option<Eax0x00000003> = Eax0x00000003::get(&eax0x00000000, &eax0x00000001);
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
        let eax0x00000012: Option<Eax0x00000012> = Eax0x00000012::get(&eax0x00000000, &eax0x00000007);
        let eax0x00000014: Option<Eax0x00000014> = Eax0x00000014::get(&eax0x00000000);
        let eax0x00000015: Option<Eax0x00000015> = Eax0x00000015::get(&eax0x00000000);
        let eax0x00000016: Option<Eax0x00000016> = Eax0x00000016::get(&eax0x00000000);
        let eax0x00000017: Option<Eax0x00000017> = Eax0x00000017::get(&eax0x00000000);
        let eax0x00000018: Option<Eax0x00000018> = Eax0x00000018::get(&eax0x00000000);
        let eax0x00000019: Option<Eax0x00000019> = Eax0x00000019::get(&eax0x00000000);
        let eax0x0000001a: Option<Eax0x0000001a> = Eax0x0000001a::get(&eax0x00000000);
        let eax0x0000001b: Option<Eax0x0000001b> = Eax0x0000001b::get(&eax0x00000000, &eax0x00000007);
        let eax0x0000001c: Option<Eax0x0000001c> = Eax0x0000001c::get(&eax0x00000000);
        let eax0x0000001d: Option<Eax0x0000001d> = Eax0x0000001d::get(&eax0x00000000);
        let eax0x0000001e: Option<Eax0x0000001e> = Eax0x0000001e::get(&eax0x00000000);
        let eax0x0000001f: Option<Eax0x0000001f> = Eax0x0000001f::get(&eax0x00000000);
        let eax0x00000020: Option<Eax0x00000020> = Eax0x00000020::get(&eax0x00000000);
        let eax0x80000000: Eax0x80000000 = Eax0x80000000::get();
        let eax0x80000001: Option<Eax0x80000001> = Eax0x80000001::get(&eax0x80000000);
        let processor_brand_string: Option<String> = String::from_utf8((0x80000002..=0x80000004)
                .take_while(|eax| *eax <= eax0x80000000.max_eax())
                .flat_map(|eax| {
                    let ecx: u32 = 0x00000000;
                    Return::get(eax, ecx)
                        .eax_ebx_ecx_edx()
                        .into_iter()
                        .flat_map(|dword| dword
                            .to_le_bytes()
                            .into_iter())
                })
                .collect())
            .ok()
            .map(|processor_brand_string| processor_brand_string.replace('\0', ""));
        let eax0x80000006: Option<Eax0x80000006> = Eax0x80000006::get(&eax0x80000000);
        let eax0x80000007: Option<Eax0x80000007> = Eax0x80000007::get(&eax0x80000000);
        let eax0x80000008: Option<Eax0x80000008> = Eax0x80000008::get(&eax0x80000000);
        let cpuid = Self {
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
            eax0x00000014,
            eax0x00000015,
            eax0x00000016,
            eax0x00000017,
            eax0x00000018,
            eax0x00000019,
            eax0x0000001a,
            eax0x0000001b,
            eax0x0000001c,
            eax0x0000001d,
            eax0x0000001e,
            eax0x0000001f,
            eax0x00000020,
            eax0x80000000,
            eax0x80000001,
            processor_brand_string,
            eax0x80000006,
            eax0x80000007,
            eax0x80000008,
        };
        unsafe {
            CPUID.set(cpuid)
        }.unwrap()
    }

    pub fn supports_apic(&self) -> bool {
        self.eax0x00000001
            .as_ref()
            .map_or(false, |eax0x00000001| eax0x00000001.supports_apic())
    }

    pub fn supports_execute_disable_bit(&self) -> bool {
        self.eax0x80000001
            .as_ref()
            .map_or(false, |eax0x80000001| eax0x80000001.supports_execute_disable_bit())
    }

    /// # GEt IA32_EFER availability.
    /// ## References
    /// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63
    pub fn supports_ia32_efer(&self) -> bool {
        self.eax0x80000001
            .as_ref()
            .map_or(false, |eax0x80000001| eax0x80000001.supports_ia32_efer())
    }

    /// # Get intel64 architecture availability.
    /// ## References
    /// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63
    pub fn supports_intel64_architecture(&self) -> bool {
        self.eax0x80000001
            .as_ref()
            .map_or(false, |eax0x80000001| eax0x80000001.supports_intel64_architecture())
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

