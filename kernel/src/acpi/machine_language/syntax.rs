//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

use {
    alloc::{
        boxed::Box,
        collections::{
            btree_map::BTreeMap,
            vec_deque::VecDeque,
        },
        string::String,
        vec::Vec,
    },
    bitfield_struct::bitfield,
    core::{
        fmt,
        iter,
    },
    crate::com2_println,
    super::{
        interpreter::{
            Evaluator,
            Holder,
            self,
        },
        name,
        reference,
    },
};

pub trait Analyzer: FirstReader + Lender + Matcher + PathGetter + Reader + ReaderInsideMethod + ReaderOutsideMethod + ReferenceToSymbolIterator + WithLength {
}

pub trait FirstReader {
    fn first_read<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) where Self: Sized;
}

pub trait Lender {
    fn lend<'a>(&'a self, root: &mut reference::Node<'a>, current: &name::Path);
}

pub trait Matcher {
    fn matches(aml: &[u8]) -> bool where Self: Sized;
}

pub trait PathGetter {
    fn get_path(&self) -> Option<name::Path>;
}

pub trait Reader {
    fn read(aml: &[u8]) -> (Self, &[u8]) where Self: Sized;
}

pub trait ReaderInsideMethod {
    fn read_inside_method<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) where Self: Sized;
}

pub trait ReaderOutsideMethod {
    fn read_outside_method(&mut self, root: &mut name::Node, current: &name::Path);
}

pub trait ReferenceToSymbolIterator {
    fn iter(&self) -> SymbolIterator<'_>;
    fn iter_mut(&mut self) -> MutSymbolIterator<'_>;
}

pub trait WithLength {
    fn length(&self) -> usize;
}

pub struct SymbolIterator<'a> {
    symbols: VecDeque<&'a dyn Analyzer>,
}

impl<'a> Iterator for SymbolIterator<'a> {
    type Item = &'a dyn Analyzer;

    fn next(&mut self) -> Option<Self::Item> {
        self.symbols.pop_front()
    }
}

pub struct MutSymbolIterator<'a> {
    symbols: VecDeque<&'a mut dyn Analyzer>,
}

impl<'a> Iterator for MutSymbolIterator<'a> {
    type Item = &'a mut dyn Analyzer;

    fn next(&mut self) -> Option<Self::Item> {
        self.symbols.pop_front()
    }
}

/// # AccessAttrib
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct AccessAttrib(ByteData);

/// # AccessField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct AccessField(
    AccessFieldOp,
    AccessType,
    AccessAttrib,
);

/// # AccessFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x01]
pub struct AccessFieldOp;

/// # AccessType
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer)]
#[bitfield(u8)]
pub struct AccessType {
    #[bits(4)]
    access_type: u8,
    #[bits(2, access = RO)]
    reserved0: u8,
    #[bits(2)]
    access_attrib: u8,
}

/// # AcquireOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct AcquireOp(
    ExtOpPrefix,
    AcquireOpSuffix,
);

/// # AcquireOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x23]
pub struct AcquireOpSuffix;

/// # AddOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x72]
pub struct AddOp;

/// # AliasOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x06]
pub struct AliasOp;

/// # AmlString
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[string]
pub struct AmlString(
    #[not_string]
    StringPrefix,
    AsciiCharList,
    NullChar,
);

impl Evaluator for AmlString {
    fn evaluate(&self, _stack_frame: &mut interpreter::StackFrame, _root: &reference::Node, _current: &name::Path) -> Option<interpreter::Value> {
        Some(interpreter::Value::String(self.into()))
    }
}

/// # AndOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x7b]
pub struct AndOp;

/// # Arg Objects Encoding
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.1 Arg Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value_min = 0x68]
#[encoding_value_max = 0x6e]
pub struct ArgObj(u8);

impl Evaluator for ArgObj {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, _root: &reference::Node, _current: &name::Path) -> Option<interpreter::Value> {
        let Self(index) = self;
        stack_frame.read_argument(*index as usize)
    }
}

impl Holder for ArgObj {
    fn hold(&self, value: interpreter::Value, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> interpreter::Value {
        let Self(index) = self;
        stack_frame.write_argument(*index as usize, value)
    }
}

/// # ArgObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ArgObject(TermArg);

impl Evaluator for ArgObject {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # ArgumentCount
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ArgumentCount(ByteData);

impl From<&ArgumentCount> for usize {
    fn from(argument_count: &ArgumentCount) -> Self {
        let ArgumentCount(byte_data) = argument_count;
        byte_data.into()
    }
}

/// # AsciiChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value_min = 0x01]
#[encoding_value_max = 0x7f]
pub struct AsciiChar(char);

/// # AsciiCharList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 0]
#[string]
pub struct AsciiCharList(Vec<AsciiChar>);

impl Evaluator for AsciiCharList {
    fn evaluate(&self, _stack_frame: &mut interpreter::StackFrame, _root: &reference::Node, _current: &name::Path) -> Option<interpreter::Value> {
        Some(interpreter::Value::String(self.into()))
    }
}

/// # AsciiUppercase
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value_min = 0x41]
#[encoding_value_max = 0x5a]
pub struct AsciiUppercase(char);

/// # AttribBytes
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x0b]
pub struct AttribBytes;

/// # AttribRawBytes
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x0e]
pub struct AttribRawBytes;

/// # AttribRawProcess
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x0f]
pub struct AttribRawProcess;

/// # BankFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct BankFieldOp(
    ExtOpPrefix,
    BankFieldOpSuffix,
);

/// # BankFieldOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x87]
pub struct BankFieldOpSuffix;

/// # BankValue
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct BankValue(TermArg);

impl Evaluator for BankValue {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # BcdValue
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct BcdValue(TermArg);

impl Evaluator for BcdValue {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # BitIndex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct BitIndex(TermArg);

impl Evaluator for BitIndex {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # BreakOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0xa5]
pub struct BreakOp;

/// # BreakPointOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0xcc]
pub struct BreakPointOp;

/// # BufData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct BufData(TermArg);

impl Evaluator for BufData {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # BuffPkgStrObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct BuffPkgStrObj(TermArg);

impl Evaluator for BuffPkgStrObj {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # BufferOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x11]
pub struct BufferOp;

/// # BufferSize
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct BufferSize(TermArg);

impl Evaluator for BufferSize {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # ByteConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ByteConst(
    BytePrefix,
    ByteData,
);

impl Evaluator for ByteConst {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _byte_prefix,
            byte_data,
        ) = self;
        byte_data.evaluate(stack_frame, root, current)
    }
}

/// # ByteData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value_min = 0x00]
#[encoding_value_max = 0xff]
pub struct ByteData(u8);

impl From<&ByteData> for u8 {
    fn from(byte_data: &ByteData) -> Self {
        let ByteData(byte_data) = byte_data;
        *byte_data
    }
}

impl From<&ByteData> for usize {
    fn from(byte_data: &ByteData) -> Self {
        let byte_data: u8 = byte_data.into();
        byte_data as Self
    }
}

impl Evaluator for ByteData {
    fn evaluate(&self, _stack_frame: &mut interpreter::StackFrame, _root: &reference::Node, _current: &name::Path) -> Option<interpreter::Value> {
        let Self(byte) = self;
        Some(interpreter::Value::Byte(*byte))
    }
}

/// # ByteIndex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ByteIndex(TermArg);

/// # ByteList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ByteList(Vec<ByteData>);

impl From<&ByteList> for Vec<u8> {
    fn from(byte_list: &ByteList) -> Self {
        let ByteList(byte_list) = byte_list;
        byte_list
            .iter()
            .map(|byte_data| byte_data.into())
            .collect()
    }
}

impl Evaluator for ByteList {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(byte_list) = self;
        Some(interpreter::Value::Buffer(byte_list
            .iter()
            .filter_map(|byte_data| byte_data
                .evaluate(stack_frame, root, current)
                .and_then(|byte_data| byte_data.get_byte()))
            .collect()))
    }
}

/// # BytePrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x0a]
pub struct BytePrefix;

/// # ComputationalData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum ComputationalData {
    AmlString(AmlString),
    ByteConst(ByteConst),
    ConstObj(ConstObj),
    DWordConst(DWordConst),
    DefBuffer(DefBuffer),
    QWordConst(QWordConst),
    RevisionOp(RevisionOp),
    WordConst(WordConst),
}

impl Evaluator for ComputationalData {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::AmlString(aml_string) => aml_string.evaluate(stack_frame, root, current),
            Self::ByteConst(byte_const) => byte_const.evaluate(stack_frame, root, current),
            Self::ConstObj(const_obj) => const_obj.evaluate(stack_frame, root, current),
            Self::DWordConst(dword_const) => dword_const.evaluate(stack_frame, root, current),
            Self::DefBuffer(def_buffer) => def_buffer.evaluate(stack_frame, root, current),
            Self::QWordConst(qword_const) => qword_const.evaluate(stack_frame, root, current),
            Self::RevisionOp(revision_op) => revision_op.evaluate(stack_frame, root, current),
            Self::WordConst(word_const) => word_const.evaluate(stack_frame, root, current),
        }
    }
}

/// # ConcatOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x73]
pub struct ConcatOp;

/// # ConcatResOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x84]
pub struct ConcatResOp;

/// # CondRefOfOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct CondRefOfOp(
    ExtOpPrefix,
    CondRefOfOpSuffix,
);

/// # CondRefOfOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x12]
pub struct CondRefOfOpSuffix;

/// # ConnectField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ConnectField(
    ConnectFieldOp,
    ConnectFieldEnum,
);

/// # ConnectFieldEnum
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum ConnectFieldEnum {
    NameString(NameString),
    BufData(BufData),
}

/// # ConnectFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x02]
pub struct ConnectFieldOp;

/// # ContinueOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x9f]
pub struct ContinueOp;

/// # CopyObjectOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x9d]
pub struct CopyObjectOp;

/// # ConstObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum ConstObj {
    One(OneOp),
    Ones(OnesOp),
    Zero(ZeroOp),
}

impl Evaluator for ConstObj {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::One(one_op) => one_op.evaluate(stack_frame, root, current),
            Self::Ones(ones_op) => ones_op.evaluate(stack_frame, root, current),
            Self::Zero(zero_op) => zero_op.evaluate(stack_frame, root, current),
        }
    }
}

/// # CreateBitFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x8d]
pub struct CreateBitFieldOp;

/// # CreateByteFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x8c]
pub struct CreateByteFieldOp;

/// # CreateDWordFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x8a]
pub struct CreateDWordFieldOp;

/// # CreateFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct CreateFieldOp(
    ExtOpPrefix,
    CreateFieldOpSuffix,
);

/// # CreateFieldOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x13]
pub struct CreateFieldOpSuffix;

/// # CreateQWordFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x8f]
pub struct CreateQWordFieldOp;

/// # CreateWordFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x8b]
pub struct CreateWordFieldOp;

/// # DWordConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DWordConst(
    DWordPrefix,
    DWordData,
);

impl Evaluator for DWordConst {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _dword_prefix,
            dword_data,
        ) = self;
        dword_data.evaluate(stack_frame, root, current)
    }
}

/// # DWordData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DWordData(
    [WordData; 2],
);

impl Evaluator for DWordData {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self([low, high]) = self;
        let low: Option<interpreter::Value> = low.evaluate(stack_frame, root, current);
        let high: Option<interpreter::Value> = high.evaluate(stack_frame, root, current);
        low
            .zip(high)
            .map(|(low, high)| low.concatenate(high))
    }
}

/// # DWordPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x0c]
pub struct DWordPrefix;

/// # Data
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct Data(TermArg);

/// # DataObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum DataObject {
    ComputationalData(ComputationalData),
    DefPackage(DefPackage),
    DefVarPackage(DefVarPackage),
}

impl Evaluator for DataObject {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::ComputationalData(computational_data) => computational_data.evaluate(stack_frame, root, current),
            Self::DefPackage(def_package) => def_package.evaluate(stack_frame, root, current),
            Self::DefVarPackage(def_var_package) => def_var_package.evaluate(stack_frame, root, current),
        }
    }
}

/// # DataRefObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum DataRefObject {
    DataObject(DataObject),
    ObjReference(ObjReference),
}

impl Evaluator for DataRefObject {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::DataObject(data_object) => data_object.evaluate(stack_frame, root, current),
            Self::ObjReference(obj_reference) => unimplemented!("obj_reference = {:#x?}", obj_reference),
        }
    }
}

/// # DataRegionOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct DataRegionOp(
    ExtOpPrefix,
    DataRegionOpSuffix,
);

/// # DataRegionOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x88]
pub struct DataRegionOpSuffix;

/// # DebugObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.3 Debug Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DebugObj(DebugOp);

impl Holder for DebugObj {
    fn hold(&self, value: interpreter::Value, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> interpreter::Value {
        com2_println!("AML DebugObj = {:#x?}", value);
        value
    }
}

/// # DebugOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.3 Debug Opects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct DebugOp(
    ExtOpPrefix,
    DebugOpSuffix,
);

/// # DebugOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.3 Debug Opects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x31]
pub struct DebugOpSuffix;

/// # DecrementOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x76]
pub struct DecrementOp;

/// # DefAcquire
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefAcquire(
    AcquireOp,
    MutexObject,
    Timeout,
);

/// # DefAdd
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefAdd(
    AddOp,
    [Operand; 2],
    Target,
);

impl Evaluator for DefAdd {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _add_op,
            [left, right],
            target,
        ) = self;
        let left: Option<interpreter::Value> = left.evaluate(stack_frame, root, current);
        let right: Option<interpreter::Value> = right.evaluate(stack_frame, root, current);
        let value: Option<interpreter::Value> = left
            .zip(right)
            .map(|(left, right)| left + right);
        value.map(|value| target.hold(value, stack_frame, root, current))
    }
}

/// # DefAlias
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(first_reader, path_getter, reader_inside_method)]
pub struct DefAlias(
    AliasOp,
    [NameString; 2],
);

impl DefAlias {
    pub fn solve(&self, current: &name::Path) -> name::AbsolutePath {
        let Self(
            _alias_op,
            [original_name, _new_name],
        ) = self;
        let original_path: name::Path = original_name.into();
        name::AbsolutePath::new(current, &original_path)
    }
}

impl FirstReader for DefAlias {
    fn first_read<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (alias_op, symbol_aml): (AliasOp, &[u8]) = AliasOp::first_read(symbol_aml, root, &current);
        let (original_name, symbol_aml): (NameString, &[u8]) = NameString::first_read(symbol_aml, root, &current);
        let (new_name, _symbol_aml): (NameString, &[u8]) = NameString::first_read(symbol_aml, root, &current);
        let original_path: name::Path = current.clone() + (&original_name).into();
        let new_path: name::Path = current.clone() + (&new_name).into();
        root.add_node(&new_path, name::Object::alias(&current, &original_path));
        let name_strings: [NameString; 2] = [original_name, new_name];
        let def_alias = Self(
            alias_op,
            name_strings,
        );
        let aml: &[u8] = &aml[def_alias.length()..];
        (def_alias, aml)
    }
}

impl PathGetter for DefAlias {
    fn get_path(&self) -> Option<name::Path> {
        let Self(
            _alias_op,
            [original_name, _new_name],
        ) = self;
        Some(original_name.into())
    }
}

impl ReaderInsideMethod for DefAlias {
    fn read_inside_method<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (alias_op, symbol_aml): (AliasOp, &[u8]) = AliasOp::read_inside_method(symbol_aml, root, &current);
        let (original_name, symbol_aml): (NameString, &[u8]) = NameString::read_inside_method(symbol_aml, root, &current);
        let (new_name, _symbol_aml): (NameString, &[u8]) = NameString::read_inside_method(symbol_aml, root, &current);
        let original_path: name::Path = current.clone() + (&original_name).into();
        let new_path: name::Path = current.clone() + (&new_name).into();
        root.add_node(&new_path, name::Object::alias(&current, &original_path));
        let name_strings: [NameString; 2] = [original_name, new_name];
        let def_alias = Self(
            alias_op,
            name_strings,
        );
        let aml: &[u8] = &aml[def_alias.length()..];
        (def_alias, aml)
    }
}

/// # DefAnd
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefAnd(
    AndOp,
    [Operand; 2],
    Target,
);

/// # DefBankField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefBankField(
    BankFieldOp,
    PkgLength,
    [NameString; 2],
    BankValue,
    FieldFlags,
    #[no_leftover]
    FieldList,
);

/// # DefBreak
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefBreak(BreakOp);

/// # DefBreakPoint
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefBreakPoint(BreakPointOp);

/// # DefBuffer
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefBuffer(
    BufferOp,
    PkgLength,
    BufferSize,
    #[no_leftover]
    ByteList,
);

impl Evaluator for DefBuffer {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _buffer_op,
            _pkg_length,
            buffer_size,
            byte_list,
        ) = self;
        let buffer_size: usize = buffer_size
            .evaluate(stack_frame, root, current)
            .as_ref()
            .unwrap()
            .into();
        let byte_list: Vec<u8> = byte_list
            .evaluate(stack_frame, root, current)
            .as_ref()
            .unwrap()
            .into();
        let buffer: Vec<u8> = byte_list
            .into_iter()
            .chain(iter::repeat(0)
                .take(buffer_size))
            .take(buffer_size)
            .collect();
        let buffer: interpreter::Value = buffer.into();
        Some(buffer)
    }
}

/// # DefCondRefOf
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefCondRefOf(
    CondRefOfOp,
    SuperName,
    Target,
);

/// # DefConcat
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefConcat(
    ConcatOp,
    [Data; 2],
    Target,
);

/// # DefConcatRes
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefConcatRes(
    ConcatResOp,
    [BufData; 2],
    Target,
);

/// # DefContinue
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefContinue(ContinueOp);

/// # DefCopyObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefCopyObject(
    CopyObjectOp,
    TermArg,
    SimpleName,
);

/// # DefCreateBitField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefCreateBitField(
    CreateBitFieldOp,
    SourceBuff,
    BitIndex,
    NameString,
);

/// # DefCreateByteField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefCreateByteField(
    CreateByteFieldOp,
    SourceBuff,
    ByteIndex,
    NameString,
);

/// # DefCreateDWordField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefCreateDWordField(
    CreateDWordFieldOp,
    SourceBuff,
    ByteIndex,
    NameString,
);

/// # DefCreateField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefCreateField(
    CreateFieldOp,
    SourceBuff,
    BitIndex,
    NumBits,
    NameString,
);

/// # DefCreateQWordField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefCreateQWordField(
    CreateQWordFieldOp,
    SourceBuff,
    ByteIndex,
    NameString,
);

/// # DefCreateWordField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefCreateWordField(
    CreateWordFieldOp,
    SourceBuff,
    ByteIndex,
    NameString,
);

/// # DefDataRegion
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefDataRegion(
    DataRegionOp,
    NameString,
    [TermArg; 3],
);

/// # DefDecrement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefDecrement(
    DecrementOp,
    SuperName,
);

impl Evaluator for DefDecrement {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _decrement_op,
            super_name,
        ) = self;
        super_name
            .evaluate(stack_frame, root, current)
            .map(|super_name| super_name - interpreter::Value::One)
    }
}

/// # DefDerefOf
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefDerefOf(
    DerefOfOp,
    ObjReference,
);

impl Evaluator for DefDerefOf {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _deref_of_op,
            obj_reference,
        ) = self;
        obj_reference.evaluate(stack_frame, root, current)
    }
}

