//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::super::Return,
};

#[derive(Debug)]
pub struct Ecx0x00000000 {
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
}

impl Ecx0x00000000 {
    pub fn get(eax: u32) -> Self {
        let ecx: u32 = 0x00000000;
        let ecx0x00000000 = Return::get(eax, ecx);
        let eax: Eax = ecx0x00000000.eax().into();
        let ebx: Ebx = ecx0x00000000.ebx().into();
        let ecx: Ecx = ecx0x00000000.ecx().into();
        Self {
            eax,
            ebx,
            ecx,
        }
    }

    pub fn max_ecx(&self) -> u32 {
        self.eax.max_ecx()
    }
}

#[bitfield(u32)]
struct Eax {
    max_ecx: u32,
}

#[bitfield(u32)]
struct Ebx {
    ia32_rtit_ctl_cr3filter_can_be_set_to_1_and_ia32_rtit_cr3_match_msr_can_be_accessed: bool,
    support_of_configurable_psb_and_cycle_accurate_mode: bool,
    support_of_ip_filtering_tracestop_filtering_and_preservation_of_intel_pt_msrs_across_warm_reset: bool,
    support_of_mtc_timing_packet_and_suppression_of_cofi_based_packets: bool,
    support_of_ptwrite: bool,
    support_of_power_event_trace: bool,
    support_for_psb_and_pmi_preservation: bool,
    writes_can_set_ia32_rtit_ctl_31_eventen_enabling_event_trace_packet_generation: bool,
    writes_can_set_ia32_rtit_ctl_55_distnt_disabling_tnt_packet_generation: bool,
    #[bits(23, access = RO)]
    reserved0: u32,
}

#[bitfield(u32)]
struct Ecx {
    tracing_can_be_enabled_with_ia32_rtit_ctl_topa_1_hence_utilizing_the_topa_output_scheme_ia32_rtit_output_base_and_ia32_rtit_output_mask_ptrs_msrs_can_be_accessed: bool,
    topa_tables_can_hold_any_number_of_output_entries_up_to_the_maximum_allowed_by_the_maskortableoffset_field_of_ia32_rtit_output_mask_ptrs: bool,
    support_of_single_range_output_scheme: bool,
    support_of_output_to_trace_transport_subsystem: bool,
    #[bits(27, access = RO)]
    reserved0: u32,
    generated_packets_which_contain_ip_payloads_have_lip_values_which_include_the_cs_base_component: bool,
}

