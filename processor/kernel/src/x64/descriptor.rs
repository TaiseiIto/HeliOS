use alloc::vec::Vec;

/// # Descriptor Type
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 3.4.5.1 Code- and Data-Segment Descriptor Types, Table 3-1. Code- and Data-Segment Types
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 3.5 System Descriptor Types, Table 3-2. System-Segment and Gate-Descriptor Types
#[derive(Clone, Debug)]
pub enum Type {
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
    const BITS: usize = 4;

    pub fn available_tss() -> Self {
        Self::AvailableTss
    }

    pub fn db(&self) -> bool {
        match self {
            Self::Code {
                accessed: _,
                readable: _,
                conforming: _,
                default_bits,
            } => match default_bits {
                16 | 64 => false,
                32 => true,
                _ => panic!("Can't get db flag."),
            },
            Self::Data {
                accessed: _,
                writable: _,
                expand_down: _,
                default_bits,
            } => match default_bits {
                32 => true,
                16 => false,
                _ => panic!("Can't get db flag."),
            },
            Self::Ldt | Self::AvailableTss | Self::BusyTss | Self::CallGate | Self::InterruptGate | Self::TrapGate => false,
        }
    }

    pub fn interrupt_gate() -> Self {
        Self::InterruptGate
    }

    pub fn is_long_descriptor(&self) -> bool {
        match self {
            Self::Code {
                accessed: _,
                readable: _,
                conforming: _,
                default_bits: _,
            } => false,
            Self::Data {
                accessed: _,
                writable: _,
                expand_down: _,
                default_bits: _,
            } => false,
            Self::Ldt | Self::AvailableTss | Self::BusyTss => true,
            Self::CallGate | Self::InterruptGate | Self::TrapGate => false,
        }
    }

    pub fn is_short_descriptor(&self) -> bool {
        match self {
            Self::Code {
                accessed: _,
                readable: _,
                conforming: _,
                default_bits: _,
            } => true,
            Self::Data {
                accessed: _,
                writable: _,
                expand_down: _,
                default_bits: _,
            } => true,
            Self::Ldt | Self::AvailableTss | Self::BusyTss => false,
            Self::CallGate | Self::InterruptGate | Self::TrapGate => true,
        }
    }

    pub fn l(&self) -> bool {
        match self {
            Self::Code {
                accessed: _,
                readable: _,
                conforming: _,
                default_bits,
            } => match default_bits {
                16 | 32 => false,
                64 => true,
                _ => panic!("Can't get l flag."),
            },
            Self::Data {
                accessed: _,
                writable: _,
                expand_down: _,
                default_bits: _,
            } => false,
            Self::Ldt | Self::AvailableTss | Self::BusyTss | Self::CallGate | Self::InterruptGate | Self::TrapGate => false,
        }
    }

    pub fn new(segment_type: u8, s: bool, db: bool, l: bool) -> Self {
        if s {
            let segment_type: Vec<bool> = (0..Self::BITS)
                .map(|offset| segment_type & (1 << offset) != 0)
                .collect();
            let segment_type: [bool; Self::BITS] = segment_type
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

    pub fn s(&self) -> bool {
        match self {
            Self::Code {
                accessed: _,
                readable: _,
                conforming: _,
                default_bits: _,
            } => true,
            Self::Data {
                accessed: _,
                writable: _,
                expand_down: _,
                default_bits: _,
            } => true,
            Self::Ldt | Self::AvailableTss | Self::BusyTss | Self::CallGate | Self::InterruptGate | Self::TrapGate => false,
        }
    }

    pub fn segment_type(&self) -> u8 {
        match self {
            Self::Code {
                accessed,
                readable,
                conforming,
                default_bits: _,
            } => (if *accessed {
                1 << 0
            } else {
                0
            }) + (if *readable {
                1 << 1
            } else {
                0
            }) + (if *conforming {
                1 << 2
            } else {
                0
            }) + (1 << 3),
            Self::Data {
                accessed: _,
                writable,
                expand_down,
                default_bits: _,
            } => (if *writable {
                 1 << 1
            } else {
                0
            }) + (if *expand_down {
                1 << 2
            } else {
                0
            }),
            Self::Ldt => 2,
            Self::AvailableTss => 9,
            Self::BusyTss => 11,
            Self::CallGate => 12,
            Self::InterruptGate => 14,
            Self::TrapGate => 15,
        }
    }
}

