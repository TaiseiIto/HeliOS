use super::Controller;

#[derive(Debug)]
pub enum Content {
    Char(char),
}

impl Content {
    pub fn process(self, controller: &mut Controller) {
        match self {
            Self::Char(character) => controller.receive_character(character),
        }
    }
}

