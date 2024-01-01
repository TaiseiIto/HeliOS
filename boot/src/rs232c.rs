use crate::asm;

pub struct Com {
    port: u16,
}

impl Com {
    pub fn new(port: u16) -> Self {
        Self {
            port,
        }
    }

}

