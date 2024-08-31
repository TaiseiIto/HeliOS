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
    super::syntax::{
        SemanticAnalyzer,
        self,
    },
};

pub struct Node {
    name: Segment,
    object: Object,
    children: Vec<Self>,
}

impl Node {
    pub fn add_node(&mut self, mut path: Path, object: Object) {
        if let Some(name) = path.pop_first_segment() {
            match name {
                Segment::Child {
                    name: _,
                } => match self
                    .children
                    .iter_mut()
                    .find(|child| child.name == name) {
                    Some(child) => {
                        child.add_node(path, object);
                    },
                    None => if path.is_empty() {
                        self.children.push(Self::new(name, object));
                    } else {
                        self.children.push(Self::new(name, Object::Scope));
                        self.add_node(path, object);
                    },
                },
                Segment::Parent => unreachable!(),
                Segment::Root => {
                    assert_eq!(self.name, Segment::Root);
                    self.add_node(path, object);
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

impl From<&syntax::TermList> for Node {
    fn from(term_list: &syntax::TermList) -> Self {
        let mut root: Self = Self::default();
        let current: Path = Path::default();
        term_list.analyze_semantics(&mut root, current);
        root
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
    Alias,
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
        number_of_arguments: u8,
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
    pub fn def_method(number_of_arguments: u8) -> Self {
        Self::Method {
            number_of_arguments,
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::Scope
    }
}

#[derive(Clone, Default)]
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
                segment @ Segment::Child {
                    name: _,
                } => {
                    segments.push_back(segment.clone());
                },
                segment @ Segment::Parent => {
                    if segments.is_empty() {
                        segments.push_back(segment.clone());
                    } else {
                        segments.pop_back();
                    }
                },
                segment @ Segment::Root => {
                    segments = VecDeque::from([segment.clone()]);
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

