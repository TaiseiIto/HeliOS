//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

use {
    alloc::{
        boxed::Box,
        collections::vec_deque::VecDeque,
        string::String,
        vec::Vec,
    },
    bitfield_struct::bitfield,
    core::{
        fmt,
        iter,
    },
    crate::com2_println,
    super::semantics,
};

pub trait Analyzer: Matcher + MethodAnalyzer + Reader + ReferenceToSymbolIterator + SemanticAnalyzer + WithLength {
}

pub trait Matcher {
    fn matches(aml: &[u8]) -> bool where Self: Sized;
}

pub trait MethodAnalyzer {
    fn analyze_methods(&mut self, root: &semantics::Node, current: semantics::Path);
}

pub trait Reader {
    fn read(aml: &[u8]) -> (Self, &[u8]) where Self: Sized;
}

pub trait ReferenceToSymbolIterator {
    fn iter(&self) -> SymbolIterator<'_>;
    fn iter_mut(&mut self) -> MutSymbolIterator<'_>;
}

pub trait SemanticAnalyzer {
    fn analyze_semantics(&self, root: &mut semantics::Node, current: semantics::Path);
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

/// # ArgObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ArgObject(TermArg);

/// # ArgumentCount
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ArgumentCount(ByteData);

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

/// # BcdValue
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct BcdValue(TermArg);

/// # BitIndex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct BitIndex(TermArg);

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

/// # BuffPkgStrObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct BuffPkgStrObj(TermArg);

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

/// # ByteConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct ByteConst(
    BytePrefix,
    ByteData,
);

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
    BufData(BufData),
    NameString(NameString),
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

/// # DWordData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DWordData(
    [WordData; 2],
);

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

/// # DataRefObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum DataRefObject {
    DataObject(DataObject),
    ObjReference(ObjReference),
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

/// # DefAlias
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefAlias(
    AliasOp,
    [NameString; 2],
);

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

/// # DefDerefOf
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefDerefOf(
    DerefOfOp,
    ObjReference,
);

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
pub struct DefExternal(
    ExternalOp,
    NameString,
    ObjectType,
    ArgumentCount,
);

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

/// # DefIncrement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct DefIncrement(
    IncrementOp,
    SuperName,
);

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
#[manual(semantic_analyzer)]
pub struct DefMethod(
    MethodOp,
    PkgLength,
    NameString,
    MethodFlags,
    #[no_leftover]
    MethodTermList,
);

