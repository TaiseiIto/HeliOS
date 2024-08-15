//! # ACPI Machine Lnaguage
//! ## References
//! * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20 ACPI MACHINE LANGUAGE (AML) SPECIFICATION

use {
    alloc::{
        boxed::Box,
        string::String,
        vec,
        vec::Vec,
    },
    bitfield_struct::bitfield,
    core::fmt,
};

pub trait Reader<'a>: From<&'a [u8]> {
    fn length(&self) -> usize;
    fn matches(aml: &[u8]) -> bool;
    fn read(aml: &'a [u8]) -> (Self, &'a [u8]);
}

/// # AcquireOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[matching_elements = 2]
pub struct AcquireOp(
    ExtOpPrefix,
    AcquireOpSuffix,
);

/// # AcquireOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x23]
pub struct AcquireOpSuffix;

/// # AddOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x72]
pub struct AddOp;

/// # AndOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x7b]
pub struct AndOp;

/// # Arg Objects Encoding
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.1 Arg Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value_min = 0x68]
#[encoding_value_max = 0x6e]
pub struct ArgObj(u8);

/// # ArgObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct ArgObject(TermArg);

/// # AsciiChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value_min = 0x01]
#[encoding_value_max = 0x7f]
pub struct AsciiChar(char);

/// # AsciiCharList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[matching_elements = 0]
#[string]
pub struct AsciiCharList(Vec<AsciiChar>);

/// # AsciiUppercase
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value_min = 0x41]
#[encoding_value_max = 0x5a]
pub struct AsciiUppercase(char);

/// # BreakOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0xa5]
pub struct BreakOp;

/// # BuffPkgStrObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct BuffPkgStrObj(Box<TermArg>);

/// # BufferOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x11]
pub struct BufferOp;

/// # BufferSize
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct BufferSize(Box<TermArg>);

/// # ByteConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct ByteConst(
    BytePrefix,
    ByteData,
);

/// # ByteData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value_min = 0x00]
#[encoding_value_max = 0xff]
pub struct ByteData(u8);

impl From<&ByteData> for usize {
    fn from(byte_data: &ByteData) -> Self {
        let ByteData(byte_data) = byte_data;
        *byte_data as Self
    }
}

/// # ByteIndex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct ByteIndex(TermArg);

/// # ByteList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct ByteList(Vec<ByteData>);

/// # BytePrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x0a]
pub struct BytePrefix;

/// # Circumflex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x5e]
pub struct Circumflex(char);

/// # ComputationalData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum ComputationalData {
    ByteConst(ByteConst),
    ConstObj(ConstObj),
    DWordConst(DWordConst),
    AmlString(AmlString),
    WordConst(WordConst),
}

/// # ConstObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum ConstObj {
    OneOp(OneOp),
    ZeroOp(ZeroOp),
}

/// # CreateDWordFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x8a]
pub struct CreateDWordFieldOp;

/// # DWordConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DWordConst(
    DWordPrefix,
    DWordData,
);

/// # DWordData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DWordData(
    [WordData; 2],
);

/// # DWordPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x0c]
pub struct DWordPrefix;

/// # DataObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum DataObject {
    ComputationalData(ComputationalData),
    DefPackage(DefPackage),
}

/// # DataRefObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum DataRefObject {
    DataObject(DataObject),
    ObjReference(ObjReference),
}

/// # DebugObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.3 Debug Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DebugObj(DebugOp);

/// # DebugOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.3 Debug Opects Encoding
#[derive(acpi_machine_language::Reader)]
#[matching_elements = 2]
pub struct DebugOp(
    ExtOpPrefix,
    DebugOpSuffix,
);

/// # DebugOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.3 Debug Opects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x31]
pub struct DebugOpSuffix;

/// # DefAcquire
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefAcquire(
    AcquireOp,
    MutexObject,
    Timeout,
);

/// # DefAdd
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefAdd(
    AddOp,
    [Operand; 2],
    Target,
);

/// # DefAnd
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefAnd(
    AndOp,
    [Operand; 2],
    Target,
);

/// # DefBreak
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefBreak(BreakOp);

/// # DefBuffer
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefBuffer(
    BufferOp,
    PkgLength,
    BufferSize,
    #[no_leftover]
    ByteList,
);

