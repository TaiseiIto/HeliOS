use bitfield_struct::bitfield;

// https://www.lookrs232.com/rs232/lcr.htm
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
        assert!(5 <= word_length && word_length <= 8);
        let length_of_stop: bool = length_of_stop.into(word_length);
        let word_length: u8 = word_length - 5;
        let parity_select: u8 = parity_select.into();
        Self::new()
            .with_word_length(word_length)
            .with_length_of_stop(length_of_stop)
            .with_parity_select(parity_select)
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
                assert!(6 <= word_length && word_length <= 8);
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

impl Into<u8> for ParitySelect {
    fn into(self) -> u8 {
        match self {
            Self::No => 0b000,
            Self::Odd => 0b001,
            Self::Even => 0b011,
            Self::High => 0b101,
            Self::Low => 0b111,
        }
    }
}