/// # DefDevice
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefDevice(
    DeviceOp,
    PkgLength,
    NameString,
    #[no_leftover]
    TermList,
);

/// # DefDivide
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefDivide(
    DivideOp,
    Dividend,
    Divisor,
    Remainder,
    Quotient,
);

/// # DefElse
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefElse(
    ElseOp,
    PkgLength,
    #[no_leftover]
    TermList,
);

impl Evaluator for DefElse {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _else_op,
            _pkg_length,
            term_list,
        ) = self;
        term_list.evaluate(stack_frame, root, current)
    }
}

/// # DefEvent
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefEvent(
    EventOp,
    NameString,
);

/// # DefExternal
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(first_reader, reader_inside_method)]
pub struct DefExternal(
    ExternalOp,
    NameString,
    ObjectType,
    ArgumentCount,
);

impl FirstReader for DefExternal {
    fn first_read<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (external_op, symbol_aml): (ExternalOp, &[u8]) = ExternalOp::first_read(symbol_aml, root, &current);
        let (name_string, symbol_aml): (NameString, &[u8]) = NameString::first_read(symbol_aml, root, &current);
        let (object_type, symbol_aml): (ObjectType, &[u8]) = ObjectType::first_read(symbol_aml, root, &current);
        let (argument_count, _symbol_aml): (ArgumentCount, &[u8]) = ArgumentCount::first_read(symbol_aml, root, &current);
        let current: name::Path = current.clone() + (&name_string).into();
        let number_of_arguments: usize = (&argument_count).into();
        root.add_node(&current, name::Object::external(number_of_arguments));
        let symbol = Self(
            external_op,
            name_string,
            object_type,
            argument_count,
        );
        let aml: &[u8] = &aml[symbol.length()..];
        (symbol, aml)
    }
}

impl ReaderInsideMethod for DefExternal {
    fn read_inside_method<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (external_op, symbol_aml): (ExternalOp, &[u8]) = ExternalOp::read_inside_method(symbol_aml, root, &current);
        let (name_string, symbol_aml): (NameString, &[u8]) = NameString::read_inside_method(symbol_aml, root, &current);
        let (object_type, symbol_aml): (ObjectType, &[u8]) = ObjectType::read_inside_method(symbol_aml, root, &current);
        let (argument_count, _symbol_aml): (ArgumentCount, &[u8]) = ArgumentCount::read_inside_method(symbol_aml, root, &current);
        let current: name::Path = current.clone() + (&name_string).into();
        let number_of_arguments: usize = (&argument_count).into();
        root.add_node(&current, name::Object::external(number_of_arguments));
        let symbol = Self(
            external_op,
            name_string,
            object_type,
            argument_count,
        );
        let aml: &[u8] = &aml[symbol.length()..];
        (symbol, aml)
    }
}

/// # DefFatal
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefFatal(
    FatalOp,
    FatalType,
    FatalCode,
    FatalArg,
);

/// # DefField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefField(
    FieldOp,
    PkgLength,
    NameString,
    FieldFlags,
    #[no_leftover]
    FieldList,
);

/// # DefFindSetLeftBit
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefFindSetLeftBit(
    FindSetLeftBitOp,
    Operand,
    Target,
);

/// # DefFindSetRightBit
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefFindSetRightBit(
    FindSetRightBitOp,
    Operand,
    Target,
);

/// # DefFromBcd
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefFromBcd(
    FromBcdOp,
    BcdValue,
    Target,
);

/// # DefIf
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefIf(
    IfOp,
    PkgLength,
    Predicate,
    #[no_leftover]
    TermList,
);

/// # DefIfElse
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefIfElse(
    DefIf,
    Option<DefElse>,
);

impl Evaluator for DefIfElse {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            DefIf(
                _if_op,
                _pkg_length,
                predicate,
                term_list,
            ),
            def_else,
        ) = self;
        if predicate
            .evaluate(stack_frame, root, current)
            .map_or(false, |predicate| (&predicate).into()) {
            term_list.evaluate(stack_frame, root, current)
        } else {
            def_else
                .as_ref()
                .and_then(|def_else| def_else.evaluate(stack_frame, root, current))
        }
    }
}

/// # DefIncrement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefIncrement(
    IncrementOp,
    SuperName,
);

impl Evaluator for DefIncrement {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _decrement_op,
            super_name,
        ) = self;
        super_name
            .evaluate(stack_frame, root, current)
            .map(|super_name| super_name + interpreter::Value::One)
    }
}

/// # DefIndex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefIndex(
    IndexOp,
    BuffPkgStrObj,
    IndexValue,
    Target,
);

impl Evaluator for DefIndex {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _index_op,
            buff_pkg_str_obj,
            index_value,
            target,
        ) = self;
        let buff_pkg_str_obj: Option<interpreter::Value> = buff_pkg_str_obj.evaluate(stack_frame, root, current);
        let index_value: Option<interpreter::Value> = index_value.evaluate(stack_frame, root, current);
        let value: Option<interpreter::Value> = buff_pkg_str_obj
            .zip(index_value)
            .and_then(|(buff_pkg_str_obj, index_value)| buff_pkg_str_obj.index(&index_value));
        value.map(|value| target.hold(value, stack_frame, root, current))
    }
}

/// # DefIndexField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefIndexField(
    IndexFieldOp,
    PkgLength,
    [NameString; 2],
    FieldFlags,
    #[no_leftover]
    FieldList,
);

/// # DefLAnd
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefLAnd(
    LAndOp,
    [Operand; 2],
);

/// # DefLEqual
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefLEqual(
    LEqualOp,
    [Operand; 2],
);

/// # DefLGreater
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefLGreater(
    LGreaterOp,
    [Operand; 2],
);

/// # DefLGreaterEqual
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefLGreaterEqual(
    LGreaterEqualOp,
    [Operand; 2],
);

impl Evaluator for DefLGreaterEqual {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _l_greater_equal_op,
            [left, right],
        ) = self;
        let left: Option<interpreter::Value> = left.evaluate(stack_frame, root, current);
        let right: Option<interpreter::Value> = right.evaluate(stack_frame, root, current);
        left
            .zip(right)
            .map(|(left, right)| (left >= right).into())
    }
}

/// # DefLLess
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefLLess(
    LLessOp,
    [Operand; 2],
);

/// # DefLLessEqual
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefLLessEqual(
    LLessEqualOp,
    [Operand; 2],
);

/// # DefLNot
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefLNot(
    LNotOp,
    Operand,
);

/// # DefLNotEqual
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefLNotEqual(
    LNotEqualOp,
    [Operand; 2],
);

/// # DefLOr
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefLOr(
    LOrOp,
    [Operand; 2],
);

/// # DefLoad
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefLoad(
    LoadOp,
    NameString,
    Target,
);

/// # DefLoadTable
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefLoadTable(
    LoadTableOp,
    [TermArg; 6],
);

/// # DefMatch
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefMatch(
    MatchOp,
    SearchPkg,
    MatchOpcode,
    Operand,
    MatchOpcode,
    Operand,
    StartIndex,
);

/// # DefMethod
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(first_reader, reader_inside_method)]
pub struct DefMethod(
    MethodOp,
    PkgLength,
    NameString,
    MethodFlags,
    #[no_leftover]
    MethodTermList,
);

impl FirstReader for DefMethod {
    fn first_read<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (method_op, symbol_aml): (MethodOp, &[u8]) = MethodOp::first_read(symbol_aml, root, &current);
        let (pkg_length, symbol_aml): (PkgLength, &[u8]) = PkgLength::first_read(symbol_aml, root, &current);
        let (name_string, symbol_aml): (NameString, &[u8]) = NameString::first_read(symbol_aml, root, &current);
        let (method_flags, symbol_aml): (MethodFlags, &[u8]) = MethodFlags::first_read(symbol_aml, root, &current);
        let current: name::Path = current.clone() + (&name_string).into();
        let number_of_arguments: usize = method_flags.arg_count() as usize;
        root.add_node(&current, name::Object::method(number_of_arguments));
        let (method_term_list, symbol_aml): (MethodTermList, &[u8]) = MethodTermList::first_read(symbol_aml, root, &current);
        assert!(symbol_aml.is_empty());
        let symbol = Self(
            method_op,
            pkg_length,
            name_string,
            method_flags,
            method_term_list,
        );
        let aml: &[u8] = &aml[symbol.length()..];
        (symbol, aml)
    }
}

impl ReaderInsideMethod for DefMethod {
    fn read_inside_method<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (method_op, symbol_aml): (MethodOp, &[u8]) = MethodOp::read_inside_method(symbol_aml, root, &current);
        let (pkg_length, symbol_aml): (PkgLength, &[u8]) = PkgLength::read_inside_method(symbol_aml, root, &current);
        let (name_string, symbol_aml): (NameString, &[u8]) = NameString::read_inside_method(symbol_aml, root, &current);
        let (method_flags, symbol_aml): (MethodFlags, &[u8]) = MethodFlags::read_inside_method(symbol_aml, root, &current);
        let current: name::Path = current.clone() + (&name_string).into();
        let number_of_arguments: usize = method_flags.arg_count() as usize;
        root.add_node(&current, name::Object::method(number_of_arguments));
        let (method_term_list, symbol_aml): (MethodTermList, &[u8]) = MethodTermList::read_inside_method(symbol_aml, root, &current);
        assert!(symbol_aml.is_empty());
        let symbol = Self(
            method_op,
            pkg_length,
            name_string,
            method_flags,
            method_term_list,
        );
        let aml: &[u8] = &aml[symbol.length()..];
        (symbol, aml)
    }
}

impl Evaluator for DefMethod {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _method_op,
            _pkg_length,
            name_string,
            _method_flags,
            method_term_list,
        ) = self;
        let name_string: name::Path = name_string.into();
        let current: name::Path = current.clone() + name_string;
        method_term_list.evaluate(stack_frame, root, &current)
    }
}

