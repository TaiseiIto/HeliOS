pub mod table;

pub use table::Table;

use {
    crate::x64::descriptor::Type,
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
    #[bits(4)]
    segment_type: u8,
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
pub struct Interface {
    #[allow(dead_code)]
    base: usize,
    #[allow(dead_code)]
    size: usize,
    #[allow(dead_code)]
    dpl: u8,
    #[allow(dead_code)]
    avl: bool,
    #[allow(dead_code)]
    segment_type: Type,
}

impl From<&Descriptor> for Option<Interface> {
    fn from(descriptor: &Descriptor) -> Self {
        if descriptor.p() {
            let base0: usize = descriptor.base0() as usize;
            let base1: usize = descriptor.base1() as usize;
            let base: usize = base0 + (base1 << Descriptor::BASE0_BITS);
            let limit0: usize = descriptor.limit0() as usize;
            let limit1: usize = descriptor.limit1() as usize;
            let limit: usize = limit0 + (limit1 << Descriptor::LIMIT0_BITS);
            let size: usize = limit + 1;
            let size: usize = if descriptor.g() {
                4 * KIB * size
            } else {
                size
            };
            let dpl: u8 = descriptor.dpl();
            let avl: bool = descriptor.avl();
            let segment_type: u8 = descriptor.segment_type();
            let s: bool = descriptor.s();
            let db: bool = descriptor.db();
            let l: bool = descriptor.l();
            let segment_type = Type::new(segment_type, s, db, l);
            Some(Interface {
                base,
                size,
                dpl,
                avl,
                segment_type,
            })
        } else {
            None
        }
    }
}

