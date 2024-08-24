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
    super::{
        syntax,
        syntax::SemanticAnalyzer,
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
        let mut current: Path = (&root).into();
        term_list.analyze_semantics(&mut root, &mut current);
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

#[derive(Clone)]
pub enum Segment {
    Child {
        name: String,
    },
    Parent,
    Root,
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

