//! # IA32_EFER
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63

mod apic_base;
mod efer;
mod fmask;
mod lstar;
mod star;

pub use {
    apic_base::ApicBase,
    efer::Efer,
    fmask::Fmask,
    lstar::Lstar,
    star::Star,
};

