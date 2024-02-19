mod descriptor;

pub use descriptor::Descriptor;

use {
    alloc::vec::Vec,
    core::{
        iter,
        mem,
    },
    crate::memory,
};

#[derive(Debug)]
#[repr(packed)]
pub struct AndIoPermissionBitMap {
    segment: Segment,
    io_permission_bit_map: IoPermissionBitMap,
}

impl AndIoPermissionBitMap {
    pub fn new(interrupt_stacks: &Vec<memory::Stack>) -> Self {
        let segment = Segment::new(interrupt_stacks, mem::size_of::<Segment>());
        let io_permission_bit_map = IoPermissionBitMap::default();
        Self {
            segment,
            io_permission_bit_map,
        }
    }
}

/// Task State Segment
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 8.7 Figure 8-11. 64-Bit TSS Format
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Segment {
    reserved0: u32,
    rsp: [usize; Self::NUMBER_OF_STACK_POINTERS],
    ist: [usize; Self::NUMBER_OF_INTERRUPT_STACKS + 1], // ist[0] is reserved.
    reserved1: u64,
    reserved2: u16,
    io_map_base_address: u16,
}

impl Segment {
    pub const NUMBER_OF_STACK_POINTERS: usize = 3;
    pub const NUMBER_OF_INTERRUPT_STACKS: usize = 7;

    #[allow(dead_code)]
    pub fn new(interrupt_stacks: &Vec<memory::Stack>, io_map_base_address: usize) -> Self {
        let reserved0: u32 = 0;
        let reserved1: u64 = 0;
        let reserved2: u16 = 0;
        let rsp: Vec<usize> = iter::repeat(0)
            .take(Self::NUMBER_OF_STACK_POINTERS)
            .collect();
        let rsp: [usize; Self::NUMBER_OF_STACK_POINTERS] = rsp
            .as_slice()
            .try_into()
            .unwrap();
        let ist: Vec<usize> = iter::once(0)
            .chain(interrupt_stacks
                .iter()
                .map(|interrupt_stack| interrupt_stack.floor()))
            .collect();
        let ist: [usize; Self::NUMBER_OF_INTERRUPT_STACKS + 1] = ist
            .as_slice()
            .try_into()
            .unwrap();
        let io_map_base_address: u16 = io_map_base_address as u16;
        Self {
            reserved0,
            rsp,
            ist,
            reserved1,
            reserved2,
            io_map_base_address,
        }
    }
}

/// I/O Permission Bit Map
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.1A 19.5.2 I/O Permission Bit Map
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct IoPermissionBitMap {
    bit_map: [u8; Self::LENGTH],
    last_byte: u8,
}

impl IoPermissionBitMap {
    const PORTS: usize = (u16::MAX as usize) + 1;
    const LENGTH: usize = Self::PORTS / (u8::BITS as usize);
}

impl Default for IoPermissionBitMap {
    fn default() -> Self {
        let bit_map: [u8; Self::LENGTH] = [u8::MAX; Self::LENGTH];
        let last_byte: u8 = u8::MAX;
        Self {
            bit_map,
            last_byte,
        }
    }
}
