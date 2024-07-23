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
pub mod def_size_of;
pub mod def_store;
pub mod def_subtract;
pub mod def_to_buffer;
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
pub mod size_of_op;
pub mod store_op;
pub mod subtract_op;
pub mod super_name;
pub mod target;
pub mod term_arg;
pub mod term_list;
pub mod term_obj;
pub mod to_buffer_op;
pub mod to_hex_string_op;
pub mod word_const;
pub mod word_data;
pub mod word_prefix;
pub mod zero_op;

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
    def_size_of::DefSizeOf,
    def_store::DefStore,
    def_subtract::DefSubtract,
    def_to_buffer::DefToBuffer,
    def_to_hex_string::DefToHexString,
    digit_char::DigitChar,
    expression_opcode::ExpressionOpcode,
    ext_op_prefix::ExtOpPrefix,
    field_element::FieldElement,
    field_flags::FieldFlags,
    field_list::FieldList,
    field_op::FieldOp,
    lead_name_char::LeadNameChar,
    local_obj::LocalObj,
    method_flags::MethodFlags,
    method_op::MethodOp,
    name_char::NameChar,
    name_path::NamePath,
    name_seg::NameSeg,
    name_space_modifier_obj::NameSpaceModifierObj,
    name_string::NameString,
    named_field::NamedField,
    named_obj::NamedObj,
    null_name::NullName,
    object::Object,
    one_op::OneOp,
    op_region_op::OpRegionOp,
    operand::Operand,
    pkg_lead_byte::PkgLeadByte,
    pkg_length::PkgLength,
    prefix_path::PrefixPath,
    region_len::RegionLen,
    region_offset::RegionOffset,
    region_space::RegionSpace,
    root_char::RootChar,
    scope_op::ScopeOp,
    simple_name::SimpleName,
    size_of_op::SizeOfOp,
    store_op::StoreOp,
    subtract_op::SubtractOp,
    super_name::SuperName,
    target::Target,
    term_arg::TermArg,
    term_list::TermList,
    term_obj::TermObj,
    to_buffer_op::ToBufferOp,
    to_hex_string_op::ToHexStringOp,
    word_const::WordConst,
    word_data::WordData,
    word_prefix::WordPrefix,
    zero_op::ZeroOp,
};

pub trait Reader<'a>: From<&'a [u8]> {
    fn length(&self) -> usize;
    fn matches(aml: &[u8]) -> bool;

    fn read(aml: &'a [u8]) -> (Self, &'a [u8]) {
        let symbol: Self = aml.into();
        let aml: &[u8] = &aml[symbol.length()..];
        (symbol, aml)
    }
}

