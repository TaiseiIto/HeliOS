//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

use {
    alloc::{
        string::String,
        vec::Vec,
    },
    core::fmt,
    super::syntax::TermList,
};

pub struct Node {
    name: String,
    children: Vec<Self>,
}

impl Default for Node {
    fn default() -> Self {
        let name: String = String::from("\\");
        let children: Vec<Self> = Vec::default();
        Self {
            name,
            children,
        }
    }
}

impl From<&TermList> for Node {
    fn from(term_list: &TermList) -> Self {
        Self::default()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name,
            children,
        } = self;
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple(name);
        children
            .iter()
            .for_each(|child| {
                debug_tuple.field(child);
            });
        debug_tuple.finish()
    }
}

