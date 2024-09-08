//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

use {
    alloc::{
        collections::vec_deque::VecDeque,
        format,
        string::String,
        vec::Vec,
    },
    core::{
        fmt,
        iter,
        ops::Add,
    },
    crate::com2_println,
    super::syntax,
};

pub struct Node {
    name: Segment,
    object: Object,
    children: Vec<Self>,
}

impl Node {
    pub fn add_node(&mut self, path: &Path, object: Object) {
        let mut path: Path = path.clone();
        if let Some(name) = path.pop_first_segment() {
            match name {
                Segment::Child {
                    name: _,
                } => match self
                    .children
                    .iter_mut()
                    .find(|child| child.name == name) {
                    Some(child) => {
                        child.add_node(&path, object);
                    },
                    None => if path.is_empty() {
                        self.children.push(Self::new(name, object));
                    } else {
                        self.children.push(Self::new(name, Object::Scope));
                        self.add_node(&path, object);
                    },
                },
                Segment::Parent => unreachable!(),
                Segment::Root => {
                    assert_eq!(self.name, Segment::Root);
                    self.add_node(&path, object);
                },
            }
        }
    }

    pub fn new(name: Segment, object: Object) -> Self {
        let children: Vec<Self> = Vec::new();
        Self {
            name,
            object,
            children,
        }
    }

    pub fn number_of_arguments(&self, method: &Path) -> Option<usize> {
        com2_println!("method = {:#x?}", method);
        let mut method: Path = method.clone();
        match method.pop_first_segment() {
            Some(segment) => match segment {
                method_segment @ Segment::Child {
                    name: _,
                } => self
                    .children
                    .iter()
                    .find(|child| child.name == method_segment)
                    .and_then(|child| child.number_of_arguments(&method)),
                Segment::Parent => unreachable!(),
                Segment::Root => {
                    assert_eq!(self.name, Segment::Root);
                    self.number_of_arguments(&method)
                },
            },
            None => Some(self.object.number_of_arguments()),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        let name: Segment = Segment::Root;
        let children: Vec<Self> = Vec::default();
        let object: Object = Object::Scope;
        Self {
            name,
            object,
            children,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name,
            object,
            children,
        } = self;
        let name: String = name.into();
        let name: String = match object {
            Object::Method {
                number_of_arguments
            } => format!("Method {:#x?} number_of_arguments = {:#x?}", name, number_of_arguments),
            object => format!("{:#x?} {:#x?}", object, name),
        };
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple(name.as_str());
        children
            .iter()
            .for_each(|child| {
                debug_tuple.field(child);
            });
        debug_tuple.finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Object {
    Alias {
        original_path: Path,
    },
    CreateBitField,
    CreateByteField,
    CreateDWordField,
    CreateField,
    CreateQWordField,
    CreateWordField,
    DataRegion,
    Device,
    Event,
    External,
    Load,
    Method {
        number_of_arguments: usize,
    },
    Mutex,
    Name,
    NamedField,
    OpRegion,
    PowerRes,
    Processor,
    Scope,
    ThermalZone,
}

impl Object {
    pub fn alias(original_path: &Path) -> Self {
        let original_path: Path = original_path.clone();
        Self::Alias {
            original_path,
        }
    }

    pub fn method(number_of_arguments: usize) -> Self {
        Self::Method {
            number_of_arguments,
        }
    }

    pub fn number_of_arguments(&self) -> usize {
        match self {
            Self::Method {
                number_of_arguments,
            } => *number_of_arguments,
            _ => 0,
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::Scope
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Path {
    segments: VecDeque<Segment>,
}

impl Path {
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn pop_first_segment(&mut self) -> Option<Segment> {
        self.segments.pop_front()
    }

    pub fn pop_last_segment(&mut self) -> Option<Segment> {
        self.segments.pop_back()
    }

    pub fn root() -> Self {
        let segments: VecDeque<Segment> = iter::once(Segment::Root).collect();
        Self {
            segments,
        }
    }
}

impl Add for Path {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut segments: VecDeque<Segment> = VecDeque::new();
        self.segments
            .iter()
            .chain(other
                .segments
                .iter())
            .for_each(|segment| match segment {
                Segment::Child {
                    name: _,
                } => {
                    segments.push_back(segment.clone());
                },
                Segment::Parent => if segments.is_empty() {
                    segments.push_back(segment.clone());
                } else {
                    segments.pop_back();
                },
                Segment::Root => {
                    segments = iter::once(segment)
                        .cloned()
                        .collect();
                },
            });
        Self::Output {
            segments,
        }
    }
}

impl From<&syntax::NameSeg> for Path {
    fn from(name_seg: &syntax::NameSeg) -> Self {
        let segment: Segment = name_seg.into();
        let segments: VecDeque<Segment> = iter::once(segment).collect();
        Self {
            segments,
        }
    }
}

impl From<&syntax::NameString> for Path {
    fn from(name_string: &syntax::NameString) -> Self {
        let segments: VecDeque<Segment> = name_string.into();
        Self {
            segments,
        }
    }
}

impl From<&Node> for Path {
    fn from(node: &Node) -> Self {
        let Node {
            name,
            object: _,
            children: _,
        } = node;
        name.into()
    }
}

impl From<&Segment> for Path {
    fn from(segment: &Segment) -> Self {
        let segments: VecDeque<Segment> = iter::once(segment)
            .cloned()
            .collect();
        Self {
            segments,
        }
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            segments,
        } = self;
        let mut print_dot: bool = false;
        segments
            .iter()
            .map(|segment| match segment {
                Segment::Child {
                    name,
                } => {
                    if print_dot {
                        formatter.write_str(".")?;
                    }
                    print_dot = true;
                    formatter.write_str(name)
                },
                Segment::Parent => {
                    print_dot = false;
                    formatter.write_str("^")
                },
                Segment::Root => {
                    print_dot = false;
                    formatter.write_str("\\")
                },
            })
            .try_fold((), |_, result| result)
    }
}

pub struct RelativePath {
    current: Path,
    target: Path,
}

impl RelativePath {
    pub fn new(current: &Path, target: &Path) -> Self {
        let current: Path = current.clone();
        let target: Path = target.clone();
        Self {
            current,
            target,
        }
    }
}

impl Iterator for RelativePath {
    type Item = Path;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            current,
            target,
        } = self;
        let absolute_path: Path = current.clone() + target.clone();
        current
            .pop_last_segment()
            .map(|_| absolute_path)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Segment {
    Child {
        name: String,
    },
    Parent,
    Root,
}

impl From<&syntax::NameSeg> for Segment {
    fn from(name_seg: &syntax::NameSeg) -> Self {
        let name: String = name_seg.into();
        Self::Child {
            name,
        }
    }
}

impl From<&syntax::ParentPrefixChar> for Segment {
    fn from(_parent_prefix_char: &syntax::ParentPrefixChar) -> Self {
        Self::Parent
    }
}

impl From<&syntax::RootChar> for Segment {
    fn from(_root_char: &syntax::RootChar) -> Self {
        Self::Root
    }
}

impl From<&Segment> for String {
    fn from(segment: &Segment) -> Self {
        match segment {
            Segment::Child {
                name,
            } => name.clone(),
            Segment::Parent => Self::from("^"),
            Segment::Root => Self::from("\\"),
        }
    }
}

