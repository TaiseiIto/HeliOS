pub mod boot;

use crate::{
    com2_print,
    com2_println,
    efi,
};

#[derive(Debug)]
pub struct Controller {
    information: efi::mp_services::ProcessorInformation,
}

impl Controller {
    pub fn boot(&self) {
        com2_println!("Boot processor {:#x?}", self.identifier());
    }

    pub fn identifier(&self) -> u64 {
        self.information.identifier()
    }

    pub fn new(information: efi::mp_services::ProcessorInformation) -> Self {
        Self {
            information,
        }
    }
}

