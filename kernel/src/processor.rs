pub mod boot;
pub mod message;

use {
    alloc::{
        collections::BTreeMap,
        string::String,
        vec::Vec,
    },
    core::{
        cell::OnceCell,
        mem::MaybeUninit,
    },
    crate::{
        acpi,
        com2_print,
        com2_println,
        elf,
        interrupt,
        memory,
        sync,
        timer,
        x64,
    },
};

static mut CONTROLLERS: OnceCell<Vec<Controller>> = OnceCell::new();

#[derive(Debug)]
pub struct Controller {
    boot_completed: bool,
    heap: Vec<MaybeUninit<u8>>,
    kernel_completed: bool,
    kernel_entry: usize,
    kernel_stack: memory::Stack,
    kernel_stack_floor: usize,
    kernel_writable_pages: Vec<memory::Page>,
    local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure,
    log: String,
    message: sync::spin::Lock<Option<message::Content>>,
    paging: memory::Paging,
}

impl Controller {
    pub fn boot(&self, boot_loader: &mut boot::Loader, local_apic_registers: &mut interrupt::apic::local::Registers, hpet: &timer::hpet::Registers, bsp_local_apic_id: u8, bsp_heap_start: usize) {
        boot_loader.initialize(&self.paging, self.kernel_entry, self.kernel_stack_floor, bsp_heap_start, &self.heap, bsp_local_apic_id, &self.message);
        let local_apic_id: u8 = self.local_apic_structure.apic_id();
        com2_println!("Boot processor {:#x?}", local_apic_id);
        let entry_point: usize = boot_loader.entry_point();
        local_apic_registers.send_init(local_apic_id, hpet);
        local_apic_registers.send_sipi(local_apic_id, entry_point, hpet);
        local_apic_registers.send_sipi(local_apic_id, entry_point, hpet);
        while !self.boot_completed {
            x64::pause();
        }
        com2_println!("{}", boot_loader.log());
    }

    pub fn boot_complete(&mut self) {
        self.boot_completed = true;
    }

    pub fn delete_messages() {
        Self::get_mut_all()
            .for_each(|controller| {
                *controller.message.lock() = None;
            });
    }

    pub fn get_all() -> impl Iterator<Item = &'static Self> {
        unsafe {
            CONTROLLERS.get()
        }   .unwrap()
            .iter()
    }

    pub fn get_mut_all() -> impl Iterator<Item = &'static mut Self> {
        unsafe {
            CONTROLLERS.get_mut()
        }   .unwrap()
            .iter_mut()
    }

    pub fn kernel_complete(&mut self) {
        self.kernel_completed = true;
    }

    pub fn kernel_is_completed(&self) -> bool {
        self.kernel_completed
    }

    pub fn local_apic_id(&self) -> u8 {
        self.local_apic_structure.apic_id()
    }

    pub fn log(&self) -> &str {
        &self.log
    }

    pub fn new(local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure, mut paging: memory::Paging, kernel: &elf::File, heap: Vec<MaybeUninit<u8>>) -> Self {
        let boot_completed: bool = false;
        let kernel_completed: bool = false;
        let kernel_writable_pages: Vec<memory::Page> = kernel.deploy_writable_segments(&mut paging);
        let kernel_stack_pages: usize = 0x10;
        let kernel_stack_floor_inclusive: usize = !0;
        let kernel_stack: memory::Stack = memory::Stack::new(&mut paging, kernel_stack_floor_inclusive, kernel_stack_pages);
        let kernel_entry: usize = kernel.entry();
        let kernel_stack_floor: usize = kernel_stack.wrapping_floor();
        let log = String::new();
        let message: sync::spin::Lock<Option<message::Content>> = sync::spin::Lock::new(None);
        Self {
            boot_completed,
            heap,
            kernel_completed,
            kernel_entry,
            kernel_stack,
            kernel_stack_floor,
            kernel_writable_pages,
            local_apic_structure,
            log,
            message,
            paging,
        }
    }

    pub fn process_messages() {
        Self::get_mut_all()
            .for_each(|controller| {
                let message: Option<message::Content> = controller.message.lock().clone();
                if let Some(message) = message {
                    message.process(controller);
                }
            })
    }

    pub fn receive_character(&mut self, character: char) {
        self.log.push(character);
    }

    pub fn set_all(controllers: Vec<Self>) {
        unsafe {
            CONTROLLERS.set(controllers)
        }.unwrap();
    }
}

