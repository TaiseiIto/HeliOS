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
    pub fn boot(&self, boot_loader: &mut boot::Loader, local_apic_registers: &mut interrupt::apic::local::Registers, hpet: &timer::hpet::Registers) {
        boot_loader.initialize();
        let local_apic_id: u8 = self.local_apic_id() as u8;
        com2_println!("Boot processor {:#x?}", local_apic_id);
        let entry_point: usize = boot_loader.entry_point();
        com2_println!("entry_point = {:#x?}", entry_point);
        local_apic_registers.send_init(local_apic_id, hpet);
        local_apic_registers.send_sipi(local_apic_id, entry_point, hpet);
        local_apic_registers.send_sipi(local_apic_id, entry_point, hpet);
        hpet.wait_seconds(1);
        com2_println!("{}", boot_loader.log());
    }

    pub fn local_apic_id(&self) -> u8 {
        self.local_apic_structure.apic_id()
    }

    pub fn new(local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure) -> Self {
        Self {
            local_apic_structure,
        }
    }
}

