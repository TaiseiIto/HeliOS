use super::super::Function;

/// # MSI Capability Structure
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.1. MSI Capability Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    capability_id: u8,
    next_pointer: u8,
}

pub struct Structures<'a> {
    function: &'a Function,
    next_pointer: u8,
}

impl<'a> From<&'a Function> for Structures<'a> {
    fn from(function: &'a Function) -> Self {
        let next_pointer: u8 = function.capabilities_pointer();
        Self {
            function,
            next_pointer,
        }
    }
}

impl<'a> Iterator for Structures<'a> {
    type Item = &'a Structure;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            function,
            next_pointer,
        } = self;
        let function: *const Function = *function as *const Function;
        let function: usize = function as usize;
        let offset: u8 = *next_pointer;
        (offset != 0).then(|| {
            let offset: usize = offset as usize;
            let structure: usize = function + offset;
            let structure: *const Structure = structure as *const Structure;
            let structure: &Structure = unsafe {
                &*structure
            };
            *next_pointer = structure.next_pointer;
            structure
        })
    }
}

