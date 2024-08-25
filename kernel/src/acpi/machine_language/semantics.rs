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
            if name == Segment::Root {
                assert_eq!(self.name, Segment::Root);
                self.add_node(path, object);
            } else {
                match self
                    .children
                    .iter_mut()
                    .find(|child| child.name == name) {
                    Some(child) => {
                        child.add_node(path, object);
                    },
                    None => if path.is_empty() {
                        self.children.push(Self::new(name, object));
                    } else {
                        self.children.push(Self::new(name, Object::DefScope));
                        self.add_node(path, object);
                    },
                }
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
        let object: Object = Object::DefScope;
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
        let current: Path = (&root).into();
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
            Object::DefMethod {
                number_of_arguments
            } => format!("DefMethod {:#x?} number_of_arguments = {:#x?}", name, number_of_arguments),
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

#[derive(Clone, Debug)]
pub enum Object {
    DefAlias,
    DefBankField,
    DefCreateBitField,
    DefCreateByteField,
    DefCreateDWordField,
    DefCreateField,
    DefCreateQWordField,
    DefCreateWordField,
    DefDataRegion,
    DefDevice,
    DefEvent,
    DefExternal,
    DefField,
    DefIndexField,
    DefLoad,
    DefMethod {
        number_of_arguments: u8,
    },
    DefMutex,
    DefName,
    DefOpRegion,
    DefPowerRes,
    DefProcessor,
    DefScope,
    DefThermalZone,
}

impl Object {
    pub fn def_method(number_of_arguments: u8) -> Self {
        Self::DefMethod {
            number_of_arguments,
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::DefScope
    }
}

#[derive(Clone)]
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

impl From<&Segment> for Path {
    fn from(segment: &Segment) -> Self {
        let segments: VecDeque<Segment> = VecDeque::from([segment.clone()]);
        Self {
            segments,
        }
    }
}

impl From<&str> for Path {
    fn from(path: &str) -> Self {
        let mut segments: VecDeque<Segment> = VecDeque::new();
        let mut name: String = String::new();
        path.chars()
            .for_each(|character| match character {
                '\\' => {
                    segments = VecDeque::from([Segment::Root]);
                    name = String::new();
                },
                '^' => {
                    if segments.is_empty() {
                        segments.push_back(Segment::Parent);
                    } else {
                        segments.pop_back();
                    }
                    name = String::new();
                },
                '.' => {
                    let segment: Segment = name
                        .as_str()
                        .into();
                    segments.push_back(segment);
                    name = String::new();
                },
                character => {
                    name.push(character);
                },
            });
        if !name.is_empty() {
            let segment: Segment = name
                .as_str()
                .into();
            segments.push_back(segment);
        }
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

impl From<&str> for Segment {
    fn from(segment: &str) -> Self {
        match segment {
            "^" => Self::Parent,
            "\\" => Self::Root,
            name => {
                let name: String = String::from(name);
                Self::Child {
                    name,
                }
            },
        }
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

