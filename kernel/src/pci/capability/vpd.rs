use {
    super::{
        super::{Address, FunctionWithAddress},
        Header,
    },
    crate::x64,
    core::{fmt, mem},
};

pub mod address;
pub mod resource;

/// # VPD Capability Structure
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I. Vital Product Data. Figure I-1. VPD Capability Structure
#[repr(packed)]
pub struct Structure {
    header: Header,
    address: address::Register,
}

impl Structure {
    fn can_read_data(&self) -> bool {
        let address: address::Register = self.address;
        address.can_read_data()
    }
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

impl From<u32> for Structure {
    fn from(structure: u32) -> Self {
        let header = (structure & 0x0000ffff) as u16;
        let address = ((structure & 0xffff0000) >> u16::BITS) as u16;
        let header: Header = header.into();
        let address: address::Register = address.into();
        Self { header, address }
    }
}

impl From<Structure> for u32 {
    fn from(structure: Structure) -> Self {
        let Structure { header, address } = structure;
        let header: u16 = header.into();
        let header: u32 = header as u32;
        let address: u16 = address.into();
        let address: u32 = address as u32;
        (address << u16::BITS) | header
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

    pub fn resource_data_iterator(&'a self) -> ResourceDataIterator<'a> {
        self.dword_iterator()
            .byte_iterator()
            .resource_data_iterator()
    }

    fn address_address(&self) -> Address {
        let Self {
            function_with_address,
            structure_offset,
        } = self;
        let bus_number: u8 = function_with_address.bus_number();
        let device_number: u8 = function_with_address.device_number();
        let function_number: u8 = function_with_address.function_number();
        Address::create(
            bus_number,
            device_number,
            function_number,
            *structure_offset,
        )
    }

    fn data_address(&self) -> Address {
        self.address_address().add(mem::size_of::<Structure>())
    }

    fn dword_iterator(&'a self) -> DwordIterator<'a> {
        self.into()
    }

    fn read(&self, address: u16) -> u32 {
        assert_eq!((address as usize) % mem::size_of::<u32>(), 0);
        let mut structure: Structure = self.read_structure();
        structure.address = address::Register::read_address(address);
        self.write_structure(structure);
        while !self.read_structure().can_read_data() {
            x64::pause();
        }
        self.data_address().read()
    }

    fn read_structure(&self) -> Structure {
        let structure: u32 = self.address_address().read();
        structure.into()
    }

    fn write_structure(&self, structure: Structure) {
        let structure: u32 = structure.into();
        self.address_address().write(structure);
    }
}

impl fmt::Debug for StructureWithFunctionWithAddress<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_list()
            .entries(self.resource_data_iterator())
            .finish()
    }
}

pub struct DwordIterator<'a> {
    structure_with_function_with_address: &'a StructureWithFunctionWithAddress<'a>,
    address: u16,
}

impl<'a> DwordIterator<'a> {
    fn byte_iterator(self) -> ByteIterator<'a> {
        self.into()
    }
}

impl<'a> From<&'a StructureWithFunctionWithAddress<'a>> for DwordIterator<'a> {
    fn from(
        structure_with_function_with_address: &'a StructureWithFunctionWithAddress<'a>,
    ) -> Self {
        let address: u16 = 0;
        Self {
            structure_with_function_with_address,
            address,
        }
    }
}

impl Iterator for DwordIterator<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            structure_with_function_with_address,
            address,
        } = self;
        let item: u32 = structure_with_function_with_address.read(*address);
        *address += mem::size_of::<Self::Item>() as u16;
        Some(item)
    }
}

pub struct ByteIterator<'a> {
    dword_iterator: DwordIterator<'a>,
    address: u16,
    dword: Option<u32>,
}

impl<'a> ByteIterator<'a> {
    fn resource_data_iterator(self) -> ResourceDataIterator<'a> {
        self.into()
    }
}

impl<'a> From<DwordIterator<'a>> for ByteIterator<'a> {
    fn from(dword_iterator: DwordIterator<'a>) -> Self {
        let address: u16 = dword_iterator.address;
        let dword: Option<u32> = None;
        Self {
            dword_iterator,
            address,
            dword,
        }
    }
}

impl Iterator for ByteIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            dword_iterator,
            address,
            dword,
        } = self;
        if dword.is_none() {
            *dword = dword_iterator.next();
        }
        (*dword).map(|current_dword| {
            let offset_in_byte: usize = ((*address) as usize) % mem::size_of::<u32>();
            let offset_in_bit: usize = offset_in_byte * (u8::BITS as usize);
            let byte: u8 = ((current_dword >> offset_in_bit) & 0xff) as u8;
            *address += 1;
            if (*address as usize) % mem::size_of::<u32>() == 0 {
                *dword = None;
            }
            byte
        })
    }
}

pub struct ResourceDataIterator<'a> {
    byte_iterator: ByteIterator<'a>,
    reaches_end: bool,
}

impl<'a> From<ByteIterator<'a>> for ResourceDataIterator<'a> {
    fn from(byte_iterator: ByteIterator<'a>) -> Self {
        let reaches_end: bool = false;
        Self {
            byte_iterator,
            reaches_end,
        }
    }
}

impl Iterator for ResourceDataIterator<'_> {
    type Item = resource::Data;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            byte_iterator,
            reaches_end,
        } = self;
        let resource_data: Option<Self::Item> = (!*reaches_end)
            .then(|| Self::Item::from_byte_iterator(byte_iterator))
            .flatten();
        *reaches_end = resource_data.as_ref().map_or(true, |resource_data| {
            resource_data.header().tag() == resource::Tag::End
        });
        resource_data
    }
}