impl SemanticAnalyzer for DefMethod {
    fn analyze_semantics(&self, root: &mut semantics::Node, current: semantics::Path) {
        let Self(
            _method_op,
            _pkg_length,
            name_string,
            method_flags,
            _method_term_list,
        ) = self;
        let name_string: semantics::Path = name_string.into();
        let number_of_arguments: u8 = method_flags.arg_count();
        root.add_node(current + name_string, semantics::Object::def_method(number_of_arguments));
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

impl From<&DualNamePath> for VecDeque<semantics::Segment> {
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
#[manual(from_slice_u8, matches, semantic_analyzer)]
pub struct MethodInvocation(
    NameString,
    Vec<TermArg>,
);

impl From<&[u8]> for MethodInvocation {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (name_string, mut aml): (NameString, &[u8]) = NameString::read(aml);
        let number_of_arguments: usize = 0;
        let mut term_args: Vec<TermArg> = Vec::new();
        (0..number_of_arguments)
            .for_each(|_| {
                let (term_arg, remaining_aml): (TermArg, &[u8]) = TermArg::read(aml);
                aml = remaining_aml;
                term_args.push(term_arg);
            });
        Self(
            name_string,
            term_args,
        )
    }
}

impl Matcher for MethodInvocation {
    fn matches(aml: &[u8]) -> bool {
        NameString::matches(aml) && !NullName::matches(aml)
    }
}

impl SemanticAnalyzer for MethodInvocation {
    fn analyze_semantics(&self, root: &mut semantics::Node, current: semantics::Path) {
        self.iter()
            .for_each(|child| {
                child.analyze_semantics(root, current.clone());
            });
    }
}

/// # MethodTermList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(method_analyzer)]
pub enum MethodTermList {
    Binary(ByteList),
    SyntaxTree(TermList),
}

impl MethodAnalyzer for MethodTermList {
    fn analyze_methods(&mut self, root: &semantics::Node, current: semantics::Path) {
        let binary: Vec<u8> = match self {
            Self::Binary(byte_list) => {
                let byte_list: ByteList = byte_list.clone();
                (&byte_list).into()
            },
            Self::SyntaxTree(_term_list) => unreachable!(),
        };
        com2_println!("analyze methods current = {:#x?}", current);
        com2_println!("binary = {:#x?}", binary);
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

/// # MultiNamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(from_slice_u8)]
#[string]
pub struct MultiNamePath(
    #[not_string]
    MultiNamePrefix,
    #[not_string]
    SegCount,
    #[delimiter = "."]
    Vec<NameSeg>,
);

impl From<&MultiNamePath> for VecDeque<semantics::Segment> {
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

impl From<&[u8]> for MultiNamePath {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (multi_name_prefix, aml): (MultiNamePrefix, &[u8]) = MultiNamePrefix::read(aml);
        let (seg_count, mut aml): (SegCount, &[u8]) = SegCount::read(aml);
        let number_of_name_segs: usize = (&seg_count).into();
        let mut name_segs: Vec<NameSeg> = Vec::new();
        (0..number_of_name_segs)
            .for_each(|_| {
                let (name_seg, remaining_aml): (NameSeg, &[u8]) = NameSeg::read(aml);
                aml = remaining_aml;
                name_segs.push(name_seg);
            });
        Self(
            multi_name_prefix,
            seg_count,
            name_segs,
        )
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

impl From<&NamePath> for VecDeque<semantics::Segment> {
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

/// # NameString
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[string]
pub enum NameString {
    AbsolutePath(
        RootChar,
        NamePath,
    ),
    #[matching_type = "ParentPrefixChar"]
    #[matching_type = "NamePath"]
    RelativePath(
        PrefixPath,
        NamePath,
    ),
}

impl From<&NameString> for VecDeque<semantics::Segment> {
    fn from(name_string: &NameString) -> Self {
        match name_string {
            NameString::AbsolutePath(
                root_char,
                name_path,
            ) => {
                let root_char: semantics::Segment = root_char.into();
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

/// # NamedField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[manual(semantic_analyzer)]
pub struct NamedField(
    NameSeg,
    PkgLength,
);

impl SemanticAnalyzer for NamedField {
    fn analyze_semantics(&self, root: &mut semantics::Node, current: semantics::Path) {
        let Self(
            name_seg,
            _pkg_length,
        ) = self;
        let name_seg: semantics::Path = name_seg.into();
        root.add_node(current + name_seg, semantics::Object::NamedField);
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

/// # Object
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum Object {
    NameSpaceModifierObj(NameSpaceModifierObj),
    NamedObj(NamedObj),
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

/// # OnesOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[encoding_value = 0xff]
pub struct OnesOp;

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
    DataRefObject(DataRefObject),
    NameString(NameString),
}

/// # PackageElementList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct PackageElementList(Vec<PackageElement>);

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
#[manual(debug, from_slice_u8, reader)]
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

impl From<&[u8]> for PkgLength {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let pkg_lead_byte: PkgLeadByte = aml.into();
        let aml: &[u8] = &aml[pkg_lead_byte.length()..];
        let (_aml, byte_data): (&[u8], Vec<ByteData>) = (0..pkg_lead_byte.byte_data_length())
            .fold((aml, Vec::new()), |(aml, mut byte_data), _| {
                let (new_byte_data, aml): (ByteData, &[u8]) = ByteData::read(aml);
                byte_data.push(new_byte_data);
                (aml, byte_data)
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

/// # PrefixPath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
#[matching_elements = 0]
#[string]
pub struct PrefixPath(Vec<ParentPrefixChar>);

impl From<&PrefixPath> for VecDeque<semantics::Segment> {
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

/// # QWordData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct QWordData(
    [DWordData; 2],
);

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

/// # RegionOffset
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct RegionOffset(TermArg);

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

/// # TermList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct TermList(
    #[no_leftover]
    Vec<TermObj>
);

/// # TermObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub enum TermObj {
    ExpressionOpcode(ExpressionOpcode),
    Object(Object),
    StatementOpcode(StatementOpcode),
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

/// # WordData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Analyzer, Clone)]
pub struct WordData(
    [ByteData; 2],
);

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

