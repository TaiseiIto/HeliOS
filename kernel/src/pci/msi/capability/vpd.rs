pub mod address;

use {
    core::fmt,
    super::{
        Header,
        super::super::{
            Function,
            FunctionWithAddress,
        },
    },
};

/// # VPD Capability Structure
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I. Vital Product Data. Figure I-1. VPD Capability Structure
#[repr(packed)]
pub struct Structure {
    header: Header,
    address: address::Register,
    data: u32,
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let address: address::Register = self.address;
        let data: u32 = self.data;
        formatter
            .debug_struct("Structure")
            .field("header", &header)
            .field("address", &address)
            .field("data", &data)
            .finish()
    }
}

pub struct StructureWithFunctionWithAddress<'a> {
    function_with_address: &'a FunctionWithAddress<'a>,
    offset: u8,
}

impl<'a> StructureWithFunctionWithAddress<'a> {
    pub fn new(function_with_address: &'a FunctionWithAddress<'a>, offset: u8) -> Self {
        Self {
            function_with_address,
            offset,
        }
    }
}

impl fmt::Debug for StructureWithFunctionWithAddress<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            function_with_address,
            offset,
        } = self;
        let function: &Function = function_with_address.function();
        let function_address: *const Function = function as *const Function;
        let function_address: usize = function_address as usize;
        let offset: usize = (*offset) as usize;
        let structure: usize = function_address + offset;
        let structure: *const Structure = structure as *const Structure;
        let structure: &Structure = unsafe {
            &*structure
        };
        structure.fmt(formatter)
    }
}

