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
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    edx: Edx,
}

impl Ecx0x00000000 {
    pub fn get(eax: u32) -> Self {
        let ecx: u32 = 0x00000000;
        let ecx0x00000000 = Return::get(eax, ecx);
        let eax: Eax = ecx0x00000000.eax().into();
        let ebx: Ebx = ecx0x00000000.ebx().into();
        let ecx: Ecx = ecx0x00000000.ecx().into();
        let edx: Edx = ecx0x00000000.edx().into();
        Self {
            eax,
            ebx,
            ecx,
            edx,
        }
    }

    pub fn max_ecx(&self) -> u32 {
        self.eax.max_ecx()
    }

    pub fn pconfig(&self) -> bool {
        self.edx.pconfig()
    }

    pub fn sgx(&self) -> bool {
        self.ebx.sgx()
    }
}

#[bitfield(u32)]
struct Eax {
    max_ecx: u32,
}

#[bitfield(u32)]
struct Ebx {
    fsgsbase: bool,
    ia32_tsc_adjust_msr_is_supported: bool,
    sgx: bool,
    bmi1: bool,
    hle: bool,
    avx2: bool,
    fdp_excptn_only: bool,
    smep: bool,
    bmi2: bool,
    supports_enhanced_rep_movsb_stosb: bool,
    invcpid: bool,
    rtm: bool,
    rdt_m: bool,
    deprecates_fpu_cs_and_fpu_ds_values: bool,
    mpx: bool,
    rdt_a: bool,
    avx512f: bool,
    avx512dq: bool,
    rdseed: bool,
    adx: bool,
    smap: bool,
    avx512_ifma: bool,
    __: bool,
    clflushopt: bool,
    clwb: bool,
    intel_processor_trace: bool,
    avx512pf: bool,
    avx512er: bool,
    avx512cd: bool,
    sha: bool,
    avx512bw: bool,
    avx512vl: bool,
}

#[bitfield(u32)]
struct Ecx {
    prefetchwt1: bool,
    avx512_vbmi: bool,
    umip: bool,
    pku: bool,
    ospke: bool,
    waitpkg: bool,
    avx512_vbmi2: bool,
    cet_ss: bool,
    gfni: bool,
    vaes: bool,
    vpclmulqdq: bool,
    avx512_vnni: bool,
    avx512_bitalg: bool,
    tme_en: bool,
    avx512_vpopcntdq: bool,
    __: bool,
    la57: bool,
    #[bits(5)]
    the_value_of_mawau_used_by_the_bndldx_and_bndstx_instructions_in_64_bit_mode: u8,
    rdpid_and_ia32_tsc_aux_are_available: bool,
    kl: bool,
    bus_lock_detect: bool,
    cldemote: bool,
    __: bool,
    movdiri: bool,
    movdir64b: bool,
    enqcmd: bool,
    sgx_lc: bool,
    pks: bool,
}

#[bitfield(u32)]
struct Edx {
    __: bool,
    sgx_keys: bool,
    avx512_4vnniw: bool,
    avx512_4fmaps: bool,
    fast_short_rep_mov: bool,
    uintr: bool,
    #[bits(2)]
    __: u8,
    avx512_vp2intersect: bool,
    srbds_ctrl: bool,
    md_clear_supported: bool,
    rtm_always_abort: bool,
    __: bool,
    rtm_force_abort_supported: bool,
    serialize: bool,
    hybrid: bool,
    tsxldtrk: bool,
    __: bool,
    pconfig: bool,
    architectural_lbrs: bool,
    cet_ibt: bool,
    __: bool,
    amx_bf16: bool,
    avx512_fp16: bool,
    amx_tile: bool,
    amx_int8: bool,
    enumerates_support_for_ibrs_and_the_ibpb: bool,
    enumerates_support_for_stibp: bool,
    enumerates_support_for_l1d_flush: bool,
    enumerates_support_for_the_ia32_arch_capabilities_msr: bool,
    enumerates_support_for_the_ia32_core_capabilities_msr: bool,
    enumerates_support_for_ssbd: bool,
}

