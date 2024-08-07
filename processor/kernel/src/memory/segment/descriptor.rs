pub mod table;

pub use table::Table;

use {
    core::mem::size_of,
    crate::x64,
    super::{
        long,
        short,
    },
};

#[derive(Debug)]
pub struct Interface {
    base: usize,
    size: usize,
    dpl: u8,
    avl: bool,
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

    pub fn with_dpl(&self, dpl: u8) -> Self {
        let Self {
            base,
            size,
            dpl: _,
            avl,
            segment_type,
        } = self;
        let base: usize = *base;
        let size: usize = *size;
        let avl: bool = *avl;
        let segment_type: x64::descriptor::Type = segment_type.clone();
        Self {
            base,
            size,
            dpl,
            avl,
            segment_type
        }
    }
}

impl From<&short::Descriptor> for Option<Interface> {
    fn from(descriptor: &short::Descriptor) -> Self {
        descriptor.present().then(|| {
            let base: usize = descriptor.base();
            let size: usize = descriptor.size();
            let dpl: u8 = descriptor.get_dpl();
            let avl: bool = descriptor.get_avl();
            let segment_type: x64::descriptor::Type = descriptor.get_segment_type();
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
        let size: usize = size_of::<x64::task::state::segment::AndIoPermissionBitMap>();
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

impl From<&long::Descriptor> for Option<Interface> {
    fn from(descriptor: &long::Descriptor) -> Self {
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

