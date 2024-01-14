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
    dpl: u8,
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
    dpl: u8,
    avl: bool,
}

impl From<&Descriptor> for Option<Readable> {
    fn from(descriptor: &Descriptor) -> Self {
        if descriptor.p() {
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
            let dpl: u8 = descriptor.dpl();
            let avl: bool = descriptor.avl();
            Some(Readable {
                base,
                size,
                dpl,
                avl,
            })
        } else {
            None
        }
    }
}

