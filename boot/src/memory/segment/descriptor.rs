pub mod table;

pub use table::Table;

use {
    bitfield_struct::bitfield,
    super::super::KIB,
};

/// # Segment Descriptor
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 3.4.5 Segment Descriptors, Figure 3-8. Segment Descriptor
#[bitfield(u64)]
pub struct Descriptor {
    limit0: u16,
    #[bits(24)]
    base0: u32,
    type0: bool,
    type1: bool,
    type2: bool,
    type3: bool,
    s: bool,
    #[bits(2)]
    dp: u8,
    p: bool,
    #[bits(4)]
    limit1: u8,
    avl: bool,
    l: bool,
    db: bool,
    g: bool,
    base1: u8,
}

impl Descriptor {
    pub fn present(&self) -> bool {
        self.p()
    }
}

#[derive(Debug)]
pub struct Readable {
    base: u32,
    size: u32,
    accessed: bool,
    conforming: bool,
    executable: bool,
    expand_down: bool,
    readable: bool,
    writable: bool,
}

impl From<&Descriptor> for Readable {
    fn from(descriptor: &Descriptor) -> Self {
        let base0: u32 = descriptor.base0();
        let base1: u32 = descriptor.base1() as u32;
        let base: u32 = base0 + (base1 << Descriptor::BASE0_BITS);
        let limit0: u32 = descriptor.limit0() as u32;
        let limit1: u32 = descriptor.limit1() as u32;
        let limit: u32 = limit0 + (limit1 << Descriptor::LIMIT1_BITS);
        let size: u32 = limit + 1;
        let size: u32 = if descriptor.g() {
            (4 * KIB as u32) * size
        } else {
            size
        };
        let type0: bool = descriptor.type0();
        let type1: bool = descriptor.type1();
        let type2: bool = descriptor.type2();
        let type3: bool = descriptor.type3();
        let executable: bool = type3;
        let accessed: bool = type0;
        let conforming: bool = executable && type2;
        let expand_down: bool = !executable && type2;
        let readable: bool = !executable || type1;
        let writable: bool = !executable && type1;
        Self {
            base,
            size,
            accessed,
            conforming,
            executable,
            expand_down,
            readable,
            writable,
        }
    }
}

