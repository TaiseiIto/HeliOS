//! # IA32_EFER
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63

use {
    bitfield_struct::bitfield,
    crate::memory,
    super::{
        rdmsr,
        wrmsr,
        super::Cpuid,
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

    pub fn enable_execute_disable_bit(cpuid: &Option<Cpuid>) -> bool {
        cpuid
            .as_ref()
            .map_or(false, |cpuid| cpuid.execute_disable_bit_available())
            .then(|| Self::get(cpuid)
                .map_or(false, |ia32_efer| {
                    ia32_efer
                        .with_nxe(true)
                        .set();
                    true
                }))
            .unwrap_or(false)
    }

    pub fn enable_system_call_enable_bit(cpuid: &Option<Cpuid>) -> bool {
        Self::get(cpuid)
            .map_or(false, |ia32_efer| {
                ia32_efer
                    .with_sce(true)
                    .with_lma(true)
                    .set();
                true
            })
    }

    pub fn get(cpuid: &Option<Cpuid>) -> Option<Self> {
        cpuid
            .as_ref()
            .and_then(|cpuid| cpuid
                .ia32_efer_is_supported()
                .then(|| rdmsr(Self::ECX).into()))
    }

    pub fn pae_paging_is_used(&self) -> bool {
        !self.lme()
    }

    pub fn set(self) {
        let ia32_efer: u64 = self.into();
        wrmsr(Self::ECX, ia32_efer);
    }
}

/// # IA32_STAR
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3 5.8.8 Figure 5-14
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63
#[bitfield(u64)]
pub struct Star {
    reserved0: u32,
    syscall_cs_and_ss: u16,
    sysret_cs_and_ss: u16,
}

impl Star {
    const ECX: u32 = 0xc0000081;

    pub fn get(cpuid: &Option<Cpuid>) -> Option<Self> {
        cpuid
            .as_ref()
            .and_then(|cpuid| cpuid
                .intel64_architecture_available()
                .then(|| rdmsr(Self::ECX).into()))
    }

    pub fn set(self) {
        let ia32_star: u64 = self.into();
        wrmsr(Self::ECX, ia32_star);
    }

    pub fn set_segment_selectors(
        cpuid: &Option<Cpuid>,
        kernel_code_segment_selector: &memory::segment::Selector,
        kernel_data_segment_selector: &memory::segment::Selector,
        application_code_segment_selector: &memory::segment::Selector,
        application_data_segment_selector: &memory::segment::Selector,
    ) {
        let kernel_code_segment_selector: u16 = kernel_code_segment_selector.clone().into();
        let kernel_data_segment_selector: u16 = kernel_data_segment_selector.clone().into();
        assert_eq!(kernel_code_segment_selector + 8, kernel_data_segment_selector);
        let application_code_segment_selector: u16 = application_code_segment_selector.clone().into();
        let application_data_segment_selector: u16 = application_data_segment_selector.clone().into();
        assert_eq!(application_data_segment_selector + 8, application_code_segment_selector);
        let syscall_cs_and_ss: u16 = kernel_code_segment_selector;
        let sysret_cs_and_ss: u16 = application_data_segment_selector - 8;
        Self::get(cpuid)
            .map(|ia32_star| ia32_star
                .with_syscall_cs_and_ss(syscall_cs_and_ss)
                .with_sysret_cs_and_ss(sysret_cs_and_ss)
                .set());
    }
}

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
                .intel64_architecture_available()
                .then(|| rdmsr(Self::ECX).into()))
    }
}

/// # IA32_FMASK
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3 5.8.8 Figure 5-14
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-64
#[bitfield(u64)]
pub struct Fmask {
    syscall_eflags_mask: u32,
    reserved0: u32,
}

impl Fmask {
    const ECX: u32 = 0xc0000084;

    pub fn get(cpuid: &Option<Cpuid>) -> Option<Self> {
        cpuid
            .as_ref()
            .and_then(|cpuid| cpuid
                .intel64_architecture_available()
                .then(|| rdmsr(Self::ECX).into()))
    }
}

