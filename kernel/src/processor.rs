pub mod boot;

use {
    core::ops::RangeInclusive,
    crate::{
        acpi,
        com2_print,
        com2_println,
        elf,
        interrupt,
        memory,
        timer,
    },
};

#[derive(Debug)]
pub struct Controller {
    local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure,
    paging: memory::Paging,
    stack: Option<memory::Stack>,
}

impl Controller {
    pub fn boot(&mut self, boot_loader: &mut boot::Loader, local_apic_registers: &mut interrupt::apic::local::Registers, hpet: &timer::hpet::Registers, kernel: &elf::File) {
        kernel.deploy_writable_segments(&mut self.paging);
        let stack_pages: usize = 0x10;
        let stack_floor_inclusive: usize = !0;
        self.stack = Some(memory::Stack::new(&mut self.paging, stack_floor_inclusive, stack_pages));
        let entry: usize = kernel.entry();
        com2_println!("entry = {:#x?}", entry);
        let stack_floor: usize = self.stack
            .as_ref()
            .unwrap()
            .wrapping_floor();
        com2_println!("stack_floor = {:#x?}", stack_floor);
        boot_loader.initialize(&self.paging);
        let local_apic_id: u8 = self.local_apic_id();
        com2_println!("Boot processor {:#x?}", local_apic_id);
        let entry_point: usize = boot_loader.entry_point();
        local_apic_registers.send_init(local_apic_id, hpet);
        local_apic_registers.send_sipi(local_apic_id, entry_point, hpet);
        local_apic_registers.send_sipi(local_apic_id, entry_point, hpet);
        hpet.wait_seconds(1);
        com2_println!("{}", boot_loader.log());
    }

    pub fn local_apic_id(&self) -> u8 {
        self.local_apic_structure.apic_id()
    }

    pub fn new(local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure, paging: memory::Paging) -> Self {
        let stack: Option<memory::Stack> = None;
        Self {
            local_apic_structure,
            paging,
            stack,
        }
    }
}