/// # DefCreateDWordField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefCreateDWordField(
    CreateDWordFieldOp,
    SourceBuff,
    ByteIndex,
    NameString,
);


/// # DefDerefOf
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefDerefOf(
    DerefOfOp,
    ObjReference,
);

/// # DefDevice
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefDevice(
    DeviceOp,
    PkgLength,
    NameString,
    #[no_leftover]
    TermList,
);

/// # DefElse
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefElse(
    ElseOp,
    PkgLength,
    #[no_leftover]
    TermList,
);

/// # DefField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefField(
    FieldOp,
    PkgLength,
    NameString,
    FieldFlags,
    #[no_leftover]
    FieldList,
);

/// # DefIf
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
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
#[derive(acpi_machine_language::Reader)]
pub struct DefIfElse(
    DefIf,
    Option<DefElse>,
);

/// # DefIncrement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefIncrement(
    IncrementOp,
    SuperName,
);

/// # DefIndex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefIndex(
    IndexOp,
    BuffPkgStrObj,
    IndexValue,
    Box<Target>,
);

/// # DefLAnd
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefLAnd(
    LAndOp,
    [Operand; 2],
);

/// # DefLEqual
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefLEqual(
    LEqualOp,
    [Operand; 2],
);

/// # DefLGreater
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefLGreater(
    LGreaterOp,
    [Operand; 2],
);

/// # DefLLess
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefLLess(
    LLessOp,
    [Operand; 2],
);

/// # DefLNot
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefLNot(
    LNotOp,
    Operand,
);

/// # DefLOr
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefLOr(
    LOrOp,
    [Operand; 2],
);

/// # DefMethod
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefMethod(
    MethodOp,
    PkgLength,
    NameString,
    MethodFlags,
    #[no_leftover]
    TermList,
);

/// # DefMutex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefMutex(
    MutexOp,
    NameString,
    SyncFlags,
);

/// # DefName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefName(
    NameOp,
    NameString,
    DataRefObject,
);

/// # DefNotify
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefNotify(
    NotifyOp,
    NotifyObject,
    NotifyValue,
);

/// # DefOpRegion
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefOpRegion(
    OpRegionOp,
    NameString,
    RegionSpace,
    RegionOffset,
    RegionLen,
);

/// # DefOr
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefOr(
    OrOp,
    [Operand; 2],
    Target,
);

/// # DefPackage
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefPackage(
    PackageOp,
    PkgLength,
    NumElements,
    #[no_leftover]
    PackageElementList,
);

/// # DefProcessor
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
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

/// # DefRelease
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefRelease(
    ReleaseOp,
    MutexObject,
);

/// # DefReturn
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefReturn(
    ReturnOp,
    ArgObject,
);

/// # DefScope
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Reader)]
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
#[derive(acpi_machine_language::Reader)]
pub struct DefShiftLeft(
    ShiftLeftOp,
    Operand,
    ShiftCount,
    Target,
);

/// # DefShiftRight
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefShiftRight(
    ShiftRightOp,
    Operand,
    ShiftCount,
    Target,
);

/// # DefSizeOf
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefSizeOf(
    SizeOfOp,
    SuperName,
);

/// # DefStore
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefStore(
    StoreOp,
    Box<TermArg>,
    SuperName,
);

/// # DefSubtract
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefSubtract(
    SubtractOp,
    [Operand; 2],
    Target,
);

/// # DefToBuffer
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefToBuffer(
    ToBufferOp,
    Operand,
    Target,
);

/// # DefToHexString
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefToHexString(
    ToHexStringOp,
    Operand,
    Target,
);

/// # DefWhile
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefWhile(
    WhileOp,
    PkgLength,
    Predicate,
    #[no_leftover]
    TermList,
);

/// # DerefOfOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x83]
pub struct DerefOfOp;

/// # DeviceOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[matching_elements = 2]
pub struct DeviceOp(
    ExtOpPrefix,
    DeviceOpSuffix,
);

/// # DeviceOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x82]
pub struct DeviceOpSuffix;

/// # DigitChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value_min = 0x30]
#[encoding_value_max = 0x39]
pub struct DigitChar(char);

/// # DualNamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[string]
pub struct DualNamePath(
    #[not_string]
    DualNamePrefix,
    #[delimiter = "."]
    [NameSeg; 2],
);

/// # DualNamePrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x2e]
pub struct DualNamePrefix;

/// # ElseOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0xa1]
pub struct ElseOp;

/// # ExpressionOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub enum ExpressionOpcode {
    Acquire(DefAcquire),
    Add(DefAdd),
    And(DefAnd),
    Buffer(DefBuffer),
    DerefOf(DefDerefOf),
    Increment(DefIncrement),
    Index(DefIndex),
    LAnd(DefLAnd),
    LEqual(DefLEqual),
    LGreater(DefLGreater),
    LLess(DefLLess),
    LNot(DefLNot),
    LOr(DefLOr),
    MethodInvocation(MethodInvocation),
    Or(DefOr),
    Package(DefPackage),
    ShiftLeft(DefShiftLeft),
    ShiftRight(DefShiftRight),
    SizeOf(DefSizeOf),
    Store(DefStore),
    Subtract(DefSubtract),
    ToBuffer(DefToBuffer),
    ToHexString(DefToHexString),
}

/// # ExtOpPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x5b]
pub struct ExtOpPrefix;

/// # FieldElement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum FieldElement {
    Named(NamedField),
    Reserved(ReservedField),
}

/// # FieldFlags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[flags]
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
#[derive(acpi_machine_language::Reader)]
pub struct FieldList(Vec<FieldElement>);

/// # FieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[matching_elements = 2]
pub struct FieldOp(
    ExtOpPrefix,
    FieldOpSuffix,
);

/// # FieldOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x81]
pub struct FieldOpSuffix;

/// # IfOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0xa0]
pub struct IfOp;

/// # IncrementOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x75]
pub struct IncrementOp;

/// # IndexOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x88]
pub struct IndexOp;

/// # IndexValue
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct IndexValue(Box<TermArg>);

/// # LAndOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x90]
pub struct LAndOp;

/// # LGreaterOp
/// # LEqualOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x93]
pub struct LEqualOp;

/// # LGreaterOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x94]
pub struct LGreaterOp;

/// # LLessOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x95]
pub struct LLessOp;

/// # LNotOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x92]
pub struct LNotOp;

/// # LOrOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x91]
pub struct LOrOp;

/// # LeadNameChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[string]
pub enum LeadNameChar {
    AsciiUppercase(AsciiUppercase),
    Underscore(Underscore),
}

/// # Local Objects Encoding
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.2 Local Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value_min = 0x60]
#[encoding_value_max = 0x67]
pub struct LocalObj(u8);

/// # MethodFlags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[flags]
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
pub struct MethodInvocation(
    NameString,
    Vec<TermArg>,
);

impl fmt::Debug for MethodInvocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("MethodInvocation");
        let Self(
            name_string,
            term_args,
        ) = self;
        debug_tuple.field(name_string);
        term_args
            .iter()
            .for_each(|term_arg| {
                debug_tuple.field(term_arg);
            });
        debug_tuple.finish()
    }
}

