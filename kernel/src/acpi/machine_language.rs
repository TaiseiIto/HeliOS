//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

pub mod def_scope;
pub mod name_space_modifier_obj;
pub mod object;
pub mod scope_op;
pub mod term_list;
pub mod term_obj;

pub use {
    def_scope::DefScope,
    name_space_modifier_obj::NameSpaceModifierObj,
    object::Object,
    scope_op::ScopeOp,
    term_list::TermList,
    term_obj::TermObj,
};

