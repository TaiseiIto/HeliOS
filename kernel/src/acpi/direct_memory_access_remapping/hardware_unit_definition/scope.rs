use {
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem::size_of,
        slice,
    },
};

/// # Device Scope Structure
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.3.1 Device Scope Structure
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

    fn path(&self) -> &[u16] {
        let structure: *const Self = self as *const Self;
        let path: *const Self = unsafe {
            structure.add(1)
        };
        let path: *const u16 = path as *const u16;
        let length: usize = (self.length() - size_of::<Self>()) / size_of::<u16>();
        unsafe {
            slice::from_raw_parts(path, length)
        }
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let structure_type: u8 = self.structure_type;
        let length: u8 = self.length;
        let flags: Flags = self.flags;
        let reserved0: u8 = self.reserved0;
        let enumeration_id: u8 = self.enumeration_id;
        let start_bus_number: u8 = self.start_bus_number;
        formatter
            .debug_struct("Structure")
            .field("structure_type", &structure_type)
            .field("length", &length)
            .field("flags", &flags)
            .field("reserved0", &reserved0)
            .field("enumeration_id", &enumeration_id)
            .field("start_bus_number", &start_bus_number)
            .field("path", &self.path())
            .finish()
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