impl From<&[u8]> for MethodInvocation {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (name_string, mut aml): (NameString, &[u8]) = NameString::read(aml);
        let method_name: String = name_string
            .name_path()
            .name_segs()
            .into_iter()
            .last()
            .unwrap()
            .into();
        let number_of_term_args: usize = match method_name.as_str() {
            "ADDR"
            | "AWAK"
            | "BFEA"
            | "BRDS"
            | "BTNC"
            | "BTNL"
            | "BTNS"
            | "CDAT"
            | "CINS"
            | "CNEW"
            | "CPEN"
            | "CRMV"
            | "CSCN"
            | "DLSI"
            | "DLSW"
            | "DQUE"
            | "DVHP"
            | "EWRD"
            | "EXFG"
            | "GPEH"
            | "HDDM"
            | "HDEM"
            | "HDMM"
            | "HEBC"
            | "HEEC"
            | "HPME"
            | "LNKA"
            | "LNKB"
            | "LNKC"
            | "LNKD"
            | "LNKS"
            | "MSWN"
            | "NHPG"
            | "NPME"
            | "OSHP"
            | "PCNT"
            | "PCS"
            | "PIDX"
            | "PNOT"
            | "PPRT"
            | "PRD"
            | "PRQ0"
            | "PRQ1"
            | "PRQ2"
            | "PRQ3"
            | "PRR0"
            | "PRRI"
            | "PWPR"
            | "PWRR"
            | "P_CS"
            | "ROMK"
            | "ROMS"
            | "RPL1"
            | "SADS"
            | "SIOH"
            | "SPL1"
            | "SPLC"
            | "SPRS"
            | "THEN"
            | "VEND"
            | "VMAP"
            | "WAK"
            | "WGDS"
            | "WIST"
            | "WRDD"
            | "WRDS"
            | "XDBA"
            | "^BN00"
            | "_AC0"
            | "_ADR"
            | "_BBN"
            | "_BIF"
            | "_BST"
            | "_CID"
            | "_CRS"
            | "_CRT"
            | "_DEP"
            | "_DIS"
            | "_DOD"
            | "_E01"
            | "_E02"
            | "_FIT"
            | "_GPE"
            | "_HID"
            | "_INI"
            | "_L00"
            | "_L01"
            | "_L02"
            | "_L03"
            | "_L08"
            | "_L09"
            | "_L0C"
            | "_L0D"
            | "_L0E"
            | "_L0F"
            | "_L26"
            | "_L41"
            | "_LID"
            | "_MAT"
            | "_NBS"
            | "_NCH"
            | "_OFF"
            | "_ON"
            | "_PCL"
            | "_PLD"
            | "_PRR"
            | "_PRS"
            | "_PRT"
            | "_PRW"
            | "_PS0"
            | "_PS3"
            | "_PSL"
            | "_PSR"
            | "_PSV"
            | "_PXM"
            | "_Q02"
            | "_Q06"
            | "_Q07"
            | "_Q08"
            | "_Q0A"
            | "_Q0B"
            | "_Q0C"
            | "_Q0D"
            | "_Q54"
            | "_Q79"
            | "_Q85"
            | "_Q8A"
            | "_QD5"
            | "_QD6"
            | "_QF0"
            | "_RMV"
            | "_RST"
            | "_S0W"
            | "_S1D"
            | "_S2D"
            | "_S3D"
            | "_S3W"
            | "_S4D"
            | "_S4W"
            | "_STA"
            | "_STR"
            | "_SWS"
            | "_TC1"
            | "_TC2"
            | "_TMP"
            | "_TSP"
            | "_UPC" => 0,
            "ADBG"
            | "ANGL"
            | "APID"
            | "BRTN"
            | "BTNE"
            | "CALK"
            | "CALS"
            | "CEJ0"
            | "CGLD"
            | "CMAT"
            | "CPXM"
            | "CRDT"
            | "CSTA"
            | "CVTD"
            | "CVTT"
            | "DBG"
            | "DBUG"
            | "DSTA"
            | "ECMD"
            | "ECRD"
            | "ENFG"
            | "ESTA"
            | "FCTL"
            | "FIT1"
            | "GGCM"
            | "GGIV"
            | "GGOV"
            | "GINX"
            | "GPC0"
            | "GPC1"
            | "GPOF"
            | "GUAM"
            | "HALL"
            // "HDSM" => 1,
            | "HEX"
            | "HEX2"
            | "HEX4"
            | "HPEM"
            | "IQCR"
            | "IQST"
            // "LCRS" => 1,
            | "LDIS"
            | "LSRS"
            | "LSTA"
            | "PBAD"
            | "PCIC"
            | "PPCK"
            | "PPTS"
            | "PSHP"
            | "PSTA"
            | "PWAK"
            | "S2BF"
            // "SCRS" => 1,
            | "SDIS"
            | "SIOS"
            | "SIOW"
            | "SKBC"
            | "SLEN"
            | "SPTS"
            | "SSTA"
            | "SWAK"
            | "THDD"
            | "THDH"
            | "THDS"
            | "TPTS"
            | "TUPC"
            | "UXDV"
            | "VMBB"
            | "VMBS"
            | "VMPS"
            | "_DOS"
            | "_EJ0"
            | "_PIC"
            | "_PSW"
            | "_PTS"
            | "_SCP"
            | "_SRS"
            | "_WAK" => 1,
            "AIDX"
            | "BEJ0"
            | "CNOT"
            | "CPRS"
            | "CTFY"
            | "DCNT"
            | "DCR2"
            | "DCR3"
            | "DCRS"
            | "DNXX"
            | "DSR2"
            | "DSR3"
            | "DSRS"
            | "DVNT"
            | "ECWT"
            | "GPLD"
            | "GPRW"
            | "GUPC"
            | "IVOC"
            | "L1MX"
            | "MIN"
            | "MTCH"
            | "P8XH"
            | "PCEJ"
            | "PCFG"
            | "PPEX"
            // "PPHR" => 2,
            | "SCMP"
            | "SCPG"
            | "SGOV"
            | "SHPO"
            | "SPC0"
            | "SPC1"
            | "SPPS"
            | "SSRS"
            | "STRC"
            | "THDA"
            | "TPLD"
            | "_REG" => 2,
            "CRYF"
            | "DLSR"
            | "IPCM"
            // "LCRS" => 3,
            | "PEJ0"
            // "PPHR" => 3,
            | "RDMA"
            | "SCOM"
            | "STRD"
            | "STRT"
            | "TSTM"
            | "_DSW"
            | "_OST" => 3,
            "COST"
            | "LPRS"
            | "PCID"
            | "RRIO"
            // "SCRS" => 4,
            | "SOSC"
            | "XPRS"
            | "_DSM"
            | "_OSC" => 4,
            "BDSM"
            | "DDSM"
            | "EDSM"
            // "HDSM" => 5,
            | "MDSM" => 5,
            // "PDSM" => 5,
            // "PDSM" => 6,
            | "XRES" => 6,
            unknown_method_name => panic!("Unknown method {:#x?}", unknown_method_name),
        };
        let mut term_args: Vec<TermArg> = Vec::new();
        (0..number_of_term_args)
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

impl Reader<'_> for MethodInvocation {
    fn length(&self) -> usize {
        let Self(
            name_string,
            term_args,
        ) = self;
        name_string.length() + term_args
            .iter()
            .map(|term_arg| term_arg.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        NameString::matches(aml) && !NullName::matches(aml)
    }

    fn read(aml: &[u8]) -> (Self, &[u8]) {
        let method_invocation: Self = aml.into();
        let aml: &[u8] = &aml[method_invocation.length()..];
        (method_invocation, aml)
    }
}

/// # MethodOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x14]
pub struct MethodOp;

/// # MultiNamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub struct MultiNamePath(
    MultiNamePrefix,
    SegCount,
    Vec<NameSeg>,
);

