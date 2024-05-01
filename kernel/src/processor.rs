pub mod boot;
pub mod message;

use {
    alloc::vec::Vec,
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
    kernel_entry: usize,
    kernel_stack: memory::Stack,
    kernel_stack_floor: usize,
    kernel_writable_pages: Vec<memory::Page>,
    local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure,
    message: Option<message::Content>,
    paging: memory::Paging,
}

impl Controller {
    pub fn local_apic_id(&self) -> u8 {
        self.local_apic_structure.apic_id()
    }

    pub fn new(local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure, mut paging: memory::Paging, kernel: &elf::File) -> Self {
        let kernel_writable_pages: Vec<memory::Page> = kernel.deploy_writable_segments(&mut paging);
        let kernel_stack_pages: usize = 0x10;
        let kernel_stack_floor_inclusive: usize = !0;
        let kernel_stack: memory::Stack = memory::Stack::new(&mut paging, kernel_stack_floor_inclusive, kernel_stack_pages);
        let kernel_entry: usize = kernel.entry();
        let kernel_stack_floor: usize = kernel_stack.wrapping_floor();
        let message: Option<message::Content> = None;
        Self {
            kernel_entry,
            kernel_stack,
            kernel_stack_floor,
            kernel_writable_pages,
            local_apic_structure,
            message,
            paging,
        }
    }

    pub fn boot(&self, boot_loader: &mut boot::Loader, local_apic_registers: &mut interrupt::apic::local::Registers, hpet: &timer::hpet::Registers, bsp_local_apic_id: u8) {
        boot_loader.initialize(&self.paging, self.kernel_entry, self.kernel_stack_floor, bsp_local_apic_id);
        let local_apic_id: u8 = self.local_apic_structure.apic_id();
        com2_println!("Boot processor {:#x?}", local_apic_id);
        let entry_point: usize = boot_loader.entry_point();
        local_apic_registers.send_init(local_apic_id, hpet);
        local_apic_registers.send_sipi(local_apic_id, entry_point, hpet);
        local_apic_registers.send_sipi(local_apic_id, entry_point, hpet);
        hpet.wait_seconds(1);
        com2_println!("{}", boot_loader.log());
    }
}

