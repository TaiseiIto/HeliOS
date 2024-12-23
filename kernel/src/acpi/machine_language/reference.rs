//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

use {
    alloc::{
        format,
        string::String,
        vec::Vec,
    },
    core::{
        fmt,
        ops::Range,
    },
    super::{
        name,
        syntax,
        syntax::Lender,
        interpreter,
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

    pub fn get_method(&self, method: &name::Path) -> Option<&'a syntax::DefMethod> {
        let mut methods: Vec<&'a syntax::DefMethod> = self.get_methods(method);
        let method: Option<&'a syntax::DefMethod> = methods.pop();
        if methods.is_empty() {
            method
        } else {
            None
        }
    }

    pub fn get_method_from_current(&self, method: &name::AbsolutePath) -> Option<(name::Path, &'a syntax::DefMethod)> {
        self.get_methods_from_current(method)
            .and_then(|(method_path, mut methods)| methods
                .pop()
                .and_then(|method| if methods.is_empty() {
                    Some((method_path, method))
                } else {
                    None
                }))
    }

    pub fn get_name(&self, name: &name::Path) -> Option<&'a syntax::DefName> {
        let mut names: Vec<&'a syntax::DefName> = self.get_names(name);
        let name: Option<&'a syntax::DefName> = names.pop();
        if names.is_empty() {
            name
        } else {
            None
        }
    }

    pub fn get_name_from_current(&self, name: &name::AbsolutePath) -> Option<(name::Path, &'a syntax::DefName)> {
        self.get_names_from_current(name)
            .and_then(|(name_path, mut names)| names
                .pop()
                .and_then(|name| if names.is_empty() {
                    Some((name_path, name))
                } else {
                    None
                }))
    }

    pub fn read_named_field(&self, stack_frame: &mut interpreter::StackFrame, root: &Node, name: &name::AbsolutePath) -> Option<interpreter::Value> {
        self.get_objects_from_current(name)
            .and_then(|(named_field_path, objects)| objects
                .iter()
                .find_map(|object| match object {
                    Object::NamedField {
                        access_type,
                        named_field,
                        offset_in_bits,
                        op_region,
                    } => {
                        let size_in_bits: usize = named_field.bits();
                        let op_region = name::AbsolutePath::new(&named_field_path, op_region);
                        self.get_objects_from_current(&op_region)
                            .and_then(|(op_region_path, objects)| objects
                                .iter()
                                .find_map(|object| match object {
                                    Object::OpRegion(op_region) => op_region.read_value(stack_frame, root, &op_region_path, *offset_in_bits, size_in_bits, access_type),
                                    _ => None,
                                }))
                    },
                    _ => None,
                }))
    }

    pub fn write_named_field(&self, value: interpreter::Value, stack_frame: &mut interpreter::StackFrame, root: &Node, name: &name::AbsolutePath) -> Option<interpreter::Value> {
        self.get_objects_from_current(name)
            .and_then(|(named_field_path, objects)| objects
                .iter()
                .find_map(|object| match object {
                    Object::NamedField {
                        access_type,
                        named_field,
                        offset_in_bits,
                        op_region,
                    } => {
                        let start_bit: usize = *offset_in_bits;
                        let size_in_bits: usize = named_field.bits();
                        let end_bit: usize = start_bit + size_in_bits;
                        let bit_range: Range<usize> = start_bit..end_bit;
                        let op_region = name::AbsolutePath::new(&named_field_path, op_region);
                        self.get_objects_from_current(&op_region)
                            .and_then(|(op_region_path, objects)| objects
                                .iter()
                                .find_map(|object| match object {
                                    Object::OpRegion(op_region) => op_region.write_value(value.clone(), stack_frame, root, &op_region_path, &bit_range, access_type),
                                    _ => None,
                                }))
                    },
                    _ => None,
                }))
    }

    fn get_methods(&self, method: &name::Path) -> Vec<&'a syntax::DefMethod> {
        match self.get_objects(method) {
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

    fn get_methods_from_current(&self, method: &name::AbsolutePath) -> Option<(name::Path, Vec<&'a syntax::DefMethod>)> {
        self.get_objects_from_current(method)
            .map(|(method_path, objects)| {
                let methods: Vec<&'a syntax::DefMethod> = objects
                    .iter()
                    .filter_map(|object| match object {
                        Object::Method(method) => Some(*method),
                        _ => None,
                    })
                    .collect();
                (method_path, methods)
            })
    }

    fn get_names(&self, name: &name::Path) -> Vec<&'a syntax::DefName> {
        match self.get_objects(name) {
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

    fn get_names_from_current(&self, name: &name::AbsolutePath) -> Option<(name::Path, Vec<&'a syntax::DefName>)> {
        self.get_objects_from_current(name)
            .map(|(name_path, objects)| {
                let names: Vec<&'a syntax::DefName> = objects
                    .iter()
                    .filter_map(|object| match object {
                        Object::Name(name) => Some(*name),
                        _ => None,
                    })
                    .collect();
                (name_path, names)
            })
    }

    fn get_objects(&self, object: &name::Path) -> Option<&[Object<'a>]> {
        let mut object: name::Path = object.clone();
        match object.pop_first_segment() {
            Some(name) => match name {
                name::Segment::Child {
                    name: _,
                } => self
                    .children
                    .iter()
                    .find(|child| child.name == name)
                    .and_then(|child| child.get_objects(&object)),
                name::Segment::Parent => unreachable!(),
                name::Segment::Root => {
                    assert_eq!(self.name, name::Segment::Root);
                    self.get_objects(&object)
                },
            },
            None => Some(&self.objects),
        }
    }

    fn get_objects_from_current(&self, name: &name::AbsolutePath) -> Option<(name::Path, &[Object<'a>])> {
        let mut name: name::AbsolutePath = self.original_path(name);
        name
            .find_map(|name| self
                .get_objects(&name)
                .map(|object| (name, object)))
    }

    fn original_path(&self, alias: &name::AbsolutePath) -> name::AbsolutePath {
        match self.solve_alias_from_current(alias) {
            Some(alias) => self.original_path(&alias),
            None => alias.clone(),
        }
    }

    fn solve_alias(&self, alias: &name::Path) -> Option<name::AbsolutePath> {
        self.get_objects(alias)
            .and_then(|objects| objects
                .iter()
                .find_map(|object| object.solve_alias(alias)))
    }

    fn solve_alias_from_current(&self, alias: &name::AbsolutePath) -> Option<name::AbsolutePath> {
        let mut alias: name::AbsolutePath = alias.clone();
        alias.find_map(|alias| self.solve_alias(&alias))
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
    NamedField {
        access_type: interpreter::AccessType,
        named_field: &'a syntax::NamedField,
        offset_in_bits: usize,
        op_region: name::Path,
    },
    OpRegion(&'a syntax::DefOpRegion),
    PowerRes(&'a syntax::DefPowerRes),
    Processor(&'a syntax::DefProcessor),
    Scope(&'a syntax::DefScope),
    ThermalZone(&'a syntax::DefThermalZone),
}

impl Object<'_> {
    fn solve_alias(&self, current: &name::Path) -> Option<name::AbsolutePath> {
        if let Self::Alias(def_alias) = self {
            Some(def_alias.solve(current))
        } else {
            None
        }
    }
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
            Self::NamedField {
                access_type: _,
                named_field: _,
                offset_in_bits: _,
                op_region: _,
            } => "NamedField",
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
        let type_name: &str = self.type_name();
        match self {
            Self::NamedField {
                access_type,
                named_field: _,
                offset_in_bits,
                op_region,
            } => formatter
                .debug_struct(type_name)
                .field("access_type", access_type)
                .field("offset_in_bits", offset_in_bits)
                .field("op_region", op_region)
                .finish(),
            _ => formatter.write_str(type_name),
        }
    }
}

