//! # ELF Header
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

use {
    super::{program, section, Addr, Half, Off, UnsignedChar, Word},
    alloc::vec::Vec,
    core::arch::asm,
};

/// # ELF Header
/// ## References
/// * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
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

impl Header {
    pub fn entry(&self) -> usize {
        self.e_entry as usize
    }

    pub fn program_headers(&self) -> Vec<&program::Header> {
        let header: *const Header = self as *const Header;
        let header: *const u8 = header as *const u8;
        let program_header: *const u8 = unsafe { header.add(self.e_phoff as usize) };
        (0..self.e_phnum)
            .map(move |index| {
                let program_header: *const u8 =
                    unsafe { program_header.add((index * self.e_phentsize) as usize) };
                let program_header: *const program::Header =
                    program_header as *const program::Header;
                unsafe { &*program_header }
            })
            .collect()
    }

    #[allow(dead_code)]
    #[inline(never)]
    pub fn run<T>(&self, stack_floor: usize, argument: &T) {
        let entry: usize = self.entry();
        let argument: *const T = argument as *const T;
        let argument: usize = argument as usize;
        unsafe {
            asm!(
                "mov rsp, {0}",
                "call {1}",
                in(reg) stack_floor,
                in(reg) entry,
                in("rdi") argument,
            );
        }
    }

