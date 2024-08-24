//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

use {
    alloc::{
        collections::vec_deque::VecDeque,
        string::String,
        vec,
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
    children: Vec<Self>,
}

impl Node {
    pub fn add_path(&mut self, mut path: Path) {
        if let Some(name) = path.pop_first_segment() {
            if name == Segment::Root {
                assert_eq!(self.name, Segment::Root);
                self.add_path(path);
            } else {
                match self
                    .children
                    .iter_mut()
                    .find(|child| child.name == name) {
                    Some(child) => {
                        child.add_path(path);
                    },
                    None => {
                        let child: Self = (&name).into();
                        self.children.push(child);
                        self.add_path(path);
                    },
                }
            }
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        let name: Segment = Segment::Root;
        let children: Vec<Self> = Vec::default();
        Self {
            name,
            children,
        }
    }
}

impl From<&Segment> for Node {
    fn from(name: &Segment) -> Self {
        let name: Segment = name.clone();
        let children: Vec<Self> = Vec::new();
        Self {
            name,
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
            children,
        } = self;
        let name: String = name.into();
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple(&name);
        children
            .iter()
            .for_each(|child| {
                debug_tuple.field(child);
            });
        debug_tuple.finish()
    }
}

#[derive(Clone)]
pub struct Path {
    segments: VecDeque<Segment>,
}

impl Path {
    pub fn pop_first_segment(&mut self) -> Option<Segment> {
        self.segments.pop_front()
    }
}

impl From<&Node> for Path {
    fn from(node: &Node) -> Self {
        let Node {
            name,
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
            .for_each(|segment| match segment {
                Segment::Child {
                    name,
                } => {
                    if print_dot {
                        formatter.write_str(".");
                    }
                    formatter.write_str(name);
                    print_dot = true;
                },
                Segment::Parent => {
                    formatter.write_str("^");
                    print_dot = false;
                },
                Segment::Root => {
                    formatter.write_str("\\");
                    print_dot = false;
                },
            });
        Ok(())
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

