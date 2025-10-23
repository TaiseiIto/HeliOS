//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {super::super::Return, bitfield_struct::bitfield};

#[derive(Debug)]
pub struct Ecx0x00000001 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
}

impl Ecx0x00000001 {
    pub fn get(eax: u32) -> Self {
        let ecx: u32 = 0x00000000;
        let ecx0x00000001 = Return::get(eax, ecx);
        let eax: Eax = ecx0x00000001.eax().into();
        let ebx: Ebx = ecx0x00000001.ebx().into();
        let ecx: Ecx = ecx0x00000001.ecx().into();
        Self { eax, ebx, ecx }
    }
}

#[bitfield(u32)]
struct Eax {
    palette_1_total_tile_bytes: u16,
    palette_1_bytes_per_tile: u16,
}

#[bitfield(u32)]
struct Ebx {
    palette_1_bytes_per_row: u16,
    palette_1_max_names: u16,
}

#[bitfield(u32)]
struct Ecx {
    palette_1_max_rows: u16,
    __: u16,
}