impl fmt::Debug for MultiNamePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let multi_name_path: String = self.into();
        formatter
            .debug_tuple("MultiNamePath")
            .field(&multi_name_path)
            .finish()
    }
}

impl From<&MultiNamePath> for String {
    fn from(multi_name_path: &MultiNamePath) -> Self {
        let MultiNamePath(
            _multi_name_prefix,
            _seg_count,
            name_segs,
        ) = multi_name_path;
        let name_segs: Vec<String> = name_segs
            .iter()
            .map(|name_seg| name_seg.into())
            .collect();
        name_segs.join(".")
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

impl Reader<'_> for MultiNamePath {
    fn length(&self) -> usize {
        let Self(
            multi_name_prefix,
            seg_count,
            name_segs,
        ) = self;
        multi_name_prefix.length() + seg_count.length() + name_segs
            .iter()
            .map(|name_seg| name_seg.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        MultiNamePrefix::matches(aml)
    }

    fn read(aml: &[u8]) -> (Self, &[u8]) {
        let multi_name_prefix: Self = aml.into();
        let aml: &[u8] = &aml[multi_name_prefix.length()..];
        (multi_name_prefix, aml)
    }
}

/// # MultiNamePrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x2f]
pub struct MultiNamePrefix;

/// # MutexObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct MutexObject(SuperName);

/// # MutexOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[matching_elements = 2]
pub struct MutexOp(
    ExtOpPrefix,
    MutexOpSuffix,
);

/// # MutexOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x01]
pub struct MutexOpSuffix;

/// # NameChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[string]
pub enum NameChar {
    DigitChar(DigitChar),
    LeadNameChar(LeadNameChar),
}

/// # NameOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x08]
pub struct NameOp;

/// # NamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[string]
pub enum NamePath {
    Dual(DualNamePath),
    Multi(MultiNamePath),
    NameSeg(NameSeg),
    NullName(NullName),
}

impl NamePath {
    fn name_segs(&self) -> Vec<&NameSeg> {
        match self {
            Self::Dual(DualNamePath(
                _dual_name_path,
                name_segs,
            )) => name_segs
                .iter()
                .collect(),
            Self::Multi(MultiNamePath(
                _multi_name_prefix,
                _seg_count,
                name_segs,
            )) => name_segs
                .iter()
                .collect(),
            Self::NameSeg(name_seg) => vec![name_seg],
            Self::NullName(_) => Vec::new(),
        }
    }
}

/// # NameSeg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[string]
pub struct NameSeg(
    LeadNameChar,
    [NameChar; 3],
);

/// # NameSpaceModifierObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum NameSpaceModifierObj {
    DefName(DefName),
    DefScope(DefScope),
}

/// # NameString
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[string]
pub enum NameString {
    RootCharNamePath(
        RootChar,
        NamePath,
    ),
    #[matching_type = "Circumflex"]
    #[matching_type = "NamePath"]
    PrefixPathNamePath(
        PrefixPath,
        NamePath,
    ),
}

impl NameString {
    fn name_path(&self) -> &NamePath {
        match self {
            Self::RootCharNamePath(
                _root_char,
                name_path,
            ) => name_path,
            Self::PrefixPathNamePath(
                _prefix_path,
                name_path,
            ) => name_path,
        }
    }
}

/// # NamedField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct NamedField(
    NameSeg,
    PkgLength,
);

/// # NamedObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum NamedObj {
    CreateDWordField(DefCreateDWordField),
    Device(DefDevice),
    Field(DefField),
    Method(DefMethod),
    Mutex(DefMutex),
    OpRegion(DefOpRegion),
    Processor(DefProcessor),
}

/// # NotifyObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct NotifyObject(SuperName);

/// # NotifyOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x86]
pub struct NotifyOp;

/// # NotifyValue
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct NotifyValue(TermArg);

/// # NullChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x00]
pub struct NullChar(char);

/// # NullName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x00]
pub struct NullName(char);

/// # NumElements
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct NumElements(ByteData);

/// # ObjReference
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct ObjReference(Box<TermArg>);

/// # Object
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum Object {
    NameSpaceModifierObj(NameSpaceModifierObj),
    NamedObj(NamedObj),
}

/// # ObjectList
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct ObjectList(Vec<Object>);

/// # OneOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x01]
pub struct OneOp;

/// # OpRegionOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[matching_elements = 2]
pub struct OpRegionOp(
    ExtOpPrefix,
    OpRegionOpSuffix,
);

/// # OpRegionOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x80]
pub struct OpRegionOpSuffix;

/// # Operand
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct Operand(Box<TermArg>);

/// # OrOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x7d]
pub struct OrOp;

/// # PackageElement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub enum PackageElement {
    DataRefObject(DataRefObject),
    NameString(NameString),
}

/// # PackageElementList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct PackageElementList(Vec<PackageElement>);

/// # PackageOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x12]
pub struct PackageOp;

/// # PblkAddr
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct PblkAddr(DWordData);

/// # PblkLen
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct PblkLen(ByteData);

/// # PkgLeadByte
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.4 Package Length Encoding
#[derive(acpi_machine_language::Reader)]
#[flags]
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
pub struct PkgLength {
    pkg_lead_byte: PkgLeadByte,
    byte_data: Vec<ByteData>,
}

impl PkgLength {
    pub fn pkg_length(&self) -> usize {
        let Self {
            pkg_lead_byte,
            byte_data,
        } = self;
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
        Self {
            pkg_lead_byte,
            byte_data,
        }
    }
}

impl Reader<'_> for PkgLength {
    fn length(&self) -> usize {
        let Self {
            pkg_lead_byte,
            byte_data
        } = self;
        pkg_lead_byte.length() + byte_data
            .iter()
            .map(|byte_data| byte_data.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        PkgLeadByte::matches(aml)
    }

    fn read(aml: &[u8]) -> (Self, &[u8]) {
        let pkg_length: Self = aml.into();
        let aml: &[u8] = &aml[pkg_length.length()..pkg_length.pkg_length()];
        (pkg_length, aml)
    }
}

