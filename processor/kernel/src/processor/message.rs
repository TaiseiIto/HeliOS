use crate::interrupt;

#[derive(Clone, Debug)]
pub enum Content {
    BootCompleted,
    Char(char),
    #[allow(dead_code)]
    HpetInterrupt,
    Initialized,
    #[allow(dead_code)]
    PitInterrupt,
    #[allow(dead_code)]
    RtcInterrupt,
}

impl Content {
    pub fn boot_completed() -> Self {
        Self::BootCompleted
    }

    pub fn char(character: char) -> Self {
        Self::Char(character)
    }

    pub fn initialized() -> Self {
        Self::Initialized
    }

    pub fn process(self, _sender_local_apic_id: u8) {
        match self {
            Self::BootCompleted => unimplemented!(),
            Self::Char(_character) => unimplemented!(),
            Self::HpetInterrupt => interrupt::Event::push(interrupt::Event::Hpet),
            Self::Initialized => unimplemented!(),
            Self::PitInterrupt => interrupt::Event::push(interrupt::Event::Pit),
            Self::RtcInterrupt => interrupt::Event::push(interrupt::Event::Rtc),
        }
    }
}

