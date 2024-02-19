pub mod table;

pub use table::Table;

use {
    bitfield_struct::bitfield,
    crate::{
        memory,
        x64::descriptor::Type,
    },
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
    const GRANULE_THRESHOLD: usize = 1 << (Self::LIMIT0_BITS + Self::LIMIT1_BITS);

    pub fn present(&self) -> bool {
        self.p()
    }
}

impl From<&Interface> for Descriptor {
    fn from(interface: &Interface) -> Self {
        let Interface {
            base,
            size,
            dpl,
            avl,
            segment_type,
        } = interface;
        let g: bool = Self::GRANULE_THRESHOLD <= *size;
        let size = if g {
            (size + Self::GRANULE_THRESHOLD - 1) / Self::GRANULE_THRESHOLD
        } else {
            *size
        };
        let limit: usize = size - 1;
        let limit0: u16 = (limit & ((1 << Self::LIMIT0_BITS) - 1)) as u16;
        let base0: u32 = (base & ((1 << Self::BASE0_BITS) - 1)) as u32;
        let s: bool = segment_type.s();
        let dpl: u8 = *dpl;
        let p: bool = true;
        let limit1: u8 = ((limit >> Self::LIMIT1_OFFSET) & ((1 << Self::LIMIT1_BITS) - 1)) as u8;
        let avl: bool = *avl;
        let l: bool = segment_type.l();
        let db: bool = segment_type.db();
        let base1: u8 = ((base >> Self::BASE1_OFFSET) & ((1 << Self::BASE1_BITS) - 1)) as u8;
        let segment_type: u8 = segment_type.segment_type();
        Self::default()
            .with_limit0(limit0)
            .with_base0(base0)
            .with_segment_type(segment_type)
            .with_s(s)
            .with_dpl(dpl)
            .with_p(p)
            .with_limit1(limit1)
            .with_avl(avl)
            .with_l(l)
            .with_db(db)
            .with_g(g)
            .with_base1(base1)
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
        descriptor.p().then(|| {
            let base0: usize = descriptor.base0() as usize;
            let base1: usize = descriptor.base1() as usize;
            let base: usize = base0 + (base1 << Descriptor::BASE0_BITS);
            let limit0: usize = descriptor.limit0() as usize;
            let limit1: usize = descriptor.limit1() as usize;
            let limit: usize = limit0 + (limit1 << Descriptor::LIMIT0_BITS);
            let size: usize = (limit + 1) * if descriptor.g() {
                memory::page::SIZE
            } else {
                1
            };
            let dpl: u8 = descriptor.dpl();
            let avl: bool = descriptor.avl();
            let segment_type: u8 = descriptor.segment_type();
            let s: bool = descriptor.s();
            let db: bool = descriptor.db();
            let l: bool = descriptor.l();
            let segment_type = Type::new(segment_type, s, db, l);
            Interface {
                base,
                size,
                dpl,
                avl,
                segment_type,
            }
        })
    }
}

