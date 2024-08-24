//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

use {
    alloc::{
        string::String,
        vec,
        vec::Vec,
    },
    core::fmt,
    super::syntax::{
        SemanticAnalyzer,
        self,
    },
};

pub struct Node {
    name: Segment,
    children: Vec<Self>,
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
    segments: Vec<Segment>,
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

impl From<&Segment> for Path {
    fn from(segment: &Segment) -> Self {
        let segments: Vec<Segment> = vec![segment.clone()];
        Self {
            segments,
        }
    }
}

impl From<&str> for Path {
    fn from(path: &str) -> Self {
        let mut segments: Vec<Segment> = Vec::new();
        let mut name: String = String::new();
        path.chars()
            .for_each(|character| match character {
                '\\' => {
                    segments.push(Segment::Root);
                    name = String::new();
                },
                '^' => {
                    segments.push(Segment::Parent);
                    name = String::new();
                },
                '.' => {
                    let segment: Segment = name
                        .as_str()
                        .into();
                    segments.push(segment);
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
            segments.push(segment);
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

#[derive(Clone)]
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

