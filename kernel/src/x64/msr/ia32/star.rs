use {
    bitfield_struct::bitfield,
    crate::memory,
    super::super::{
        rdmsr,
        super::Cpuid,
        wrmsr,
    },
};

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
                .supports_intel64_architecture()
                .then(|| rdmsr(Self::ECX).into()))
    }

    pub fn set(self) {
        let star: u64 = self.into();
        wrmsr(Self::ECX, star);
    }

    pub fn set_segment_selectors(
        cpuid: &Option<Cpuid>,
        kernel_code_segment_selector: &memory::segment::Selector,
        kernel_data_segment_selector: &memory::segment::Selector,
        application_code_segment_selector: &memory::segment::Selector,
        application_data_segment_selector: &memory::segment::Selector,
    ) {
        let kernel_code_segment_selector: u16 = (*kernel_code_segment_selector).into();
        let kernel_data_segment_selector: u16 = (*kernel_data_segment_selector).into();
        assert_eq!(kernel_code_segment_selector + 8, kernel_data_segment_selector);
        let application_code_segment_selector: u16 = (*application_code_segment_selector).into();
        let application_data_segment_selector: u16 = (*application_data_segment_selector).into();
        assert_eq!(application_data_segment_selector + 8, application_code_segment_selector);
        let syscall_cs_and_ss: u16 = kernel_code_segment_selector;
        let sysret_cs_and_ss: u16 = application_data_segment_selector - 8;
        if let Some(star) = Self::get(cpuid) {
            star.with_syscall_cs_and_ss(syscall_cs_and_ss)
                .with_sysret_cs_and_ss(sysret_cs_and_ss)
                .set();
        }
    }
}

