//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

mod ecx0x00000000;
mod ecx0x00000001;
mod ecx0x00000002;

use {
    super::Eax0x00000000,
    ecx0x00000000::Ecx0x00000000,
    ecx0x00000001::Ecx0x00000001,
    ecx0x00000002::Ecx0x00000002,
};

#[derive(Debug)]
pub struct Eax0x00000007 {
    #[allow(dead_code)]
    ecx0x00000000: Ecx0x00000000,
    #[allow(dead_code)]
    ecx0x00000001: Option<Ecx0x00000001>,
    #[allow(dead_code)]
    ecx0x00000002: Option<Ecx0x00000002>,
}

impl Eax0x00000007 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000007;
        if eax <= eax0x00000000.max_eax() {
            let ecx0x00000000 = Ecx0x00000000::get(eax);
            let ecx0x00000001: Option<Ecx0x00000001> = Ecx0x00000001::get(eax, &ecx0x00000000);
            let ecx0x00000002: Option<Ecx0x00000002> = Ecx0x00000002::get(eax, &ecx0x00000000);
            Some(Self {
                ecx0x00000000,
                ecx0x00000001,
                ecx0x00000002,
            })
        } else {
            None
        }
    }

    pub fn pconfig(&self) -> bool {
        self.ecx0x00000000.pconfig()
    }

    pub fn sgx(&self) -> bool {
        self.ecx0x00000000.sgx()
    }
}

