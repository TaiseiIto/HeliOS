//! # ELF Header
//! ## References
//! * [Tool Interface Standard (TIS) Executable and Linking Format (ELF) Specification](https://refspecs.linuxfoundation.org/elf/elf.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

use super::{
    Addr,
    Half,
    Off,
    Word,
    UnsignedChar,
};

/// # ELF Header
/// ## References
/// * [Tool Interface Standard (TIS) Executable and Linking Format (ELF) Specification](https://refspecs.linuxfoundation.org/elf/elf.pdf)
/// * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    e_ident: Ei,
    e_type: Et,
    e_machine: Em,
    e_version: Ev,
    e_entry: Addr,
    e_phoff: Off,
    e_shoff: Off,
    e_flags: Word,
    e_ehsize: Half,
    e_phentsize: Half,
    e_phnum: Half,
    e_shentsize: Half,
    e_shnum: Half,
    e_shstrndx: Half,
}

#[derive(Debug)]
#[repr(C)]
struct Ei {
    mag: [UnsignedChar; 4],
    class: Class,
    data: Data,
    version: UnsignedChar,
    osabi: Osabi,
    abiversion: UnsignedChar,
    pad: [UnsignedChar; 7],
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u8)]
enum Class {
    Invalid = 0,
    Bit32 = 1,
    Bit64 = 2,
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u8)]
enum Data {
    None = 0,
    LittleEndian = 1,
    BigEndian = 2,
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u8)]
enum Osabi {
    SystemV = 0x00,
    HpUx = 0x01,
    NetBsd = 0x02,
    Linux = 0x03,
    GnuHurd = 0x04,
    Solaris = 0x06,
    Aix = 0x07,
    Irix = 0x08,
    FreeBsd = 0x09,
    Tru64 = 0x0A,
    NovellModesto = 0x0B,
    OpenBsd = 0x0C,
    OpenVms = 0x0D,
    NonStopKernel = 0x0E,
    Aros = 0x0F,
    FenixOs = 0x10,
    NuxiCloudAbi = 0x11,
    StratusTechnologiesOpenVos = 0x12,
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u16)]
enum Et {
    None = 0,
    Rel = 1,
    Exec = 2,
    Dyn = 3,
    Core = 4,
    LoOs = 0xfe00,
    HiOs = 0xfeff,
    LoProc = 0xff00,
    HiProc = 0xffff,
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u16)]
enum Em {
    NoSpecificInstructionSet = 0x00,
    AtAndT = 0x01,
    Sparc = 0x02,
    X86 = 0x03,
    Motorola68000 = 0x04,
    Motorola88000 = 0x05,
    IntelMcu = 0x06,
    Intel80860 = 0x07,
    Mips = 0x08,
    IbmSystem370 = 0x09,
    MipsRS3000 = 0x0A,
    HewlettPackard = 0x0F,
    Intel80960 = 0x13,
    PowerPc = 0x14,
    PowerPc64 = 0x15,
    S390 = 0x16,
    IbmSpuSpc = 0x17,
    NecV800 = 0x24,
    FujitsuFR20 = 0x25,
    TrwRh32 = 0x26,
    MotorolaRCE = 0x27,
    Arm32 = 0x28,
    DigitalAlpha = 0x29,
    SuperH = 0x2A,
    SparcVersion9 = 0x2B,
    SiemensTriCoreEmbeddedProcessor = 0x2C,
    ArgonautRiscCore = 0x2D,
    HitachiH8300 = 0x2E,
    HitachiH8300H = 0x2F,
    HitachiH8S = 0x30,
    HitachiH8500 = 0x31,
    IA64 = 0x32,
    StanfordMipsX = 0x33,
    MotorolaColdFire = 0x34,
    MotorolaM68HC12 = 0x35,
    FujitsuMmaMultimediaAccelerator = 0x36,
    SiemensPcp = 0x37,
    SonyNcpuEmbeddedRiscProcessor = 0x38,
    DensoNdr1Microprocessor = 0x39,
    MotorolaStarCoreProcessor = 0x3A,
    ToyotaME16Processor = 0x3B,
    StMicroelectronicsST100Processor = 0x3C,
    AdvancedLogicCorpTinyJEmbeddedProcessorFamily = 0x3D,
    AmdX64 = 0x3E,
    SonyDspProcessor = 0x3F,
    DigitalEquipmentCorpPdp10 = 0x40,
    DigitalEquipmentCorpPdp11 = 0x41,
    SiemensFX66Microcontroller = 0x42,
    STMicroelectronicsST9_8_16bitMicrocontroller = 0x43,
    STMicroelectronicsST7_8bitMicrocontroller = 0x44,
    MotorolaMC68HC16Microcontroller = 0x45,
    MotorolaMC68HC11Microcontroller = 0x46,
    MotorolaMC68HC08Microcontroller = 0x47,
    MotorolaMC68HC05Microcontroller = 0x48,
    SiliconGraphicsSVx = 0x49,
    STMicroelectronicsST19_8bitMicrocontroller = 0x4A,
    DigitalVax = 0x4B,
    AxisCommunications32bitEmbeddedProcessor = 0x4C,
    InfineonTechnologies32bitEmbeddedProcessor = 0x4D,
    Element14_64bitDspProcessor = 0x4E,
    LsiLogic16bitDspProcessor = 0x4F,
    TMS320C6000Family = 0x8C,
    McstElbrusE2k = 0xAF,
    Arm64 = 0xB7,
    ZilogZ80 = 0xDC,
    RiscV = 0xF3,
    BerkeleyPacketFilter = 0xF7,
    WDC65C816 = 0x101,
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u32)]
enum Ev {
    None = 0,
    Current = 1,
}

