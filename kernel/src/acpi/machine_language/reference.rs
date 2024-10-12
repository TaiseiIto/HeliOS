//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

use {
    alloc::{
        format,
        string::String,
        vec::Vec,
    },
    core::fmt,
    super::{
        name,
        syntax,
    },
};

#[derive(Default)]
pub struct Nodes<'a>(Vec<Node<'a>>);

impl<'a> Nodes<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &Node<'a>> {
        let Self(nodes) = self;
        nodes.iter()
    }
}

impl fmt::Debug for Nodes<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_list()
            .entries(self.iter())
            .finish()
    }
}

pub struct Node<'a> {
    name: name::Segment,
    object: Object<'a>,
    children: Nodes<'a>,
}

impl fmt::Debug for Node<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name,
            object,
            children,
        } = self;
        let name: String = name.into();
        let object: &str = object.type_name();
        let header: String = format!("{} {}", object, name);
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple(header.as_str());
        children
            .iter()
            .for_each(|child| {
                debug_tuple.field(child);
            });
        debug_tuple.finish()
    }
}

pub enum Object<'a> {
    Alias(&'a syntax::DefAlias),
    CreateBitField(&'a syntax::DefCreateBitField),
    CreateByteField(&'a syntax::DefCreateByteField),
    CreateDWordField(&'a syntax::DefCreateDWordField),
    CreateField(&'a syntax::DefCreateField),
    CreateQWordField(&'a syntax::DefCreateQWordField),
    CreateWordField(&'a syntax::DefCreateWordField),
    DataRegion(&'a syntax::DefDataRegion),
    Device(&'a syntax::DefDevice),
    Event(&'a syntax::DefEvent),
    External(&'a syntax::DefExternal),
    Load(&'a syntax::DefLoad),
    Method(&'a syntax::DefMethod),
    Mutex(&'a syntax::DefMutex),
    Name(&'a syntax::DefName),
    NamedField(&'a syntax::NamedField),
    OpRegion(&'a syntax::DefOpRegion),
    PowerRes(&'a syntax::DefPowerRes),
    Processor(&'a syntax::DefProcessor),
    Scope(&'a syntax::DefScope),
    ThermalZone(&'a syntax::DefThermalZone),
}

impl Object<'_> {
    fn type_name(&self) -> &str {
        match self {
            Self::Alias(_) => "Alias",
            Self::CreateBitField(_) => "CreateBitField",
            Self::CreateByteField(_) => "CreateByteField",
            Self::CreateDWordField(_) => "CreateDWordField",
            Self::CreateField(_) => "CreateField",
            Self::CreateQWordField(_) => "CreateQWordField",
            Self::CreateWordField(_) => "CreateWordField",
            Self::DataRegion(_) => "DataRegion",
            Self::Device(_) => "Device",
            Self::Event(_) => "Event",
            Self::External(_) => "External",
            Self::Load(_) => "Load",
            Self::Method(_) => "Method",
            Self::Mutex(_) => "Mutex",
            Self::Name(_) => "Name",
            Self::NamedField(_) => "NamedField",
            Self::OpRegion(_) => "OpRegion",
            Self::PowerRes(_) => "PowerRes",
            Self::Processor(_) => "Processor",
            Self::Scope(_) => "Scope",
            Self::ThermalZone(_) => "ThermalZone",
        }
    }
}

