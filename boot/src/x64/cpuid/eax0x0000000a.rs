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
pub struct Eax0x0000000a {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Eax0x0000000a {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x0000000a;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let eax0x0000000a = Return::get(eax, ecx);
            let eax: Eax = eax0x0000000a.eax().into();
            let ebx: Ebx = eax0x0000000a.ebx().into();
            let ecx: Ecx = eax0x0000000a.ecx().into();
            let edx: Edx = eax0x0000000a.edx().into();
            Self {
                eax,
                ebx,
                ecx,
                edx,
            }
        })
    }
}

#[bitfield(u32)]
struct Eax {
    version_id_of_architectural_performance_monitoring: u8,
    number_of_general_purpose_performance_monitoring_counter_per_logical_processor: u8,
    bit_width_of_general_purpose_performance_monitoring_counter: u8,
    length_of_ebx_bit_vector_to_enumerate_architectural_performance_monitoring_events: u8,
}

#[bitfield(u32)]
struct Ebx {
    core_cycle_event_not_available: bool,
    instruction_retired_event_not_available: bool,
    reference_cycles_event_not_available: bool,
    last_level_cache_reference_event_not_available: bool,
    last_level_cache_misses_event_not_available: bool,
    branch_instruction_retired_event_not_available: bool,
    branch_mispredict_retired_event_not_available: bool,
    top_down_slots_event_not_available: bool,
    #[bits(24)]
    __: u32,
}

#[bitfield(u32)]
struct Ecx {
    supported_fixed_counters_bit_mask: u32,
}

#[bitfield(u32)]
struct Edx {
    #[bits(5)]
    number_of_contiguous_fixed_function_performance_counters_starting_from_0: u8,
    bit_width_of_fixed_function_performance_counters: u8,
    #[bits(2)]
    __: u8,
    any_thread_deprecation: bool,
    __: u16,
}

