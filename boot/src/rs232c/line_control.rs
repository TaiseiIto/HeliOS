use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct Register {
    #[bits(2)]
    word_length: usize,
    length_of_stop_bit: bool,
    #[bits(3)]
    parity_select: usize,
    set_break_enable: bool,
    divisor_latch_access_bit: bool,
}

impl Register {
    pub fn disable_divisor_latch_access_bit(self) -> Self {
        self.with_divisor_latch_access_bit(false)
    }

    pub fn enable_divisor_latch_access_bit(self) -> Self {
        self.with_divisor_latch_access_bit(true)
    }

    pub fn read_divisor_latch_access_bit(&self) -> bool {
        self.divisor_latch_access_bit()
    }
}

