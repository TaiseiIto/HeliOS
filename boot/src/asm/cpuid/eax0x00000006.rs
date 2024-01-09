//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::{
        Eax0x00000000,
        Return,
    },
};

#[derive(Debug)]
pub struct Eax0x00000006 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Eax0x00000006 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000006;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
            let eax0x00000006 = Return::get(eax, ecx);
            let eax: Eax = eax0x00000006.eax().into();
            let ebx: Ebx = eax0x00000006.ebx().into();
            let ecx: Ecx = eax0x00000006.ecx().into();
            let edx: Edx = eax0x00000006.edx().into();
            Some(Self {
                eax,
                ebx,
                ecx,
                edx,
            })
        } else {
            None
        }
    }
}

#[bitfield(u32)]
pub struct Eax {
    digital_temperature_sensor_is_supported: bool,
    intel_turbo_boost_technology_available: bool,
    arat: bool,
    #[bits(access = RO)]
    reserved0: bool,
    pln: bool,
    ecmd: bool,
    ptm: bool,
    hwp: bool,
    hwp_notification: bool,
    hwp_activity_window: bool,
    hwp_energy_performance_preference: bool,
    hwp_package_level_request: bool,
    #[bits(access = RO)]
    reserved1: bool,
    hdc: bool,
    intel_turbo_boost_max_technology_30_available: bool,
    hwp_capabilities: bool,
    hwp_peci_override_is_supported: bool,
    flexible_hwp_is_supported: bool,
    fast_access_mode_for_the_ia32_hwp_request_msr_is_supported: bool,
    hw_feedback: bool,
    ignoring_idle_logical_processor_hwp_request_is_supported: bool,
    #[bits(2, access = RO)]
    reserved2: u8,
    intel_thread_director_supported: bool,
    ia32_therm_interrupt_msr_bit_25_is_supported: bool,
    #[bits(7, access = RO)]
    reserved3: u8,
}

#[bitfield(u32)]
pub struct Ebx {
    #[bits(4)]
    number_of_interrupt_thresholds_in_digital_thermal_sensor: u8,
    #[bits(28, access = RO)]
    reserved0: u32,
}

#[bitfield(u32)]
pub struct Ecx {
    hardware_coordination_feedback_capability: bool,
    #[bits(2, access = RO)]
    reserved0: u8,
    the_processor_supports_performance_energy_bias_preference: bool,
    #[bits(4, access = RO)]
    reserved1: u8,
    number_of_intel_thread_director_classes_supported_by_the_processor: u8,
    #[bits(access = RO)]
    reserved2: u16,
}

#[bitfield(u32)]
pub struct Edx  {
    bitmap_of_supported_hardware_feedback_interface_capabilities: u8,
    #[bits(4)]
    enumerates_the_size_of_the_hardware_feedback_interface_structure_in_number_of_4_kb_pages: u8,
    #[bits(4, access = RO)]
    reserved0: u8,
    index_of_this_logical_processors_row_in_the_hardware_feedback_interface_structure: u16,
}

