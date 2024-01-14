pub mod table;

pub use table::Table;

use {
    alloc::vec::Vec,
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
pub struct Readable {
    base: u32,
    size: u32,
    dpl: u8,
    avl: bool,
    segment_type: Type,
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
            let segment_type: u8 = descriptor.segment_type();
            let s: bool = descriptor.s();
            let db: bool = descriptor.db();
            let l: bool = descriptor.l();
            let segment_type = Type::new(segment_type, s, db, l);
            Some(Readable {
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

#[derive(Debug)]
enum Type {
    Code {
        accessed: bool,
        readable: bool,
        conforming: bool,
        default_bits: usize,
    },
    Data {
        accessed: bool,
        writable: bool,
        expand_down: bool,
        default_bits: usize,
    },
    Ldt,
    AvailableTss,
    BusyTss,
    CallGate,
    InterruptGate,
    TrapGate,
}

impl Type {
    fn new(segment_type: u8, s: bool, db: bool, l: bool) -> Self {
        if s {
            let segment_type: Vec<bool> = (0..Descriptor::SEGMENT_TYPE_BITS)
                .map(|offset| segment_type & (1 << offset) != 0)
                .collect();
            let segment_type: [bool; Descriptor::SEGMENT_TYPE_BITS] = segment_type
                .try_into()
                .unwrap();
            let accessed: bool = segment_type[0];
            if segment_type[3] {
                let readable: bool = segment_type[1];
                let conforming: bool = segment_type[2];
                let default_bits: usize = match (db, l) {
                    (false, false) => 16,
                    (false, true) => 64,
                    (true, false) => 32,
                    (true, true) => panic!("Invalid code segment."),
                };
                Self::Code {
                    accessed,
                    readable,
                    conforming,
                    default_bits,
                }
            } else {
                let writable: bool = segment_type[1];
                let expand_down: bool = segment_type[2];
                let default_bits: usize = if db {
                    32
                } else {
                    16
                };
                Self::Data {
                    accessed,
                    writable,
                    expand_down,
                    default_bits,
                }
            }
        } else {
            match segment_type {
                2 => Self::Ldt,
                9 => Self::AvailableTss,
                11 => Self::BusyTss,
                12 => Self::CallGate,
                14 => Self::InterruptGate,
                15 => Self::TrapGate,
                segment_type => panic!("Invalid segment type {}.", segment_type),
            }
        }
    }
}

