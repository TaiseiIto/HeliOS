//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

pub mod arg_obj;
pub mod byte_data;
pub mod computational_data;
pub mod const_obj;
pub mod data_object;
pub mod def_field;
pub mod def_method;
pub mod def_op_region;
pub mod def_scope;
pub mod def_to_hex_string;
pub mod digit_char;
pub mod dual_name_prefix;
pub mod expression_opcode;
pub mod ext_op_prefix;
pub mod field_element;
pub mod field_flags;
pub mod field_list;
pub mod field_op;
pub mod lead_name_char;
pub mod local_obj;
pub mod method_flags;
pub mod method_op;
pub mod multi_name_prefix;
pub mod name_char;
pub mod name_path;
pub mod name_seg;
pub mod name_space_modifier_obj;
pub mod name_string;
pub mod named_field;
pub mod named_obj;
pub mod null_name;
pub mod object;
pub mod one_op;
pub mod op_region_op;
pub mod operand;
pub mod pkg_lead_byte;
pub mod pkg_length;
pub mod prefix_path;
pub mod region_len;
pub mod region_offset;
pub mod region_space;
pub mod root_char;
pub mod scope_op;
pub mod simple_name;
pub mod super_name;
pub mod target;
pub mod term_arg;
pub mod term_list;
pub mod term_obj;
pub mod to_hex_string_op;
pub mod word_const;
pub mod word_data;
pub mod word_prefix;

pub use {
    arg_obj::{
        ARG_OBJ_MAX,
        ARG_OBJ_MIN,
        ArgObj,
    },
    byte_data::ByteData,
    computational_data::ComputationalData,
    const_obj::ConstObj,
    data_object::DataObject,
    def_field::DefField,
    def_method::DefMethod,
    def_op_region::DefOpRegion,
    def_scope::DefScope,
    def_to_hex_string::DefToHexString,
    digit_char::DigitChar,
    dual_name_prefix::DUAL_NAME_PREFIX,
    expression_opcode::ExpressionOpcode,
    ext_op_prefix::{
        EXT_OP_PREFIX,
        ExtOpPrefix,
    },
    field_element::FieldElement,
    field_flags::FieldFlags,
    field_list::FieldList,
    field_op::{
        FIELD_OP,
        FieldOp,
    },
    lead_name_char::LeadNameChar,
    local_obj::{
        LOCAL_OBJ_MAX,
        LOCAL_OBJ_MIN,
        LocalObj,
    },
    method_flags::MethodFlags,
    method_op::{
        METHOD_OP,
        MethodOp,
    },
    multi_name_prefix::MULTI_NAME_PREFIX,
    name_char::NameChar,
    name_path::NamePath,
    name_seg::NameSeg,
    name_space_modifier_obj::NameSpaceModifierObj,
    name_string::NameString,
    named_field::NamedField,
    named_obj::NamedObj,
    null_name::{
        NULL_NAME,
        NullName,
    },
    object::Object,
    one_op::{
        ONE_OP,
        OneOp,
    },
    op_region_op::{
        OP_REGION_OP,
        OpRegionOp,
    },
    operand::Operand,
    pkg_lead_byte::PkgLeadByte,
    pkg_length::PkgLength,
    prefix_path::{
        PREFIX_PATH,
        PrefixPath,
    },
    region_len::RegionLen,
    region_offset::RegionOffset,
    region_space::RegionSpace,
    root_char::{
        ROOT_CHAR,
        RootChar,
    },
    scope_op::{
        SCOPE_OP,
        ScopeOp,
    },
    simple_name::SimpleName,
    super_name::SuperName,
    target::Target,
    term_arg::TermArg,
    term_list::TermList,
    term_obj::TermObj,
    to_hex_string_op::{
        TO_HEX_STRING_OP,
        ToHexStringOp,
    },
    word_const::WordConst,
    word_data::WordData,
    word_prefix::{
        WORD_PREFIX,
        WordPrefix,
    },
};

pub trait Reader<'a>: From<&'a [u8]> {
    fn length(&self) -> usize;

    fn read(aml: &'a [u8]) -> (Self, &'a [u8]) {
        let symbol: Self = aml.into();
        let aml: &[u8] = &aml[symbol.length()..];
        (symbol, aml)
    }
}

