use {
    super::descriptor,
    crate::{memory, x64},
    bitfield_struct::bitfield,
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

    pub fn base(&self) -> usize {
        let base0: usize = self.base0() as usize;
        let base1: usize = self.base1() as usize;
        base0 + (base1 << Self::BASE0_BITS)
    }

    pub fn get_avl(&self) -> bool {
        self.avl()
    }

    pub fn get_dpl(&self) -> u8 {
        self.dpl()
    }

    pub fn get_segment_type(&self) -> x64::descriptor::Type {
        let segment_type: u8 = self.segment_type();
        let s: bool = self.s();
        let db: bool = self.db();
        let l: bool = self.l();
        x64::descriptor::Type::new(segment_type, s, db, l)
    }

    pub fn is_long(&self) -> bool {
        let interface: Option<descriptor::Interface> = self.into();
        interface.map_or(false, |interface| interface.is_long_descriptor())
    }

    pub fn is_short(&self) -> bool {
        let interface: Option<descriptor::Interface> = self.into();
        interface.map_or(true, |interface| interface.is_short_descriptor())
    }

    pub fn present(&self) -> bool {
        self.p()
    }

    pub fn size(&self) -> usize {
        let limit0: usize = self.limit0() as usize;
        let limit1: usize = self.limit1() as usize;
        let limit: usize = limit0 + (limit1 << Self::LIMIT0_BITS);
        let size: usize = limit + 1;
        size * if self.g() { 4 * memory::KIB } else { 1 }
    }
}

impl From<&descriptor::Interface> for Descriptor {
    fn from(interface: &descriptor::Interface) -> Self {
        let base: usize = interface.base();
        let size: usize = interface.size();
        let dpl: u8 = interface.dpl();
        let avl: bool = interface.avl();
        let segment_type: &x64::descriptor::Type = interface.segment_type();
        let g: bool = Self::GRANULE_THRESHOLD <= size;
        let size = if g {
            (size + Self::GRANULE_THRESHOLD - 1) / Self::GRANULE_THRESHOLD
        } else {
            size
        };
        let limit: usize = size - 1;
        let limit0: u16 = (limit & ((1 << Self::LIMIT0_BITS) - 1)) as u16;
        let base0: u32 = (base & ((1 << Self::BASE0_BITS) - 1)) as u32;
        let s: bool = segment_type.s();
        let dpl: u8 = dpl;
        let p: bool = true;
        let limit1: u8 = ((limit >> Self::LIMIT1_OFFSET) & ((1 << Self::LIMIT1_BITS) - 1)) as u8;
        let avl: bool = avl;
        let l: bool = segment_type.l();
        let db: bool = segment_type.db();
        let base1: u8 = ((base >> Self::BASE0_BITS) & ((1 << Self::BASE1_BITS) - 1)) as u8;
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

impl From<&x64::task::state::segment::AndIoPermissionBitMap> for Descriptor {
    fn from(
        segment_and_io_permission_bit_map: &x64::task::state::segment::AndIoPermissionBitMap,
    ) -> Self {
        let interface: descriptor::Interface = segment_and_io_permission_bit_map.into();
        (&interface).into()
    }
}
