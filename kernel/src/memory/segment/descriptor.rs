pub mod table;

pub use table::Table;

use {
    bitfield_struct::bitfield,
    core::mem,
    crate::{
        memory,
        x64,
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

    pub fn is_long(&self) -> bool {
        let interface: Option<Interface> = self.into();
        interface.map_or(false, |interface| interface.is_long_descriptor())
    }

    pub fn is_short(&self) -> bool {
        let interface: Option<Interface> = self.into();
        interface.map_or(true, |interface| interface.is_short_descriptor())
    }

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

impl From<&x64::task::state::segment::AndIoPermissionBitMap> for Descriptor {
    fn from(segment_and_io_permission_bit_map: &x64::task::state::segment::AndIoPermissionBitMap) -> Self {
        let interface: Interface = segment_and_io_permission_bit_map.into();
        (&interface).into()
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
    segment_type: x64::descriptor::Type,
}

impl Interface {
    pub fn base_address(&self) -> usize {
        self.base
    }

    pub fn is_long_descriptor(&self) -> bool {
        self.segment_type.is_long_descriptor()
    }

    pub fn is_short_descriptor(&self) -> bool {
        self.segment_type.is_short_descriptor()
    }
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
            let segment_type = x64::descriptor::Type::new(segment_type, s, db, l);
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

impl From<&x64::task::state::segment::AndIoPermissionBitMap> for Interface {
    fn from(segment_and_io_permission_bit_map: &x64::task::state::segment::AndIoPermissionBitMap) -> Self {
        let base: *const x64::task::state::segment::AndIoPermissionBitMap = segment_and_io_permission_bit_map as *const x64::task::state::segment::AndIoPermissionBitMap;
        let base: usize = base as usize;
        let size: usize = mem::size_of::<x64::task::state::segment::AndIoPermissionBitMap>();
        let dpl: u8 = 0;
        let avl: bool = false;
        let segment_type = x64::descriptor::Type::available_tss();
        Self {
            base,
            size,
            dpl,
            avl,
            segment_type,
        }
    }
}

impl From<&x64::task::state::segment::Descriptor> for Option<Interface> {
    fn from(descriptor: &x64::task::state::segment::Descriptor) -> Self {
        let lower_descriptor: Self = (&descriptor.lower_descriptor()).into();
        let base: Option<usize> = descriptor.base_address();
        lower_descriptor
            .zip(base)
            .map(|(Interface {
                base: _,
                size,
                dpl,
                avl,
                segment_type,
            }, base)| Interface {
                base,
                size,
                dpl,
                avl,
                segment_type,
            })
    }
}

