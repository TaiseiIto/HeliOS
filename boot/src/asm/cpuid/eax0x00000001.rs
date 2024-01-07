//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::Return,
};

#[derive(Debug)]
pub struct Eax0x00000001 {
    eax: Eax,
    ebx: Ebx,
    ecx: Ecx,
    edx: Edx,
}

impl Eax0x00000001 {
    pub fn get() -> Option<Self> {
        let eax: u32 = 0x00000001;
        let ecx: u32 = 0x00000000;
        Return::get(eax, ecx).map(|cpuid_return| {
            let eax: Eax = cpuid_return.eax().into();
            let ebx: Ebx = cpuid_return.ebx().into();
            let ecx: Ecx = cpuid_return.ecx().into();
            let edx: Edx = cpuid_return.edx().into();
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
pub struct Eax {
    version_information: u32,
}

#[bitfield(u32)]
pub struct Ebx {
    brand_index: u8,
    clflush_line_size: u8,
    maximum_number_of_addressable_ids_for_logical_processors: u8,
    initial_apic_id: u8,
}

/// # CPUID 0x00000001
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-243 Figure 3-7. Feature Information Returned in the ECX Register
#[bitfield(u32)]
pub struct Ecx {
    sse3: bool,
    pclmulqdq: bool,
    dtes64: bool,
    monitor: bool,
    ds_cpl: bool,
    vmx: bool,
    smx: bool,
    eist: bool,
    tm2: bool,
    ssse3: bool,
    cnxt_id: bool,
    sdbg: bool,
    fma: bool,
    cmpxchg16b: bool,
    xtpr: bool,
    pdcm: bool,
    reserved0: bool,
    pcid: bool,
    dca: bool,
    sse4_1: bool,
    sse4_2: bool,
    x2apic: bool,
    movbe: bool,
    popcnt: bool,
    tsc_deadline: bool,
    aes: bool,
    xsave: bool,
    osxsave: bool,
    avx: bool,
    f16c: bool,
    rdrand: bool,
    reserved1: bool,
}

/// # CPUID 0x00000001
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-245 Figure 3-8. Feature Information Returned in the EDX Register
#[bitfield(u32)]
pub struct Edx {
    fpu: bool,
    vme: bool,
    de: bool,
    pse: bool,
    tsc: bool,
    msr: bool,
    pae: bool,
    mce: bool,
    cx8: bool,
    apic: bool,
    reserved0: bool,
    sep: bool,
    mtrr: bool,
    pge: bool,
    mca: bool,
    cmov: bool,
    pat: bool,
    ps: bool,
    psn: bool,
    clfsh: bool,
    reserved1: bool,
    ds: bool,
    acpi: bool,
    mmx: bool,
    fxsr: bool,
    sse: bool,
    sse2: bool,
    ss: bool,
    htt: bool,
    tm: bool,
    reserved2: bool,
    pbe: bool,
}

