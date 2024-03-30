use bitfield_struct::bitfield;

/// # Device Scope Structure
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.3.1 Device Scope Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    flags: Flags,
    reserved0: u8,
    enumeration_id: u8,
    start_bus_number: u8,
}

impl Structure {
    pub fn scan(bytes: &[u8]) -> Option<(&Self, &[u8])> {
        bytes
            .first()
            .map(|structure| {
                let structure: *const u8 = structure as *const u8;
                let structure: *const Self = structure as *const Self;
                let structure: &Self = unsafe {
                    &*structure
                };
                let remaining_bytes: &[u8] = &bytes[structure.length()..];
                (structure, remaining_bytes)
            })
    }

    fn length(&self) -> usize {
        self.length as usize
    }
}

#[bitfield(u8)]
struct Flags {
    req_wo_pasid_nested_notallowed: bool,
    req_wo_pasid_pwsnp_notallowed: bool,
    req_wo_pasid_pgsnp_notallowed: bool,
    atc_hardened: bool,
    atc_required: bool,
    #[bits(3, access = RO)]
    reserved0: u8,
}