/// # DefMid
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefMid(
    MidOp,
    MidObj,
    [TermArg; 2],
    Target,
);

/// # DefMod
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefMod(
    ModOp,
    Dividend,
    Divisor,
    Target,
);

/// # DefMultiply
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefMultiply(
    MultiplyOp,
    [Operand; 2],
    Target,
);

/// # DefMutex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefMutex(
    MutexOp,
    NameString,
    SyncFlags,
);

/// # DefNAnd
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefNAnd(
    NAndOp,
    [Operand; 2],
    Target,
);

/// # DefNOr
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefNOr(
    NOrOp,
    [Operand; 2],
    Target,
);

/// # DefName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefName(
    NameOp,
    NameString,
    DataRefObject,
);

impl Evaluator for DefName {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _name_op,
            name_string,
            data_ref_object,
        ) = self;
        let path: name::Path = name_string.into();
        let current: name::Path = current.clone() + path;
        let name: name::Path = name_string.into();
        let data_ref_object: Option<interpreter::Value> = data_ref_object.evaluate(stack_frame, root, &current);
        if let Some(data_ref_object) = data_ref_object.as_ref() {
            stack_frame.add_named_local(&name, data_ref_object.clone());
        }
        data_ref_object
    }
}

/// # DefNoop
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefNoop(NoopOp);

/// # DefNot
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefNot(
    NotOp,
    Operand,
    Target,
);

/// # DefNotify
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefNotify(
    NotifyOp,
    NotifyObject,
    NotifyValue,
);

/// # DefOpRegion
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefOpRegion(
    OpRegionOp,
    NameString,
    RegionSpace,
    RegionOffset,
    RegionLen,
);

/// # DefObjectType
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefObjectType(
    ObjectTypeOp,
    ObjectTypeEnum,
);

/// # DefOr
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefOr(
    OrOp,
    [Operand; 2],
    Target,
);

/// # DefPackage
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefPackage(
    PackageOp,
    PkgLength,
    NumElements,
    #[no_leftover]
    PackageElementList,
);

impl Evaluator for DefPackage {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _package_op,
            _pkg_length,
            _num_elements,
            package_element_list,
        ) = self;
        package_element_list.evaluate(stack_frame, root, current)
    }
}

/// # DefPowerRes
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefPowerRes(
    PowerResOp,
    PkgLength,
    NameString,
    SystemLevel,
    ResourceOrder,
    #[no_leftover]
    TermList,
);

/// # DefProcessor
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefProcessor(
    ProcessorOp,
    PkgLength,
    NameString,
    ProcId,
    PblkAddr,
    PblkLen,
    #[no_leftover]
    ObjectList,
);

/// # DefRefOf
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefRefOf(
    RefOfOp,
    Box<SuperName>,
);

/// # DefRelease
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefRelease(
    ReleaseOp,
    MutexObject,
);

/// # DefReset
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefReset(
    ResetOp,
    EventObject,
);

/// # DefReturn
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefReturn(
    ReturnOp,
    ArgObject,
);

impl Evaluator for DefReturn {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _return_op,
            arg_object,
        ) = self;
        arg_object
            .evaluate(stack_frame, root, current)
            .map(|arg_object| stack_frame.write_return(arg_object))
    }
}

/// # DefScope
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefScope(
    ScopeOp,
    PkgLength,
    NameString,
    #[no_leftover]
    TermList,
);

/// # DefShiftLeft
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefShiftLeft(
    ShiftLeftOp,
    Operand,
    ShiftCount,
    Target,
);

/// # DefShiftRight
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefShiftRight(
    ShiftRightOp,
    Operand,
    ShiftCount,
    Target,
);

/// # DefSignal
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefSignal(
    SignalOp,
    EventObject,
);

/// # DefSizeOf
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefSizeOf(
    SizeOfOp,
    SuperName,
);

/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 19.6.124 SizeOf (Get Data Object Size)
impl Evaluator for DefSizeOf {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _size_of_op,
            super_name,
        ) = self;
        super_name
            .evaluate(stack_frame, root, current)
            .map(|super_name| super_name.size())
    }
}

/// # DefSleep
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefSleep(
    SleepOp,
    MsecTime,
);

/// # DefStall
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefStall(
    StallOp,
    UsecTime,
);

/// # DefStore
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefStore(
    StoreOp,
    TermArg,
    SuperName,
);

impl Evaluator for DefStore {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _store_op,
            term_arg,
            super_name,
        ) = self;
        term_arg
            .evaluate(stack_frame, root, current)
            .map(|term_arg| super_name.hold(term_arg, stack_frame, root, current))
    }
}

/// # DefSubtract
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefSubtract(
    SubtractOp,
    [Operand; 2],
    Target,
);

/// # DefThermalZone
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefThermalZone(
    ThermalZoneOp,
    PkgLength,
    NameString,
    #[no_leftover]
    TermList,
);

/// # DefTimer
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefTimer(
    TimerOp,
);

/// # DefToBcd
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefToBcd(
    ToBcdOp,
    Operand,
    Target,
);

/// # DefToBuffer
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefToBuffer(
    ToBufferOp,
    Operand,
    Target,
);

/// # DefToDecimalString
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefToDecimalString(
    ToDecimalStringOp,
    Operand,
    Target,
);

/// # DefToHexString
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefToHexString(
    ToHexStringOp,
    Operand,
    Target,
);

/// # DefToInteger
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefToInteger(
    ToIntegerOp,
    Operand,
    Target,
);

/// # DefToString
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefToString(
    ToStringOp,
    TermArg,
    LengthArg,
    Target,
);

/// # DefVarPackage
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefVarPackage(
    VarPackageOp,
    PkgLength,
    VarNumElements,
    #[no_leftover]
    PackageElementList,
);

impl Evaluator for DefVarPackage {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _var_package_op,
            _pkg_length,
            _var_num_elements,
            package_element_list,
        ) = self;
        package_element_list.evaluate(stack_frame, root, current)
    }
}

/// # DefWait
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefWait(
    WaitOp,
    EventObject,
    Operand,
);

/// # DefWhile
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefWhile(
    WhileOp,
    PkgLength,
    Predicate,
    #[no_leftover]
    TermList,
);

impl Evaluator for DefWhile {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _while_op,
            _pkg_length,
            predicate,
            term_list,
        ) = self;
        while predicate
            .evaluate(stack_frame, root, current)
            .map_or(false, |predicate| (&predicate).into()) {
            term_list.evaluate(stack_frame, root, current);
        }
        None
    }
}

/// # DefXOr
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefXOr(
    XOrOp,
    [Operand; 2],
    Target,
);

/// # DerefOfOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x83]
pub struct DerefOfOp;

/// # DeviceOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct DeviceOp(
    ExtOpPrefix,
    DeviceOpSuffix,
);

/// # DeviceOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x82]
pub struct DeviceOpSuffix;

/// # DigitChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value_min = 0x30]
#[encoding_value_max = 0x39]
pub struct DigitChar(char);

/// # DivideOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x78]
pub struct DivideOp;

/// # Dividend
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct Dividend(TermArg);

/// # Divisor
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct Divisor(TermArg);

/// # DualNamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[string]
pub struct DualNamePath(
    #[not_string]
    DualNamePrefix,
    #[delimiter = "."]
    [NameSeg; 2],
);

impl From<&DualNamePath> for VecDeque<name::Segment> {
    fn from(dual_name_path: &DualNamePath) -> Self {
        let DualNamePath(
            _dual_name_prefix,
            name_segs,
        ) = dual_name_path;
        name_segs
            .as_slice()
            .iter()
            .map(|name_seg| name_seg.into())
            .collect()
    }
}

/// # DualNamePrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x2e]
pub struct DualNamePrefix;

/// # ElseOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0xa1]
pub struct ElseOp;

/// # EventObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct EventObject(SuperName);

/// # EventOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct EventOp(
    ExtOpPrefix,
    EventOpSuffix,
);

/// # EventOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x02]
pub struct EventOpSuffix;

/// # ExpressionOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum ExpressionOpcode {
    Acquire(DefAcquire),
    Add(DefAdd),
    And(DefAnd),
    Buffer(DefBuffer),
    Concat(DefConcat),
    ConcatRes(DefConcatRes),
    CondRefOf(DefCondRefOf),
    CopyObject(DefCopyObject),
    Decrement(DefDecrement),
    DerefOf(DefDerefOf),
    Divide(DefDivide),
    FindSetLeftBit(DefFindSetLeftBit),
    FindSetRightBit(DefFindSetRightBit),
    FromBcd(DefFromBcd),
    Increment(DefIncrement),
    Index(DefIndex),
    LAnd(DefLAnd),
    LEqual(DefLEqual),
    LGreater(DefLGreater),
    LGreaterEqual(DefLGreaterEqual),
    LLess(DefLLess),
    LLessEqual(DefLLessEqual),
    LNot(DefLNot),
    LNotEqual(DefLNotEqual),
    LOr(DefLOr),
    Load(DefLoad),
    LoadTable(DefLoadTable),
    Match(DefMatch),
    MethodInvocation(MethodInvocation),
    Mid(DefMid),
    Mod(DefMod),
    Multiply(DefMultiply),
    NAnd(DefNAnd),
    NOr(DefNOr),
    Not(DefNot),
    ObjectType(DefObjectType),
    Or(DefOr),
    Package(DefPackage),
    RefOf(DefRefOf),
    ShiftLeft(DefShiftLeft),
    ShiftRight(DefShiftRight),
    SizeOf(DefSizeOf),
    Store(DefStore),
    Subtract(DefSubtract),
    Timer(DefTimer),
    ToBcd(DefToBcd),
    ToBuffer(DefToBuffer),
    ToDecimalString(DefToDecimalString),
    ToHexString(DefToHexString),
    ToInteger(DefToInteger),
    ToString(DefToString),
    VarPackage(DefVarPackage),
    Wait(DefWait),
    XOr(DefXOr),
}

