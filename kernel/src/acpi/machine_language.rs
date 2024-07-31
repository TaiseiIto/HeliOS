//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

pub mod acquire_op;
pub mod arg_obj;
pub mod arg_object;
pub mod ascii_char;
pub mod ascii_char_list;
pub mod buff_pkg_str_obj;
pub mod buffer_op;
pub mod buffer_size;
pub mod byte_const;
pub mod byte_data;
pub mod byte_list;
pub mod byte_prefix;
pub mod computational_data;
pub mod const_obj;
pub mod d_word_const;
pub mod d_word_data;
pub mod d_word_prefix;
pub mod data_object;
pub mod data_ref_object;
pub mod def_acquire;
pub mod def_buffer;
pub mod def_deref_of;
pub mod def_device;
pub mod def_else;
pub mod def_field;
pub mod def_if_else;
pub mod def_increment;
pub mod def_index;
pub mod def_l_equal;
pub mod def_l_less;
pub mod def_l_not;
pub mod def_method;
pub mod def_mutex;
pub mod def_name;
pub mod def_op_region;
pub mod def_package;
pub mod def_return;
pub mod def_scope;
pub mod def_size_of;
pub mod def_store;
pub mod def_subtract;
pub mod def_to_buffer;
pub mod def_to_hex_string;
pub mod def_while;
pub mod deref_of_op;
pub mod device_op;
pub mod digit_char;
pub mod dual_name_path;
pub mod dual_name_prefix;
pub mod else_op;
pub mod expression_opcode;
pub mod ext_op_prefix;
pub mod field_element;
pub mod field_flags;
pub mod field_list;
pub mod field_op;
pub mod if_op;
pub mod increment_op;
pub mod index_op;
pub mod index_value;
pub mod l_equal_op;
pub mod l_less_op;
pub mod l_not_op;
pub mod lead_name_char;
pub mod local_obj;
pub mod method_flags;
pub mod method_op;
pub mod mutex_op;
pub mod name_char;
pub mod name_op;
pub mod name_path;
pub mod name_seg;
pub mod name_space_modifier_obj;
pub mod name_string;
pub mod named_field;
pub mod named_obj;
pub mod null_char;
pub mod null_name;
pub mod num_elements;
pub mod obj_reference;
pub mod object;
pub mod one_op;
pub mod op_region_op;
pub mod operand;
pub mod package_element;
pub mod package_element_list;
pub mod package_op;
pub mod pkg_lead_byte;
pub mod pkg_length;
pub mod predicate;
pub mod prefix_path;
pub mod reference_type_opcode;
pub mod region_len;
pub mod region_offset;
pub mod region_space;
pub mod return_op;
pub mod root_char;
pub mod scope_op;
pub mod simple_name;
pub mod size_of_op;
pub mod statement_opcode;
pub mod store_op;
pub mod string;
pub mod string_prefix;
pub mod subtract_op;
pub mod super_name;
pub mod sync_flags;
pub mod target;
pub mod term_arg;
pub mod term_list;
pub mod term_obj;
pub mod to_buffer_op;
pub mod to_hex_string_op;
pub mod while_op;
pub mod word_const;
pub mod word_data;
pub mod word_prefix;
pub mod zero_op;

pub use {
    acquire_op::AcquireOp,
    arg_obj::ArgObj,
    arg_object::ArgObject,
    ascii_char::AsciiChar,
    ascii_char_list::AsciiCharList,
    buff_pkg_str_obj::BuffPkgStrObj,
    buffer_op::BufferOp,
    buffer_size::BufferSize,
    byte_const::ByteConst,
    byte_data::ByteData,
    byte_list::ByteList,
    byte_prefix::BytePrefix,
    computational_data::ComputationalData,
    const_obj::ConstObj,
    d_word_const::DWordConst,
    d_word_data::DWordData,
    d_word_prefix::DWordPrefix,
    data_object::DataObject,
    data_ref_object::DataRefObject,
    def_acquire::DefAcquire,
    def_buffer::DefBuffer,
    def_deref_of::DefDerefOf,
    def_device::DefDevice,
    def_else::DefElse,
    def_field::DefField,
    def_if_else::DefIfElse,
    def_increment::DefIncrement,
    def_index::DefIndex,
    def_l_equal::DefLEqual,
    def_l_less::DefLLess,
    def_l_not::DefLNot,
    def_method::DefMethod,
    def_mutex::DefMutex,
    def_name::DefName,
    def_op_region::DefOpRegion,
    def_package::DefPackage,
    def_return::DefReturn,
    def_scope::DefScope,
    def_size_of::DefSizeOf,
    def_store::DefStore,
    def_subtract::DefSubtract,
    def_to_buffer::DefToBuffer,
    def_to_hex_string::DefToHexString,
    def_while::DefWhile,
    deref_of_op::DerefOfOp,
    device_op::DeviceOp,
    digit_char::DigitChar,
    dual_name_path::DualNamePath,
    dual_name_prefix::DualNamePrefix,
    else_op::ElseOp,
    expression_opcode::ExpressionOpcode,
    ext_op_prefix::ExtOpPrefix,
    field_element::FieldElement,
    field_flags::FieldFlags,
    field_list::FieldList,
    field_op::FieldOp,
    if_op::IfOp,
    increment_op::IncrementOp,
    index_op::IndexOp,
    index_value::IndexValue,
    l_equal_op::LEqualOp,
    l_less_op::LLessOp,
    l_not_op::LNotOp,
    lead_name_char::LeadNameChar,
    local_obj::LocalObj,
    method_flags::MethodFlags,
    method_op::MethodOp,
    mutex_op::MutexOp,
    name_char::NameChar,
    name_op::NameOp,
    name_path::NamePath,
    name_seg::NameSeg,
    name_space_modifier_obj::NameSpaceModifierObj,
    name_string::NameString,
    named_field::NamedField,
    named_obj::NamedObj,
    null_char::NullChar,
    null_name::NullName,
    num_elements::NumElements,
    obj_reference::ObjReference,
    object::Object,
    one_op::OneOp,
    op_region_op::OpRegionOp,
    operand::Operand,
    package_element::PackageElement,
    package_element_list::PackageElementList,
    package_op::PackageOp,
    pkg_lead_byte::PkgLeadByte,
    pkg_length::PkgLength,
    predicate::Predicate,
    prefix_path::PrefixPath,
    reference_type_opcode::ReferenceTypeOpcode,
    region_len::RegionLen,
    region_offset::RegionOffset,
    region_space::RegionSpace,
    return_op::ReturnOp,
    root_char::RootChar,
    scope_op::ScopeOp,
    simple_name::SimpleName,
    size_of_op::SizeOfOp,
    statement_opcode::StatementOpcode,
    store_op::StoreOp,
    string::String,
    string_prefix::StringPrefix,
    subtract_op::SubtractOp,
    super_name::SuperName,
    sync_flags::SyncFlags,
    target::Target,
    term_arg::TermArg,
    term_list::TermList,
    term_obj::TermObj,
    to_buffer_op::ToBufferOp,
    to_hex_string_op::ToHexStringOp,
    while_op::WhileOp,
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