    pub fn section_headers(&self) -> Vec<&section::Header> {
        let header: *const Header = self as *const Header;
        let header: *const u8 = header as *const u8;
        let section_header: *const u8 = unsafe { header.add(self.e_shoff as usize) };
        (0..self.e_shnum)
            .map(move |index| {
                let section_header: *const u8 =
                    unsafe { section_header.add((index * self.e_shentsize) as usize) };
                let section_header: *const section::Header =
                    section_header as *const section::Header;
                unsafe { &*section_header }
            })
            .collect()
    }
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

#[derive(Debug)]
#[repr(u8)]
enum Class {
    #[allow(dead_code)]
    Invalid = 0,
    #[allow(dead_code)]
    Bit32 = 1,
    #[allow(dead_code)]
    Bit64 = 2,
}

#[derive(Debug)]
#[repr(u8)]
enum Data {
    #[allow(dead_code)]
    None = 0,
    #[allow(dead_code)]
    LittleEndian = 1,
    #[allow(dead_code)]
    BigEndian = 2,
}

#[derive(Debug)]
#[repr(u8)]
enum Osabi {
    #[allow(dead_code)]
    SystemV = 0x00,
    #[allow(dead_code)]
    HpUx = 0x01,
    #[allow(dead_code)]
    NetBsd = 0x02,
    #[allow(dead_code)]
    Linux = 0x03,
    #[allow(dead_code)]
    GnuHurd = 0x04,
    #[allow(dead_code)]
    Solaris = 0x06,
    #[allow(dead_code)]
    Aix = 0x07,
    #[allow(dead_code)]
    Irix = 0x08,
    #[allow(dead_code)]
    FreeBsd = 0x09,
    #[allow(dead_code)]
    Tru64 = 0x0A,
    #[allow(dead_code)]
    NovellModesto = 0x0B,
    #[allow(dead_code)]
    OpenBsd = 0x0C,
    #[allow(dead_code)]
    OpenVms = 0x0D,
    #[allow(dead_code)]
    NonStopKernel = 0x0E,
    #[allow(dead_code)]
    Aros = 0x0F,
    #[allow(dead_code)]
    FenixOs = 0x10,
    #[allow(dead_code)]
    NuxiCloudAbi = 0x11,
    #[allow(dead_code)]
    StratusTechnologiesOpenVos = 0x12,
}

#[derive(Debug)]
#[repr(u16)]
enum Et {
    #[allow(dead_code)]
    None = 0,
    #[allow(dead_code)]
    Rel = 1,
    #[allow(dead_code)]
    Exec = 2,
    #[allow(dead_code)]
    Dyn = 3,
    #[allow(dead_code)]
    Core = 4,
    #[allow(dead_code)]
    LoOs = 0xfe00,
    #[allow(dead_code)]
    HiOs = 0xfeff,
    #[allow(dead_code)]
    LoProc = 0xff00,
    #[allow(dead_code)]
    HiProc = 0xffff,
}

#[derive(Debug)]
#[repr(u16)]
enum Em {
    #[allow(dead_code)]
    NoSpecificInstructionSet = 0x00,
    #[allow(dead_code)]
    AtAndT = 0x01,
    #[allow(dead_code)]
    Sparc = 0x02,
    #[allow(dead_code)]
    X86 = 0x03,
    #[allow(dead_code)]
    Motorola68000 = 0x04,
    #[allow(dead_code)]
    Motorola88000 = 0x05,
    #[allow(dead_code)]
    IntelMcu = 0x06,
    #[allow(dead_code)]
    Intel80860 = 0x07,
    #[allow(dead_code)]
    Mips = 0x08,
    #[allow(dead_code)]
    IbmSystem370 = 0x09,
    #[allow(dead_code)]
    MipsRS3000 = 0x0A,
    #[allow(dead_code)]
    HewlettPackard = 0x0F,
    #[allow(dead_code)]
    Intel80960 = 0x13,
    #[allow(dead_code)]
    PowerPc = 0x14,
    #[allow(dead_code)]
    PowerPc64 = 0x15,
    #[allow(dead_code)]
    S390 = 0x16,
    #[allow(dead_code)]
    IbmSpuSpc = 0x17,
    #[allow(dead_code)]
    NecV800 = 0x24,
    #[allow(dead_code)]
    FujitsuFR20 = 0x25,
    #[allow(dead_code)]
    TrwRh32 = 0x26,
    #[allow(dead_code)]
    MotorolaRCE = 0x27,
    #[allow(dead_code)]
    Arm32 = 0x28,
    #[allow(dead_code)]
    DigitalAlpha = 0x29,
    #[allow(dead_code)]
    SuperH = 0x2A,
    #[allow(dead_code)]
    SparcVersion9 = 0x2B,
    #[allow(dead_code)]
    SiemensTriCoreEmbeddedProcessor = 0x2C,
    #[allow(dead_code)]
    ArgonautRiscCore = 0x2D,
    #[allow(dead_code)]
    HitachiH8300 = 0x2E,
    #[allow(dead_code)]
    HitachiH8300H = 0x2F,
    #[allow(dead_code)]
    HitachiH8S = 0x30,
    #[allow(dead_code)]
    HitachiH8500 = 0x31,
    #[allow(dead_code)]
    IA64 = 0x32,
    #[allow(dead_code)]
    StanfordMipsX = 0x33,
    #[allow(dead_code)]
    MotorolaColdFire = 0x34,
    #[allow(dead_code)]
    MotorolaM68HC12 = 0x35,
    #[allow(dead_code)]
    FujitsuMmaMultimediaAccelerator = 0x36,
    #[allow(dead_code)]
    SiemensPcp = 0x37,
    #[allow(dead_code)]
    SonyNcpuEmbeddedRiscProcessor = 0x38,
    #[allow(dead_code)]
    DensoNdr1Microprocessor = 0x39,
    #[allow(dead_code)]
    MotorolaStarCoreProcessor = 0x3A,
    #[allow(dead_code)]
    ToyotaME16Processor = 0x3B,
    #[allow(dead_code)]
    StMicroelectronicsST100Processor = 0x3C,
    #[allow(dead_code)]
    AdvancedLogicCorpTinyJEmbeddedProcessorFamily = 0x3D,
    #[allow(dead_code)]
    AmdX64 = 0x3E,
    #[allow(dead_code)]
    SonyDspProcessor = 0x3F,
    #[allow(dead_code)]
    DigitalEquipmentCorpPdp10 = 0x40,
    #[allow(dead_code)]
    DigitalEquipmentCorpPdp11 = 0x41,
    #[allow(dead_code)]
    SiemensFX66Microcontroller = 0x42,
    #[allow(dead_code)]
    STMicroelectronicsST9_8_16bitMicrocontroller = 0x43,
    #[allow(dead_code)]
    STMicroelectronicsST7_8bitMicrocontroller = 0x44,
    #[allow(dead_code)]
    MotorolaMC68HC16Microcontroller = 0x45,
    #[allow(dead_code)]
    MotorolaMC68HC11Microcontroller = 0x46,
    #[allow(dead_code)]
    MotorolaMC68HC08Microcontroller = 0x47,
    #[allow(dead_code)]
    MotorolaMC68HC05Microcontroller = 0x48,
    #[allow(dead_code)]
    SiliconGraphicsSVx = 0x49,
    #[allow(dead_code)]
    STMicroelectronicsST19_8bitMicrocontroller = 0x4A,
    #[allow(dead_code)]
    DigitalVax = 0x4B,
    #[allow(dead_code)]
    AxisCommunications32bitEmbeddedProcessor = 0x4C,
    #[allow(dead_code)]
    InfineonTechnologies32bitEmbeddedProcessor = 0x4D,
    #[allow(dead_code)]
    Element14_64bitDspProcessor = 0x4E,
    #[allow(dead_code)]
    LsiLogic16bitDspProcessor = 0x4F,
    #[allow(dead_code)]
    TMS320C6000Family = 0x8C,
    #[allow(dead_code)]
    McstElbrusE2k = 0xAF,
    #[allow(dead_code)]
    Arm64 = 0xB7,
    #[allow(dead_code)]
    ZilogZ80 = 0xDC,
    #[allow(dead_code)]
    RiscV = 0xF3,
    #[allow(dead_code)]
    BerkeleyPacketFilter = 0xF7,
    #[allow(dead_code)]
    WDC65C816 = 0x101,
}

#[derive(Debug)]
#[repr(u32)]
enum Ev {
    #[allow(dead_code)]
    None = 0,
    #[allow(dead_code)]
    Current = 1,
}
