pub mod boot;

use crate::{
    com2_print,
    com2_println,
    efi,
    interrupt,
    timer,
};

#[derive(Debug)]
pub struct Controller {
    information: efi::mp_services::ProcessorInformation,
}

impl Controller {
    pub fn boot(&self, boot_loader: &boot::Loader, local_apic_registers: &mut interrupt::apic::local::Registers, hpet: &timer::hpet::Registers) {
        let apic_id: u8 = self.apic_id() as u8;
        com2_println!("Boot processor {:#x?}", apic_id);
        local_apic_registers.clear_all_errors();
        local_apic_registers.send_init(apic_id);
    }

    pub fn apic_id(&self) -> u64 {
        self.information.identifier()
    }

    pub fn new(information: efi::mp_services::ProcessorInformation) -> Self {
        Self {
            information,
        }
    }
}

