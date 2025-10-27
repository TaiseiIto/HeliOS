mod native_c_state_instruction;
mod other;

use {
    super::system_description,
    alloc::vec::Vec,
    core::{fmt, mem, slice},
};

/// # Low Power Idle Table (LPIT)
/// ## References
/// * [Intel Low Power S0 Idle](https://uefi.org/sites/default/files/resources/Intel_ACPI_Low_Power_S0_Idle.pdf) 2.1 LPTI Table Main Structure
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn bytes(&self) -> &[u8] {
        let table: *const Self = self as *const Self;
        let table: *const Self = unsafe { table.add(1) };
        let table: *const u8 = table as *const u8;
        let size: usize = self.header.table_size() - mem::size_of_val(self);
        unsafe { slice::from_raw_parts(table, size) }
    }

    fn iter(&self) -> StateStructures<'_> {
        self.into()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let state_structures: Vec<StateStructure> = self.iter().collect();
        formatter
            .debug_struct("Table")
            .field("header", &header)
            .field("state_structures", &state_structures)
            .finish()
    }
}

struct StateStructures<'a> {
    bytes: &'a [u8],
}

impl<'a> From<&'a Table> for StateStructures<'a> {
    fn from(table: &'a Table) -> Self {
        let bytes: &[u8] = table.bytes();
        Self { bytes }
    }
}

impl<'a> Iterator for StateStructures<'a> {
    type Item = StateStructure<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes: &[u8] = self.bytes;
        Self::Item::scan(bytes).map(|(state_structure, remaining_bytes)| {
            self.bytes = remaining_bytes;
            state_structure
        })
    }
}

#[derive(Debug)]
enum StateStructure<'a> {
    NativeCStateInstruction(&'a native_c_state_instruction::Structure),
    Other(&'a other::Structure),
}

impl<'a> StateStructure<'a> {
    fn scan(bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        let (structure_type, structure_type_length): (u32, usize) =
            bytes.iter().take(mem::size_of::<u32>()).rev().fold(
                (0u32, 0usize),
                |(structure_type, structure_type_length), byte| {
                    (
                        (structure_type << u8::BITS) + (*byte as u32),
                        structure_type_length + 1,
                    )
                },
            );
        (structure_type_length == mem::size_of::<u32>()).then(|| match structure_type {
            0x00000000 => {
                let structure: *const u8 = bytes.first().unwrap() as *const u8;
                let structure: *const native_c_state_instruction::Structure =
                    structure as *const native_c_state_instruction::Structure;
                let structure: &native_c_state_instruction::Structure = unsafe { &*structure };
                let structure = Self::NativeCStateInstruction(structure);
                let remaining_bytes: &[u8] = &bytes[structure.size()..];
                (structure, remaining_bytes)
            }
            _ => {
                let structure: *const u8 = bytes.first().unwrap() as *const u8;
                let structure: *const other::Structure = structure as *const other::Structure;
                let structure: &other::Structure = unsafe { &*structure };
                let structure = Self::Other(structure);
                let remaining_bytes: &[u8] = &bytes[structure.size()..];
                (structure, remaining_bytes)
            }
        })
    }

    fn size(&self) -> usize {
        match self {
            Self::NativeCStateInstruction(structure) => structure.length(),
            Self::Other(structure) => structure.length(),
        }
    }
}
