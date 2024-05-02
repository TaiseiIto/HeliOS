#[derive(Clone, Debug)]
pub enum Content {
    Char(char),
}

impl Content {
    pub fn char(character: char) -> Self {
        Self::Char(character)
    }
}

