#[derive(Clone, Debug)]
pub enum Content {
    BootCompleted,
    Char(char),
}

impl Content {
    pub fn boot_completed() -> Self {
        Self::BootCompleted
    }

    pub fn char(character: char) -> Self {
        Self::Char(character)
    }
}