/// # Predicate
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct Predicate(TermArg);

/// # PrefixPath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[matching_elements = 0]
#[string]
pub struct PrefixPath(Vec<Circumflex>);

/// # ProcessorOp
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[matching_elements = 2]
pub struct ProcessorOp(
    ExtOpPrefix,
    ProcessorOpSuffix,
);

/// # ProcessorOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x83]
pub struct ProcessorOpSuffix;

/// # ProcId
/// ## References
/// * [Advanced Configuration and Power Interface Specification](https://uefi.org/sites/default/files/resources/ACPI_5_1release.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct ProcId(ByteData);


/// # ReferenceTypeOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub enum ReferenceTypeOpcode {
    DefIndex(DefIndex),
}

/// # RegionLen
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct RegionLen(TermArg);

/// # RegionOffset
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct RegionOffset(TermArg);

/// # RegionSpace
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value_min = 0x00]
#[encoding_value_max = 0xff]
pub struct RegionSpace(u8);

/// # ReleaseOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[matching_elements = 2]
pub struct ReleaseOp(
    ExtOpPrefix,
    ReleaseOpSuffix,
);

/// # ReleaseOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x27]
pub struct ReleaseOpSuffix;

/// # ReservedField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct ReservedField(
    ReservedFieldOp,
    PkgLength,
);

/// # ReservedFieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x00]
pub struct ReservedFieldOp;

/// # ReturnOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0xa4]
pub struct ReturnOp;

/// # RootChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x5c]
pub struct RootChar(char);

/// # ScopeOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x10]
pub struct ScopeOp;

/// # SegCount
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
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
#[derive(acpi_machine_language::Reader)]
pub struct ShiftCount(Box<TermArg>);

/// # ShiftLeftOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x79]
pub struct ShiftLeftOp;

/// # ShiftRightOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x7a]
pub struct ShiftRightOp;

/// # SimpleName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum SimpleName {
    NameString(NameString),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

/// # SizeOfOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x87]
pub struct SizeOfOp;

/// # SourceBuff
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct SourceBuff(TermArg);


/// # StatementOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub enum StatementOpcode {
    Break(DefBreak),
    IfElse(DefIfElse),
    Notyfy(DefNotify),
    Release(DefRelease),
    Return(DefReturn),
    While(DefWhile),
}

/// # StoreOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x70]
pub struct StoreOp;

/// # String
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[string]
pub struct AmlString(
    #[not_string]
    StringPrefix,
    AsciiCharList,
    NullChar,
);

/// # StringPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x0d]
pub struct StringPrefix;

/// # SubtractOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x74]
pub struct SubtractOp;


/// # SuperName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum SuperName {
    DebugObj(DebugObj),
    ReferenceTypeOpcode(ReferenceTypeOpcode),
    SimpleName(SimpleName),
}

/// # SyncFlags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[flags]
#[bitfield(u8)]
pub struct SyncFlags {
    #[bits(4)]
    sync_level: u8,
    #[bits(4, access = RO)]
    reserved0: u8,
}

/// # Target
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum Target {
    NullName(NullName),
    SuperName(SuperName),
}

/// # TermArg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum TermArg {
    ExpressionOpcode(ExpressionOpcode),
    DataObject(DataObject),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

/// # TermList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct TermList(
    #[debug]
    #[no_leftover]
    Vec<TermObj>
);

/// # TermObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum TermObj {
    ExpressionOpcode(ExpressionOpcode),
    Object(Object),
    StatementOpcode(StatementOpcode),
}

/// # Timeout
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct Timeout(WordData);

/// # ToBufferOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x96]
pub struct ToBufferOp;

/// # ToHexStringOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x98]
pub struct ToHexStringOp;

/// # Underscore
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x5f]
pub struct Underscore(char);

/// # WhileOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0xa2]
pub struct WhileOp;

/// # WordConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct WordConst(
    WordPrefix,
    WordData,
);

/// # WordData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct WordData(
    [ByteData; 2],
);

/// # WordPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x0b]
pub struct WordPrefix;

/// # ZeroOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value = 0x00]
pub struct ZeroOp;