impl Evaluator for ExpressionOpcode {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::Add(def_add) => def_add.evaluate(stack_frame, root, current),
            Self::Decrement(def_decrement) => def_decrement.evaluate(stack_frame, root, current),
            Self::Increment(def_increment) => def_increment.evaluate(stack_frame, root, current),
            Self::Index(def_index) => def_index.evaluate(stack_frame, root, current),
            Self::LGreaterEqual(def_l_greater_equal) => def_l_greater_equal.evaluate(stack_frame, root, current),
            Self::MethodInvocation(method_invocation) => method_invocation.evaluate(stack_frame, root, current),
            Self::SizeOf(def_size_of) => def_size_of.evaluate(stack_frame, root, current),
            Self::Store(def_store) => def_store.evaluate(stack_frame, root, current),
            Self::DerefOf(def_deref_of) => def_deref_of.evaluate(stack_frame, root, current),
            _ => unimplemented!("self = {:#x?}", self),
        }
    }
}

/// # ExtOpPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x5b]
pub struct ExtOpPrefix;

/// # ExtendedAccessField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum ExtendedAccessField {
    Bytes(AttribBytes),
    RawBytes(AttribRawBytes),
    RawProcess(AttribRawProcess),
}

/// # ExternalOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x15]
pub struct ExternalOp;

/// # FieldElement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum FieldElement {
    AccessField(AccessField),
    ConnectField(ConnectField),
    ExtendedAccessField(ExtendedAccessField),
    Named(NamedField),
    Reserved(ReservedField),
}

/// # FataArg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct FatalArg(TermArg);

/// # FatalCode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct FatalCode(DWordData);

/// # FatalOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct FatalOp(
    ExtOpPrefix,
    FatalOpSuffix,
);

/// # FatalOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x32]
pub struct FatalOpSuffix;

/// # FatalType
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct FatalType(ByteData);

/// # FieldFlags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer)]
#[bitfield(u8)]
pub struct FieldFlags {
    #[bits(4)]
    access_type: u8,
    lock_rule: bool,
    #[bits(2)]
    update_rule: u8,
    #[bits(access = RO)]
    reserved0: bool,
}

/// # FieldList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct FieldList(Vec<FieldElement>);

/// # FieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct FieldOp(
    ExtOpPrefix,
    FieldOpSuffix,
);

/// # FieldOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x81]
pub struct FieldOpSuffix;

/// # FindSetLeftBitOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x81]
pub struct FindSetLeftBitOp;

/// # FindSetRightBitOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x82]
pub struct FindSetRightBitOp;

/// # FromBcdOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct FromBcdOp(
    ExtOpPrefix,
    FromBcdOpSuffix,
);

/// # FromBcdOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x28]
pub struct FromBcdOpSuffix;

/// # IfOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0xa0]
pub struct IfOp;

/// # IncrementOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x75]
pub struct IncrementOp;

/// # IndexFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct IndexFieldOp(
    ExtOpPrefix,
    IndexFieldOpSuffix,
);

/// # IndexFieldOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x86]
pub struct IndexFieldOpSuffix;

/// # IndexOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x88]
pub struct IndexOp;

/// # IndexValue
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct IndexValue(TermArg);

impl Evaluator for IndexValue {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # LAndOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x90]
pub struct LAndOp;

/// # LEqualOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x93]
pub struct LEqualOp;

/// # LGreaterEqualOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct LGreaterEqualOp(
    LNotOp,
    LLessOp,
);

/// # LGreaterOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x94]
pub struct LGreaterOp;

/// # LLessEqualOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct LLessEqualOp(
    LNotOp,
    LGreaterOp,
);

/// # LLessOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x95]
pub struct LLessOp;

/// # LNotEqualOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct LNotEqualOp(
    LNotOp,
    LEqualOp,
);

/// # LNotOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x92]
pub struct LNotOp;

/// # LOrOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x91]
pub struct LOrOp;

/// # LengthArg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct LengthArg(TermArg);

impl Evaluator for LengthArg {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # LoadOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct LoadOp(
    ExtOpPrefix,
    LoadOpSuffix,
);

/// # LoadOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x20]
pub struct LoadOpSuffix;

/// # LoadTableOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct LoadTableOp(
    ExtOpPrefix,
    LoadTableOpSuffix,
);

/// # LoadTableOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x1f]
pub struct LoadTableOpSuffix;

/// # LeadNameChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[string]
pub enum LeadNameChar {
    AsciiUppercase(AsciiUppercase),
    Underscore(Underscore),
}

/// # Local Objects Encoding
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.2 Local Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value_min = 0x60]
#[encoding_value_max = 0x67]
pub struct LocalObj(u8);

impl Evaluator for LocalObj {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, _root: &reference::Node, _current: &name::Path) -> Option<interpreter::Value> {
        let Self(index) = self;
        stack_frame.read_local(*index as usize)
    }
}

impl Holder for LocalObj {
    fn hold(&self, value: interpreter::Value, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> interpreter::Value {
        let Self(index) = self;
        stack_frame.write_local(*index as usize, value)
    }
}

/// # MatchOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x89]
pub struct MatchOp;

/// # MatchOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct MatchOpcode(ByteData);

/// # MethodFlags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer)]
#[bitfield(u8)]
pub struct MethodFlags {
    #[bits(3)]
    arg_count: u8,
    serialize: bool,
    #[bits(4)]
    sync_level: u8,
}

/// # MethodInvocation
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(first_reader, reader, reader_inside_method)]
pub struct MethodInvocation(
    NameString,
    Vec<TermArg>,
);

impl FirstReader for MethodInvocation {
    fn first_read<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (name_string, symbol_aml): (NameString, &[u8]) = NameString::first_read(symbol_aml, root, &current);
        let method: name::Path = (&name_string).into();
        let method = name::AbsolutePath::new(&current, &method);
        let number_of_arguments: usize = root
            .find_number_of_arguments_from_current(&method)
            .unwrap();
        let mut symbol_aml: &[u8] = symbol_aml;
        let mut term_args: Vec<TermArg> = Vec::new();
        (0..number_of_arguments)
            .for_each(|_| {
                let (term_arg, remaining_aml): (TermArg, &[u8]) = TermArg::first_read(symbol_aml, root, &current);
                symbol_aml = remaining_aml;
                term_args.push(term_arg);
            });
        let method_invocation = Self(
            name_string,
            term_args,
        );
        let aml: &[u8] = &aml[method_invocation.length()..];
        (method_invocation, aml)
    }
}

impl Reader for MethodInvocation {
    fn read(aml: &[u8]) -> (Self, &[u8]) {
        let (name_string, aml): (NameString, &[u8]) = NameString::read(aml);
        let term_args: Vec<TermArg> = Vec::new();
        let method_invocation = Self(
            name_string,
            term_args,
        );
        (method_invocation, aml)
    }
}

impl ReaderInsideMethod for MethodInvocation {
    fn read_inside_method<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (name_string, symbol_aml): (NameString, &[u8]) = NameString::read_inside_method(symbol_aml, root, &current);
        let method: name::Path = (&name_string).into();
        let method = name::AbsolutePath::new(&current, &method);
        let number_of_arguments: usize = root
            .find_number_of_arguments_from_current(&method)
            .or_else(|| method
                .last_segment()
                .and_then(|segment| {
                    let method2number_of_arguments: BTreeMap<&str, usize> = BTreeMap::from([
                        ("_OS", 0),
                        ("_OSI", 1),
                        ("_REV", 0),
                    ]);
                    let segment: String = (&segment).into();
                    method2number_of_arguments
                        .get(segment.as_str())
                        .cloned()
                }))
            .unwrap();
        let mut symbol_aml: &[u8] = symbol_aml;
        let mut term_args: Vec<TermArg> = Vec::new();
        (0..number_of_arguments)
            .for_each(|_| {
                let (term_arg, remaining_aml): (TermArg, &[u8]) = TermArg::read_inside_method(symbol_aml, root, &current);
                symbol_aml = remaining_aml;
                term_args.push(term_arg);
            });
        let method_invocation = Self(
            name_string,
            term_args,
        );
        let aml: &[u8] = &aml[method_invocation.length()..];
        (method_invocation, aml)
    }
}

impl Evaluator for MethodInvocation {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            name_string,
            term_args,
        ) = self;
        let method: name::Path = name_string.into();
        stack_frame
            .read_named_local(&method)
            .or_else(|| {
                let method = name::AbsolutePath::new(current, &method);
                let method: &DefMethod = root
                    .get_method_from_current(&method)
                    .unwrap();
                let term_args: Vec<interpreter::Value> = term_args
                    .iter()
                    .filter_map(|term_arg| term_arg.evaluate(stack_frame, root, current))
                    .collect();
                let mut stack_frame = interpreter::StackFrame::default()
                    .set_arguments(term_args);
                method.evaluate(&mut stack_frame, root, current)
            })
    }
}

/// # MethodTermList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(reader_inside_method, reader_outside_method)]
pub enum MethodTermList {
    Binary(ByteList),
    SyntaxTree(TermList),
}

impl ReaderInsideMethod for MethodTermList {
    fn read_inside_method<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (term_list, _symbol_aml): (TermList, &[u8]) = TermList::read_inside_method(symbol_aml, root, &current);
        let method_term_list = Self::SyntaxTree(term_list);
        let aml: &[u8] = &aml[method_term_list.length()..];
        (method_term_list, aml)
    }
}

impl ReaderOutsideMethod for MethodTermList {
    fn read_outside_method(&mut self, root: &mut name::Node, current: &name::Path) {
        let aml: Vec<u8> = match self {
            Self::Binary(byte_list) => (&*byte_list).into(),
            Self::SyntaxTree(_) => unreachable!(),
        };
        let (term_list, aml): (TermList, &[u8]) = TermList::read_inside_method(&aml, root, current);
        assert!(aml.is_empty());
        *self = Self::SyntaxTree(term_list);
    }
}

