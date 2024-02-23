use {
    core::mem,
    crate::{
        memory,
        x64,
    },
    super::Descriptor,
};

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
    pub fn avl(&self) -> bool {
        self.avl
    }

    pub fn base(&self) -> usize {
        self.base
    }

    pub fn dpl(&self) -> u8 {
        self.dpl
    }

    pub fn is_long_descriptor(&self) -> bool {
        self.segment_type.is_long_descriptor()
    }

    pub fn is_short_descriptor(&self) -> bool {
        self.segment_type.is_short_descriptor()
    }

    pub fn segment_type(&self) -> &x64::descriptor::Type {
        &self.segment_type
    }

    pub fn size(&self) -> usize {
        self.size
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

