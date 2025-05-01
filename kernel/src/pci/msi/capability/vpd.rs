pub mod address;

use {
    core::{
        fmt,
        mem,
    },
    super::{
        Header,
        super::super::{
            Address,
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
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let address: address::Register = self.address;
        formatter
            .debug_struct("Structure")
            .field("header", &header)
            .field("address", &address)
            .finish()
    }
}

pub struct StructureWithFunctionWithAddress<'a> {
    function_with_address: &'a FunctionWithAddress<'a>,
    structure_offset: u8,
}

impl<'a> StructureWithFunctionWithAddress<'a> {
    pub fn new(function_with_address: &'a FunctionWithAddress<'a>, structure_offset: u8) -> Self {
        Self {
            function_with_address,
            structure_offset,
        }
    }

    fn address_address(&self) -> Address {
        let Self {
            function_with_address,
            structure_offset,
        } = self;
        let bus_number: u8 = function_with_address.bus_number();
        let device_number: u8 = function_with_address.device_number();
        let function_number: u8 = function_with_address.function_number();
        Address::create(bus_number, device_number, function_number, *structure_offset)
    }

    fn data_address(&self) -> Address {
        self.address_address().add(mem::size_of::<Structure>())
    }
}

impl fmt::Debug for StructureWithFunctionWithAddress<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            function_with_address,
            structure_offset,
        } = self;
        let function: &Function = function_with_address.function();
        let function_address: *const Function = function as *const Function;
        let function_address: usize = function_address as usize;
        let structure_offset: usize = (*structure_offset) as usize;
        let structure: usize = function_address + structure_offset;
        let structure: *const Structure = structure as *const Structure;
        let structure: &Structure = unsafe {
            &*structure
        };
        structure.fmt(formatter)
    }
}

