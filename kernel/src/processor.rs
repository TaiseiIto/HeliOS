pub mod boot;

use crate::{
    acpi,
    com2_print,
    com2_println,
    interrupt,
    timer,
};

#[derive(Debug)]
pub struct Controller {
    local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure,
}

impl Controller {
    pub fn boot(&self, boot_loader: &boot::Loader, local_apic_registers: &mut interrupt::apic::local::Registers, hpet: &timer::hpet::Registers) {
        let apic_id: u8 = self.apic_id() as u8;
        com2_println!("Boot processor {:#x?}", apic_id);
        local_apic_registers.send_init(apic_id);
        hpet.wait_milliseconds(10);
    }

    pub fn apic_id(&self) -> u8 {
        self.local_apic_structure.apic_id()
    }

    pub fn new(local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure) -> Self {
        Self {
            local_apic_structure,
        }
    }
}

