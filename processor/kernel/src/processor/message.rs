use crate::interrupt;

#[derive(Clone, Debug)]
pub enum Content {
    BootCompleted,
    Char(char),
    HpetInterrupt,
    Initialized,
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

    pub fn process(self) {
        match self {
            Self::BootCompleted => unimplemented!(),
            Self::Char(character) => unimplemented!(),
            Self::HpetInterrupt => interrupt::Event::push(interrupt::Event::Hpet),
            Self::Initialized => unimplemented!(),
        }
    }
}

