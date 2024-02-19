/// Task State Segment
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 8.7 Figure 8-11. 64-Bit TSS Format
#[allow(dead_code)]
#[derive(Debug)]
#[repr(packed)]
pub struct Segment {
    reserved0: u32,
    rsp: [u64; Self::NUMBER_OF_STACK_POINTERS],
    ist: [u64; Self::NUMBER_OF_INTERRUPT_STACKS + 1], // ist[0] is reserved.
    reserved1: u64,
    reserved2: u16,
    io_map_base_address: u16,
}

impl Segment {
    pub const NUMBER_OF_STACK_POINTERS: usize = 3;
    pub const NUMBER_OF_INTERRUPT_STACKS: usize = 7;

    #[allow(dead_code)]
    pub fn new(rsp: [u64; 3], ist: [u64; 8], io_map_base_address: u16) -> Self {
        let reserved0: u32 = 0;
        let reserved1: u64 = 0;
        let reserved2: u16 = 0;
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