impl Evaluator for MethodTermList {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::Binary(_) => unreachable!(),
            Self::SyntaxTree(term_list) => term_list.evaluate(stack_frame, root, current),
        }
    }
}

/// # MethodOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x14]
pub struct MethodOp;

/// # MidObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct MidObj(TermArg);

impl Evaluator for MidObj {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # MidOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x9e]
pub struct MidOp;

/// # ModOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x85]
pub struct ModOp;

/// # MsecTime
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct MsecTime(TermArg);

impl Evaluator for MsecTime {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # MultiNamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(first_reader, reader, reader_inside_method)]
#[string]
pub struct MultiNamePath(
    #[not_string]
    MultiNamePrefix,
    #[not_string]
    SegCount,
    #[delimiter = "."]
    Vec<NameSeg>,
);

impl FirstReader for MultiNamePath {
    fn first_read<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (multi_name_prefix, symbol_aml): (MultiNamePrefix, &[u8]) = MultiNamePrefix::first_read(symbol_aml, root, &current);
        let (seg_count, symbol_aml): (SegCount, &[u8]) = SegCount::first_read(symbol_aml, root, &current);
        let number_of_name_segs: usize = (&seg_count).into();
        let mut symbol_aml: &[u8] = symbol_aml;
        let mut name_segs: Vec<NameSeg> = Vec::new();
        (0..number_of_name_segs)
            .for_each(|_| {
                let (name_seg, remaining_aml): (NameSeg, &[u8]) = NameSeg::first_read(symbol_aml, root, &current);
                symbol_aml = remaining_aml;
                name_segs.push(name_seg);
            });
        let symbol = Self(
            multi_name_prefix,
            seg_count,
            name_segs,
        );
        let aml: &[u8] = &aml[symbol.length()..];
        (symbol, aml)
    }
}

impl From<&MultiNamePath> for VecDeque<name::Segment> {
    fn from(multi_name_path: &MultiNamePath) -> Self {
        let MultiNamePath(
            _multi_name_prefix,
            _seg_count,
            name_segs,
        ) = multi_name_path;
        name_segs
            .iter()
            .map(|name_seg| name_seg.into())
            .collect()
    }
}

impl Reader for MultiNamePath {
    fn read(aml: &[u8]) -> (Self, &[u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let symbol_aml: &[u8] = aml;
        let (multi_name_prefix, symbol_aml): (MultiNamePrefix, &[u8]) = MultiNamePrefix::read(symbol_aml);
        let (seg_count, mut symbol_aml): (SegCount, &[u8]) = SegCount::read(symbol_aml);
        let number_of_name_segs: usize = (&seg_count).into();
        let mut name_segs: Vec<NameSeg> = Vec::new();
        (0..number_of_name_segs)
            .for_each(|_| {
                let (name_seg, remaining_aml): (NameSeg, &[u8]) = NameSeg::read(symbol_aml);
                symbol_aml = remaining_aml;
                name_segs.push(name_seg);
            });
        let symbol = Self(
            multi_name_prefix,
            seg_count,
            name_segs,
        );
        let aml: &[u8] = &aml[symbol.length()..];
        (symbol, aml)
    }
}

impl ReaderInsideMethod for MultiNamePath {
    fn read_inside_method<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (multi_name_prefix, symbol_aml): (MultiNamePrefix, &[u8]) = MultiNamePrefix::read_inside_method(symbol_aml, root, &current);
        let (seg_count, symbol_aml): (SegCount, &[u8]) = SegCount::read_inside_method(symbol_aml, root, &current);
        let number_of_name_segs: usize = (&seg_count).into();
        let mut symbol_aml: &[u8] = symbol_aml;
        let mut name_segs: Vec<NameSeg> = Vec::new();
        (0..number_of_name_segs)
            .for_each(|_| {
                let (name_seg, remaining_aml): (NameSeg, &[u8]) = NameSeg::read_inside_method(symbol_aml, root, &current);
                symbol_aml = remaining_aml;
                name_segs.push(name_seg);
            });
        let symbol = Self(
            multi_name_prefix,
            seg_count,
            name_segs,
        );
        let aml: &[u8] = &aml[symbol.length()..];
        (symbol, aml)
    }
}

/// # MultiNamePrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x2f]
pub struct MultiNamePrefix;

/// # MultiplyOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x77]
pub struct MultiplyOp;

/// # MutexObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct MutexObject(SuperName);

/// # MutexOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct MutexOp(
    ExtOpPrefix,
    MutexOpSuffix,
);

/// # MutexOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x01]
pub struct MutexOpSuffix;

/// # NAndOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x7c]
pub struct NAndOp;

/// # NOrOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x7e]
pub struct NOrOp;

/// # NameChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[string]
pub enum NameChar {
    DigitChar(DigitChar),
    LeadNameChar(LeadNameChar),
}

/// # NameOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x08]
pub struct NameOp;

/// # NamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[string]
pub enum NamePath {
    Dual(DualNamePath),
    Multi(MultiNamePath),
    NameSeg(NameSeg),
    NullName(NullName),
}

impl From<&NamePath> for VecDeque<name::Segment> {
    fn from(name_path: &NamePath) -> Self {
        match name_path {
            NamePath::Dual(dual_name_path) => dual_name_path.into(),
            NamePath::Multi(multi_name_path) => multi_name_path.into(),
            NamePath::NameSeg(name_seg) => iter::once(name_seg.into()).collect(),
            NamePath::NullName(_null_name) => Self::new(),
        }
    }
}

/// # NameSeg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[string]
pub struct NameSeg(
    LeadNameChar,
    [NameChar; 3],
);

/// # NameSpaceModifierObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum NameSpaceModifierObj {
    Alias(DefAlias),
    Name(DefName),
    Scope(DefScope),
}

impl Evaluator for NameSpaceModifierObj {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::Alias(def_alias) => unimplemented!("def_alias = {:#x?}", def_alias),
            Self::Name(def_name) => def_name.evaluate(stack_frame, root, current),
            Self::Scope(def_scope) => unimplemented!("def_scope = {:#x?}", def_scope),
        }
    }
}

/// # NameString
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(matcher)]
#[string]
pub enum NameString {
    AbsolutePath(
        RootChar,
        NamePath,
    ),
    RelativePath(
        PrefixPath,
        NamePath,
    ),
}

impl From<&NameString> for VecDeque<name::Segment> {
    fn from(name_string: &NameString) -> Self {
        match name_string {
            NameString::AbsolutePath(
                root_char,
                name_path,
            ) => {
                let root_char: name::Segment = root_char.into();
                let name_path: Self = name_path.into();
                iter::once(&root_char)
                    .chain(name_path.iter())
                    .cloned()
                    .collect()
            },
            NameString::RelativePath(
                prefix_path,
                name_path,
            ) => {
                let prefix_path: Self = prefix_path.into();
                let name_path: Self = name_path.into();
                prefix_path
                    .iter()
                    .chain(name_path.iter())
                    .cloned()
                    .collect()
            },
        }
    }
}

impl Holder for NameString {
    fn hold(&self, value: interpreter::Value, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> interpreter::Value {
        let name: name::Path = self.into();
        stack_frame
            .write_named_local(&name, value.clone())
            .unwrap_or_else(|| unimplemented!("self = {:#x?}, value = {:#x?}", self, value))
    }
}

impl Matcher for NameString {
    fn matches(aml: &[u8]) -> bool {
        DualNamePath::matches(aml)
        || MultiNamePath::matches(aml)
        || NameSeg::matches(aml)
        || ParentPrefixChar::matches(aml)
        || RootChar::matches(aml)
    }
}

/// # NamedField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(first_reader, lender, path_getter, reader_inside_method)]
pub struct NamedField(
    NameSeg,
    PkgLength,
);

impl FirstReader for NamedField {
    fn first_read<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (name_seg, symbol_aml): (NameSeg, &[u8]) = NameSeg::first_read(symbol_aml, root, &current);
        let path: name::Path = (&name_seg).into();
        let current: name::Path = current.clone() + path;
        root.add_node(&current, name::Object::NamedField);
        let pkg_length: PkgLength = symbol_aml.into();
        let named_field = Self(
            name_seg,
            pkg_length,
        );
        let aml: &[u8] = &aml[named_field.length()..];
        (named_field, aml)
    }
}

impl Lender for NamedField {
    fn lend<'a>(&'a self, root: &mut reference::Node<'a>, current: &name::Path) {
        let current: name::Path = current.clone() + self
            .get_path()
            .unwrap_or_default();
        let object = reference::Object::NamedField(self);
        root.add_node(&current, object);
        self.iter()
            .for_each(|child| child.lend(root, &current));
    }
}

impl PathGetter for NamedField {
    fn get_path(&self) -> Option<name::Path> {
        let Self(
            name_seg,
            _pkg_length,
        ) = self;
        Some(name_seg.into())
    }
}

impl ReaderInsideMethod for NamedField {
    fn read_inside_method<'a>(aml: &'a [u8], root: &mut name::Node, current: &name::Path) -> (Self, &'a [u8]) {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let current: name::Path = current.clone();
        let symbol_aml: &[u8] = aml;
        let (name_seg, symbol_aml): (NameSeg, &[u8]) = NameSeg::read_inside_method(symbol_aml, root, &current);
        let path: name::Path = (&name_seg).into();
        let current: name::Path = current.clone() + path;
        root.add_node(&current, name::Object::NamedField);
        let pkg_length: PkgLength = symbol_aml.into();
        let named_field = Self(
            name_seg,
            pkg_length,
        );
        let aml: &[u8] = &aml[named_field.length()..];
        (named_field, aml)
    }
}

/// # NamedObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum NamedObj {
    BankField(DefBankField),
    CreateBitField(DefCreateBitField),
    CreateByteField(DefCreateByteField),
    CreateDWordField(DefCreateDWordField),
    CreateField(DefCreateField),
    CreateQWordField(DefCreateQWordField),
    CreateWordField(DefCreateWordField),
    DataRegion(DefDataRegion),
    Device(DefDevice),
    Event(DefEvent),
    External(DefExternal),
    Field(DefField),
    IndexField(DefIndexField),
    Method(DefMethod),
    Mutex(DefMutex),
    OpRegion(DefOpRegion),
    PowerRes(DefPowerRes),
    Processor(DefProcessor),
    ThermalZone(DefThermalZone),
}

/// # NoopOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0xa3]
pub struct NoopOp;

/// # NotOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x80]
pub struct NotOp;

/// # NotifyObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct NotifyObject(SuperName);

/// # NotifyOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x86]
pub struct NotifyOp;

/// # NotifyValue
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct NotifyValue(TermArg);

impl Evaluator for NotifyValue {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # NullChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x00]
pub struct NullChar(char);

/// # NullName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x00]
pub struct NullName(char);

/// # NumBits
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct NumBits(TermArg);

impl Evaluator for NumBits {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}
/// # NumElements
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct NumElements(ByteData);

/// # ObjReference
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ObjReference(TermArg);

impl Evaluator for ObjReference {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # Object
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum Object {
    NameSpaceModifierObj(NameSpaceModifierObj),
    NamedObj(NamedObj),
}

impl Evaluator for Object {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::NameSpaceModifierObj(name_space_modifier_obj) => name_space_modifier_obj.evaluate(stack_frame, root, current),
            Self::NamedObj(named_obj) => unimplemented!("named_obj = {:#x?}", named_obj),
        }
    }
}

/// # ObjectList
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ObjectList(Vec<Object>);

/// # ObjectType
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ObjectType(ByteData);

/// # ObjectTypeEnum
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum ObjectTypeEnum {
    SimpleName(SimpleName),
    DebugObj(DebugObj),
    DefRefOf(DefRefOf),
    DefDerefOf(DefDerefOf),
    DefIndex(DefIndex),
}

/// # ObjectTypeOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x8e]
pub struct ObjectTypeOp;

/// # OneOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x01]
pub struct OneOp;

impl Evaluator for OneOp {
    fn evaluate(&self, _stack_frame: &mut interpreter::StackFrame, _root: &reference::Node, _current: &name::Path) -> Option<interpreter::Value> {
        Some(interpreter::Value::One)
    }
}

/// # OnesOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0xff]
pub struct OnesOp;

impl Evaluator for OnesOp {
    fn evaluate(&self, _stack_frame: &mut interpreter::StackFrame, _root: &reference::Node, _current: &name::Path) -> Option<interpreter::Value> {
        Some(interpreter::Value::Ones)
    }
}

/// # OpRegionOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct OpRegionOp(
    ExtOpPrefix,
    OpRegionOpSuffix,
);

/// # OpRegionOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x80]
pub struct OpRegionOpSuffix;

/// # Operand
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct Operand(TermArg);

impl Evaluator for Operand {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # OrOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x7d]
pub struct OrOp;

/// # PackageElement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum PackageElement {
    NameString(NameString),
    DataRefObject(DataRefObject),
}

impl Evaluator for PackageElement {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::NameString(name_string) => unimplemented!("name_string = {:#x?}", name_string),
            Self::DataRefObject(data_ref_object) => data_ref_object.evaluate(stack_frame, root, current),
        }
    }
}

/// # PackageElementList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct PackageElementList(Vec<PackageElement>);

impl Evaluator for PackageElementList {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(package_elements) = self;
        Some(interpreter::Value::Package(package_elements
            .iter()
            .filter_map(|package_element| package_element.evaluate(stack_frame, root, current))
            .collect()))
    }
}

/// # PackageOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x12]
pub struct PackageOp;

/// # ParentPrefixChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x5e]
pub struct ParentPrefixChar(char);

/// # PblkAddr
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct PblkAddr(DWordData);

/// # PblkLen
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct PblkLen(ByteData);

/// # PkgLeadByte
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.4 Package Length Encoding
#[derive(acpi_machine_language::Analyzer)]
#[bitfield(u8)]
pub struct PkgLeadByte {
    #[bits(6)]
    nybble: u8,
    #[bits(2)]
    byte_data_count: u8,
}

impl PkgLeadByte {
    pub fn byte_data_length(&self) -> usize {
        self.byte_data_count() as usize
    }

    pub fn pkg_length(&self) -> usize {
        self.nybble() as usize
    }
}

/// # PkgLength
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.4 Package Length Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(debug, first_reader, reader, reader_inside_method)]
pub struct PkgLength(
    PkgLeadByte,
    Vec<ByteData>,
);

impl PkgLength {
    pub fn pkg_length(&self) -> usize {
        let Self(
            pkg_lead_byte,
            byte_data,
        ) = self;
        (byte_data
            .iter()
            .rev()
            .fold(0, |length, byte_data| {
                let byte_data: usize = byte_data.into();
                (length << u8::BITS) + byte_data
            }) << 4) + pkg_lead_byte.pkg_length()
    }
}

impl fmt::Debug for PkgLength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("PkgLength")
            .field(&self.pkg_length())
            .finish()
    }
}

impl FirstReader for PkgLength {
    fn first_read<'a>(aml: &'a [u8], _root: &mut name::Node, _current: &name::Path) -> (Self, &'a [u8]) {
        let pkg_length: Self = aml.into();
        let aml: &[u8] = &aml[pkg_length.length()..pkg_length.pkg_length()];
        (pkg_length, aml)
    }
}

impl From<&[u8]> for PkgLength {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let symbol_aml: &[u8] = aml;
        let (pkg_lead_byte, symbol_aml): (PkgLeadByte, &[u8]) = PkgLeadByte::read(symbol_aml);
        let (_symbol_aml, byte_data): (&[u8], Vec<ByteData>) = (0..pkg_lead_byte.byte_data_length())
            .fold((symbol_aml, Vec::new()), |(symbol_aml, mut byte_data), _| {
                let (new_byte_data, symbol_aml): (ByteData, &[u8]) = ByteData::read(symbol_aml);
                byte_data.push(new_byte_data);
                (symbol_aml, byte_data)
            });
        Self(
            pkg_lead_byte,
            byte_data,
        )
    }
}

impl Reader for PkgLength {
    fn read(aml: &[u8]) -> (Self, &[u8]) {
        let pkg_length: Self = aml.into();
        let aml: &[u8] = &aml[pkg_length.length()..pkg_length.pkg_length()];
        (pkg_length, aml)
    }
}

impl ReaderInsideMethod for PkgLength {
    fn read_inside_method<'a>(aml: &'a [u8], _root: &mut name::Node, _current: &name::Path) -> (Self, &'a [u8]) {
        let pkg_length: Self = aml.into();
        let aml: &[u8] = &aml[pkg_length.length()..pkg_length.pkg_length()];
        (pkg_length, aml)
    }
}

/// # PowerResOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct PowerResOp(
    ExtOpPrefix,
    PowerResOpSuffix,
);

/// # PowerResOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x84]
pub struct PowerResOpSuffix;

/// # Predicate
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct Predicate(TermArg);

impl Evaluator for Predicate {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # PrefixPath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 0]
#[string]
pub struct PrefixPath(Vec<ParentPrefixChar>);

impl From<&PrefixPath> for VecDeque<name::Segment> {
    fn from(prefix_path: &PrefixPath) -> Self {
        let PrefixPath(prefix_path) = prefix_path;
        prefix_path
            .iter()
            .map(|parent_prefix_char| parent_prefix_char.into())
            .collect()
    }
}

/// # ProcessorOp
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct ProcessorOp(
    ExtOpPrefix,
    ProcessorOpSuffix,
);

/// # ProcessorOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x83]
pub struct ProcessorOpSuffix;

/// # ProcId
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ProcId(ByteData);

/// # QWordConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct QWordConst(
    QWordPrefix,
    QWordData,
);

impl Evaluator for QWordConst {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _qword_prefix,
            qword_data,
        ) = self;
        qword_data.evaluate(stack_frame, root, current)
    }
}

/// # QWordData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct QWordData(
    [DWordData; 2],
);

impl Evaluator for QWordData {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self([low, high]) = self;
        let low: Option<interpreter::Value> = low.evaluate(stack_frame, root, current);
        let high: Option<interpreter::Value> = high.evaluate(stack_frame, root, current);
        low
            .zip(high)
            .map(|(low, high)| low.concatenate(high))
    }
}

/// # QWordPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x0e]
pub struct QWordPrefix;

/// # Quotient
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct Quotient(Target);

/// # ReferenceTypeOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum ReferenceTypeOpcode {
    DefIndex(DefIndex),
    DerefOf(DefDerefOf),
    RefOf(DefRefOf),
}

/// # RefOfOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x71]
pub struct RefOfOp;

/// # RegionLen
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct RegionLen(TermArg);

impl Evaluator for RegionLen {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # RegionOffset
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct RegionOffset(TermArg);

impl Evaluator for RegionOffset {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # RegionSpace
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value_min = 0x00]
#[encoding_value_max = 0xff]
pub struct RegionSpace(u8);

/// # ReleaseOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct ReleaseOp(
    ExtOpPrefix,
    ReleaseOpSuffix,
);

/// # ReleaseOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x27]
pub struct ReleaseOpSuffix;

/// # Remainder
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct Remainder(Target);

/// # ReservedField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ReservedField(
    ReservedFieldOp,
    PkgLength,
);

/// # ReservedFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x00]
pub struct ReservedFieldOp;

/// # ResetOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct ResetOp(
    ExtOpPrefix,
    ResetOpSuffix,
);

/// # ResetOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x26]
pub struct ResetOpSuffix;

