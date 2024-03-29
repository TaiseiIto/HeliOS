//! # RS232C line control register
//! ## References
//! * [Look RS232 Line Control Register](https://www.lookrs232.com/rs232/lcr.htm)

use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct Register {
    #[bits(2)]
    word_length: u8,
    length_of_stop: bool,
    #[bits(3)]
    parity_select: u8,
    set_break_enable: bool,
    divisor_latch_access: bool,
}

impl Register {
    pub fn disable_divisor_latch_access(self) -> Self {
        self.with_divisor_latch_access(false)
    }

    pub fn enable_divisor_latch_access(self) -> Self {
        self.with_divisor_latch_access(true)
    }

    pub fn create(
        word_length: u8,
        length_of_stop: LengthOfStop,
        parity_select: ParitySelect,
        set_break_enable: bool,
        divisor_latch_access: bool,
    ) -> Self {
        assert!((5..=8).contains(&word_length));
        Self::new()
            .with_word_length(word_length - 5)
            .with_length_of_stop(length_of_stop.into(word_length))
            .with_parity_select(parity_select.into())
            .with_set_break_enable(set_break_enable)
            .with_divisor_latch_access(divisor_latch_access)
    }

    pub fn read_divisor_latch_access(&self) -> bool {
        self.divisor_latch_access()
    }
}

pub enum LengthOfStop {
    One,
    OnePointFive,
    Two,
}

impl LengthOfStop {
    fn into(self, word_length: u8) -> bool {
        match self {
            Self::One => false,
            Self::OnePointFive => {
                assert!(word_length == 5);
                true
            },
            Self::Two => {
                assert!((6..=8).contains(&word_length));
                true
            },
        }
    }
}

pub enum ParitySelect {
    No,
    Odd,
    Even,
    High,
    Low,
}

impl From<ParitySelect> for u8 {
    fn from(parity_select: ParitySelect) -> Self {
        match parity_select {
            ParitySelect::No => 0b000,
            ParitySelect::Odd => 0b001,
            ParitySelect::Even => 0b011,
            ParitySelect::High => 0b101,
            ParitySelect::Low => 0b111,
        }
    }
}

