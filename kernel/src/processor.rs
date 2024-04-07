pub mod boot;

use crate::{
    com2_print,
    com2_println,
    efi,
    interrupt,
};

#[derive(Debug)]
pub struct Controller {
    information: efi::mp_services::ProcessorInformation,
}

impl Controller {
    pub fn boot(&self, boot_loader: &boot::Loader, local_apic_registers: &mut interrupt::apic::local::Registers, efi_system_table: &efi::SystemTable) {
        com2_println!("Boot processor {:#x?}", self.identifier());
        local_apic_registers.clear_all_errors();
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