/// # ResourceOrder
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ResourceOrder(WordData);

/// # ReturnOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0xa4]
pub struct ReturnOp;

/// # RevisionOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct RevisionOp(
    ExtOpPrefix,
    RevisionOpSuffix,
);

impl Evaluator for RevisionOp {
    fn evaluate(&self, _stack_frame: &mut interpreter::StackFrame, _root: &reference::Node, _current: &name::Path) -> Option<interpreter::Value> {
        Some(interpreter::Value::Revision)
    }
}


/// # RevisionOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x30]
pub struct RevisionOpSuffix;

/// # RootChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x5c]
pub struct RootChar(char);

/// # ScopeOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x10]
pub struct ScopeOp;

/// # SearchPkg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct SearchPkg(TermArg);

impl Evaluator for SearchPkg {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # SegCount
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct SegCount(ByteData);

impl From<&SegCount> for usize {
    fn from(seg_count: &SegCount) -> Self {
        let SegCount(byte_data) = seg_count;
        byte_data.into()
    }
}

/// # ShiftCount
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ShiftCount(TermArg);

impl Evaluator for ShiftCount {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # ShiftLeftOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x79]
pub struct ShiftLeftOp;

/// # ShiftRightOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x7a]
pub struct ShiftRightOp;

/// # SignalOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct SignalOp(
    ExtOpPrefix,
    SignalOpSuffix,
);

/// # SignalOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x24]
pub struct SignalOpSuffix;

/// # SimpleName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum SimpleName {
    NameString(NameString),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

impl Evaluator for SimpleName {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::NameString(name_string) => unimplemented!("name_string = {:#x?}", name_string),
            Self::ArgObj(arg_obj) => arg_obj.evaluate(stack_frame, root, current),
            Self::LocalObj(local_obj) => local_obj.evaluate(stack_frame, root, current),
        }
    }
}

impl Holder for SimpleName {
    fn hold(&self, value: interpreter::Value, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> interpreter::Value {
        match self {
            Self::NameString(name_string) => name_string.hold(value, stack_frame, root, current),
            Self::ArgObj(arg_obj) => arg_obj.hold(value, stack_frame, root, current),
            Self::LocalObj(local_obj) => local_obj.hold(value, stack_frame, root, current),
        }
    }
}

/// # SizeOfOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x87]
pub struct SizeOfOp;

/// # SleepOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct SleepOp(
    ExtOpPrefix,
    SleepOpSuffix,
);

/// # SleepOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x22]
pub struct SleepOpSuffix;

/// # SourceBuff
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct SourceBuff(TermArg);

impl Evaluator for SourceBuff {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # StatementOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum StatementOpcode {
    Break(DefBreak),
    BreakPoint(DefBreakPoint),
    Continue(DefContinue),
    Fatal(DefFatal),
    IfElse(DefIfElse),
    Noop(DefNoop),
    Notify(DefNotify),
    Release(DefRelease),
    Reset(DefReset),
    Return(DefReturn),
    Signal(DefSignal),
    Sleep(DefSleep),
    Stall(DefStall),
    While(DefWhile),
}

impl Evaluator for StatementOpcode {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::IfElse(def_if_else) => def_if_else.evaluate(stack_frame, root, current),
            Self::Return(def_return) => def_return.evaluate(stack_frame, root, current),
            Self::While(def_while) => def_while.evaluate(stack_frame, root, current),
            _ => unimplemented!("self = {:#x?}", self),
        }
    }
}

/// # StallOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct StallOp(
    ExtOpPrefix,
    StallOpSuffix,
);

/// # StallOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x21]
pub struct StallOpSuffix;

/// # StartIndex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct StartIndex(TermArg);

impl Evaluator for StartIndex {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # StoreOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x70]
pub struct StoreOp;

/// # StringPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x0d]
pub struct StringPrefix;

/// # SubtractOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x74]
pub struct SubtractOp;

/// # SuperName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum SuperName {
    DebugObj(DebugObj),
    ReferenceTypeOpcode(ReferenceTypeOpcode),
    SimpleName(SimpleName),
}

impl Evaluator for SuperName {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::DebugObj(debug_obj) => unreachable!("debug_obj = {:#x?}", debug_obj),
            Self::ReferenceTypeOpcode(reference_type_opcode) => unimplemented!("reference_type_opcode = {:#x?}", reference_type_opcode),
            Self::SimpleName(simple_name) => simple_name.evaluate(stack_frame, root, current),
        }
    }
}

impl Holder for SuperName {
    fn hold(&self, value: interpreter::Value, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> interpreter::Value {
        match self {
            Self::DebugObj(debug_obj) => debug_obj.hold(value, stack_frame, root, current),
            Self::ReferenceTypeOpcode(reference_type_opcode) => unimplemented!("reference_type_opcode = {:#x?}", reference_type_opcode),
            Self::SimpleName(simple_name) => simple_name.hold(value, stack_frame, root, current),
        }
    }
}

/// # SyncFlags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer)]
#[bitfield(u8)]
pub struct SyncFlags {
    #[bits(4)]
    sync_level: u8,
    #[bits(4, access = RO)]
    reserved0: u8,
}

/// # SystemLevel
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct SystemLevel(ByteData);

/// # Target
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum Target {
    NullName(NullName),
    SuperName(Box::<SuperName>),
}

impl Holder for Target {
    fn hold(&self, value: interpreter::Value, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> interpreter::Value {
        match self {
            Self::NullName(_) => value,
            Self::SuperName(super_name) => super_name.hold(value, stack_frame, root, current),
        }
    }
}

/// # TermArg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum TermArg {
    ExpressionOpcode(Box::<ExpressionOpcode>),
    DataObject(Box::<DataObject>),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

impl Evaluator for TermArg {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::ExpressionOpcode(expression_opcode) => expression_opcode.evaluate(stack_frame, root, current),
            Self::DataObject(data_object) => data_object.evaluate(stack_frame, root, current),
            Self::ArgObj(arg_obj) => arg_obj.evaluate(stack_frame, root, current),
            Self::LocalObj(local_obj) => local_obj.evaluate(stack_frame, root, current),
        }
    }
}

/// # TermList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct TermList(
    #[no_leftover]
    Vec<TermObj>,
);

impl Evaluator for TermList {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_objs) = self;
        term_objs
            .iter()
            .for_each(|term_obj| if stack_frame.read_return().is_none() {
                term_obj.evaluate(stack_frame, root, current);
            });
        stack_frame
            .read_return()
            .cloned()
    }
}

/// # TermObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum TermObj {
    ExpressionOpcode(ExpressionOpcode),
    Object(Object),
    StatementOpcode(StatementOpcode),
}

impl Evaluator for TermObj {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        match self {
            Self::ExpressionOpcode(expression_opcode) => expression_opcode.evaluate(stack_frame, root, current),
            Self::Object(object) => object.evaluate(stack_frame, root, current),
            Self::StatementOpcode(statement_opcode) => statement_opcode.evaluate(stack_frame, root, current),
        }
    }
}

/// # ThermalZoneOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct ThermalZoneOp(
    ExtOpPrefix,
    ThermalZoneOpSuffix,
);

/// # ThermalZoneOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x85]
pub struct ThermalZoneOpSuffix;

/// # Timeout
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct Timeout(WordData);

/// # TimerOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct TimerOp(
    ExtOpPrefix,
    TimerOpSuffix,
);

/// # TimerOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x33]
pub struct TimerOpSuffix;

/// # ToBcdOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct ToBcdOp(
    ExtOpPrefix,
    ToBdcOpSuffix,
);

/// # ToBcdOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x29]
pub struct ToBdcOpSuffix;

/// # ToBufferOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x96]
pub struct ToBufferOp;

/// # ToDecimalStringOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x97]
pub struct ToDecimalStringOp;

/// # ToHexStringOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x98]
pub struct ToHexStringOp;

/// # ToIntegerOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x99]
pub struct ToIntegerOp;

/// # ToStringOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x9c]
pub struct ToStringOp;

/// # Underscore
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x5f]
pub struct Underscore(char);

/// # UsecTime
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct UsecTime(TermArg);

impl Evaluator for UsecTime {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # VarPackageOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x13]
pub struct VarPackageOp;

/// # VarNumElements
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct VarNumElements(TermArg);

impl Evaluator for VarNumElements {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(term_arg) = self;
        term_arg.evaluate(stack_frame, root, current)
    }
}

/// # WaitOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 2]
pub struct WaitOp(
    ExtOpPrefix,
    WaitOpSuffix,
);

/// # WaitOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x25]
pub struct WaitOpSuffix;

/// # WhileOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0xa2]
pub struct WhileOp;

/// # WordConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct WordConst(
    WordPrefix,
    WordData,
);

impl Evaluator for WordConst {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self(
            _word_prefix,
            word_data,
        ) = self;
        word_data.evaluate(stack_frame, root, current)
    }
}

/// # WordData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct WordData(
    [ByteData; 2],
);

impl Evaluator for WordData {
    fn evaluate(&self, stack_frame: &mut interpreter::StackFrame, root: &reference::Node, current: &name::Path) -> Option<interpreter::Value> {
        let Self([low, high]) = self;
        let low: Option<interpreter::Value> = low.evaluate(stack_frame, root, current);
        let high: Option<interpreter::Value> = high.evaluate(stack_frame, root, current);
        low
            .zip(high)
            .map(|(low, high)| low.concatenate(high))
    }
}

/// # WordPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x0b]
pub struct WordPrefix;

/// # XOrOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x7f]
pub struct XOrOp;

/// # ZeroOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0x00]
pub struct ZeroOp;

impl Evaluator for ZeroOp {
    fn evaluate(&self, _stack_frame: &mut interpreter::StackFrame, _root: &reference::Node, _current: &name::Path) -> Option<interpreter::Value> {
        Some(interpreter::Value::Zero)
    }
}

