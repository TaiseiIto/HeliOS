use {
    core::{
        cmp,
        fmt,
        mem,
        slice,
    },
    super::{
        Header,
        super::super::Function,
    },
};

/// # Capability IDs
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) H. Capability IDs
#[repr(packed)]
pub struct Structure {
    header: Header,
    length: u8,
}

pub struct StructureInFunction<'a> {
    function: &'a Function,
    structure_offset: u8,
}

impl<'a> StructureInFunction<'a> {
    pub fn new(function: &'a Function, structure_offset: u8) -> Self {
        Self {
            function,
            structure_offset,
        }
    }

    fn structure(&'a self) -> &'a Structure {
        let Self {
            function,
            structure_offset,
        } = self;
        let function: &Function = function;
        let function: *const Function = function as *const Function;
        let function: usize = function as usize;
        let structure_offset: u8 = *structure_offset;
        let structure_offset: usize = structure_offset as usize;
        let structure: usize = function + structure_offset;
        let structure: *const Structure = structure as *const Structure;
        unsafe {
            &*structure
        }
    }

    fn bytes(&'a self) -> &[u8] {
        let structure: &Structure = self.structure();
        let length: usize = self.length();
        let Self {
            function,
            structure_offset,
        } = self;
        let structure_offset: usize = *structure_offset as usize;
        let start: usize = structure_offset + mem::size_of::<Structure>();
        let end: usize = structure_offset + length;
        let length: usize = end - start;
        let function: *const Function = (*function) as *const Function;
        let function: *const u8 = function as *const u8;
        let start: *const u8 = unsafe {
            function.add(start)
        };
        unsafe {
            slice::from_raw_parts(start, length)
        }
    }

    fn length(&self) -> usize {
        let structure: &Structure = self.structure();
        let length: u8 = structure.length;
        let length: usize = length as usize;
        cmp::max(length, mem::size_of::<Structure>())
    }
}

impl fmt::Debug for StructureInFunction<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Structure {
            header,
            length,
        } = self.structure();
        let bytes: &[u8] = self.bytes();
        formatter
            .debug_struct("StructureInFunction")
            .field("header", header)
            .field("length", length)
            .field("bytes", &bytes)
            .finish()
    }
}

