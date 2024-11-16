use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem::size_of,
        slice,
    },
    super::hardware_unit_definition,
};

/// # Root Port ATS Capability Reporting Structure
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.5 Root Port ATS Capability Reporting Structure
#[repr(packed)]
pub struct Structure {
    structure_type: u16,
    length: u16,
    flags: Flags,
    __: u8,
    segment_number: u16,
}

impl Structure {
    pub fn bytes(&self) -> &[u8] {
        let structure: *const Self = self as *const Self;
        let first_byte: *const Self = unsafe {
            structure.add(1)
        };
        let first_byte: *const u8 = first_byte as *const u8;
        let size: usize = self.length() - size_of::<Self>();
        unsafe {
            slice::from_raw_parts(first_byte, size)
        }
    }

    pub fn length(&self) -> usize {
        self.length as usize
    }

    fn iter(&self) -> hardware_unit_definition::Scopes<'_> {
        self.into()
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let structure_type: u16 = self.structure_type;
        let length: u16 = self.length;
        let flags: Flags = self.flags;
        let segment_number: u16 = self.segment_number;
        let scopes: Vec<&hardware_unit_definition::scope::Structure> = self
            .iter()
            .collect();
        formatter
            .debug_struct("Structure")
            .field("structure_type", &structure_type)
            .field("length", &length)
            .field("flags", &flags)
            .field("segment_number", &segment_number)
            .field("scopes", &scopes)
            .finish()
    }
}

#[bitfield(u8)]
struct Flags {
    all_ports: bool,
    #[bits(7)]
    __: u8,
}

