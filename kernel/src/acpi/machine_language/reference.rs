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
        syntax::Lender,
    },
};

pub struct Node<'a> {
    name: name::Segment,
    objects: Vec<Object<'a>>,
    children: Vec<Self>,
}

impl<'a> Node<'a> {
    pub fn add_node(&mut self, path: &name::Path, object: Object<'a>) {
        let mut path: name::Path = path.clone();
        match path.pop_first_segment() {
            Some(name) => match name {
                name::Segment::Child {
                    name: _,
                } => match self
                    .children
                    .iter_mut()
                    .find(|child| child.name == name) {
                    Some(child) => {
                        child.add_node(&path, object);
                    },
                    None => {
                        let objects: Vec<Object<'a>> = Vec::default();
                        let children: Vec<Self> = Vec::default();
                        let mut child = Self {
                            name,
                            objects,
                            children,
                        };
                        child.add_node(&path, object);
                        self.children.push(child);
                    },
                },
                name::Segment::Parent => unreachable!(),
                name::Segment::Root => {
                    assert_eq!(self.name, name::Segment::Root);
                    self.add_node(&path, object);
                },
            },
            None => {
                self.objects.push(object);
            },
        }
    }

    pub fn get_method(&self, path: &name::Path) -> Option<&'a syntax::DefMethod> {
        let mut methods: Vec<&'a syntax::DefMethod> = self.get_methods(path);
        let method: Option<&'a syntax::DefMethod> = methods.pop();
        if methods.is_empty() {
            method
        } else {
            None
        }
    }

    pub fn get_name(&self, path: &name::Path) -> Option<&'a syntax::DefName> {
        let mut names: Vec<&'a syntax::DefName> = self.get_names(path);
        let name: Option<&'a syntax::DefName> = names.pop();
        if names.is_empty() {
            name
        } else {
            None
        }
    }

    fn get_methods(&self, path: &name::Path) -> Vec<&'a syntax::DefMethod> {
        match self.get_objects(path) {
            Some(objects) => objects
                .iter()
                .filter_map(|object| match object {
                    Object::Method(method) => Some(*method),
                    _ => None,
                })
                .collect(),
            None => Vec::new(),
        }
    }

    fn get_names(&self, path: &name::Path) -> Vec<&'a syntax::DefName> {
        match self.get_objects(path) {
            Some(objects) => objects
                .iter()
                .filter_map(|object| match object {
                    Object::Name(name) => Some(*name),
                    _ => None,
                })
                .collect(),
            None => Vec::new(),
        }
    }

    fn get_objects(&self, path: &name::Path) -> Option<&[Object<'a>]> {
        let mut path: name::Path = path.clone();
        match path.pop_first_segment() {
            Some(name) => match name {
                name::Segment::Child {
                    name: _,
                } => self
                    .children
                    .iter()
                    .find(|child| child.name == name)
                    .and_then(|child| child.get_objects(&path)),
                name::Segment::Parent => unreachable!(),
                name::Segment::Root => {
                    assert_eq!(self.name, name::Segment::Root);
                    self.get_objects(&path)
                },
            },
            None => Some(&self.objects),
        }
    }
}

impl<'a> From<&'a syntax::TermList> for Node<'a> {
    fn from(term_list: &'a syntax::TermList) -> Self {
        let name: name::Segment = name::Segment::Root;
        let objects: Vec<Object<'a>> = Vec::default();
        let children: Vec<Self> = Vec::default();
        let mut node = Self {
            name,
            objects,
            children,
        };
        let current = name::Path::root();
        term_list.lend(&mut node, &current);
        node
    }
}

impl fmt::Debug for Node<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name,
            objects,
            children,
        } = self;
        let name: String = name.into();
        let objects: Vec<&str> = objects
            .iter()
            .map(|object| object.type_name())
            .collect();
        let name: String = format!("{:#x?} {}", name, objects.join(" "));
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple(name.as_str());
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

impl fmt::Debug for Object<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.type_name())
    }
}
