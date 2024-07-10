pub mod boot;
pub mod message;

use {
    alloc::{
        string::String,
        vec::Vec,
    },
    core::{
        cell::OnceCell,
        mem::MaybeUninit,
        sync::atomic::{
            AtomicBool,
            Ordering,
        },
    },
    crate::{
        Argument,
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
    boot_completed: AtomicBool,
    heap: Vec<MaybeUninit<u8>>,
    initialized: bool,
    kernel_entry: usize,
    #[allow(dead_code)]
    kernel_stack: memory::Stack,
    kernel_stack_floor: usize,
    #[allow(dead_code)]
    kernel_writable_pages: Vec<memory::Page>,
    local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure,
    log: String,
    paging: memory::Paging,
    receiver: sync::spin::Lock<Option<message::Content>>,
    sender: sync::spin::Lock<Option<message::Content>>,
}

impl Controller {
    pub fn boot(&self, boot_loader: &mut boot::Loader, local_apic_registers: &mut interrupt::apic::local::Registers, hpet: &timer::hpet::Registers, bsp_local_apic_id: u8, bsp_heap_start: usize) {
        boot_loader.initialize(self, bsp_heap_start, bsp_local_apic_id);
        let local_apic_id: u8 = self.local_apic_structure.apic_id();
        com2_println!("Boot processor {:#x?}", local_apic_id);
        let entry_point: usize = boot_loader.entry_point();
        local_apic_registers.send_init(local_apic_id, hpet);
        local_apic_registers.send_sipi(local_apic_id, entry_point, hpet);
        local_apic_registers.send_sipi(local_apic_id, entry_point, hpet);
        while !self.boot_completed.load(Ordering::Acquire) {
            x64::pause();
        }
        com2_println!("{}", boot_loader.log());
    }

    pub fn boot_complete(&mut self) {
        self.boot_completed.store(true, Ordering::Release);
    }

    pub fn delete_received_messages() {
        Self::get_mut_all()
            .for_each(|controller| {
                *controller.receiver.lock() = None;
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

    pub fn heap(&self) -> &[MaybeUninit<u8>] {
        &self.heap
    }

    pub fn initialized(&mut self) {
        self.initialized = true;
    }

    pub fn kernel_entry(&self) -> usize {
        self.kernel_entry
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn kernel_stack_floor(&self) -> usize {
        self.kernel_stack_floor
    }

    pub fn local_apic_id(&self) -> u8 {
        self.local_apic_structure.apic_id()
    }

    pub fn log(&self) -> &str {
        &self.log
    }

    pub fn new(local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure, mut paging: memory::Paging, kernel: &elf::File, heap: Vec<MaybeUninit<u8>>) -> Self {
        let boot_completed: AtomicBool = AtomicBool::new(false);
        let initialized: bool = false;
        let kernel_writable_pages: Vec<memory::Page> = kernel.deploy_writable_segments(&mut paging);
        let kernel_stack_pages: usize = 0x10;
        let kernel_stack_floor_inclusive: usize = !0;
        let kernel_stack: memory::Stack = memory::Stack::new(&mut paging, kernel_stack_floor_inclusive, kernel_stack_pages);
        let kernel_entry: usize = kernel.entry();
        let kernel_stack_floor: usize = kernel_stack.wrapping_floor();
        let log = String::new();
        let receiver: sync::spin::Lock<Option<message::Content>> = sync::spin::Lock::new(None);
        let sender: sync::spin::Lock<Option<message::Content>> = sync::spin::Lock::new(None);
        Self {
            boot_completed,
            heap,
            initialized,
            kernel_entry,
            kernel_stack,
            kernel_stack_floor,
            kernel_writable_pages,
            local_apic_structure,
            log,
            paging,
            receiver,
            sender,
        }
    }

    pub fn paging(&self) -> &memory::Paging {
        &self.paging
    }

    pub fn save_received_messages() {
        Self::get_mut_all()
            .for_each(|processor| {
                let message: Option<message::Content> = processor.receiver.lock().clone();
                if let Some(message) = message {
                    match message {
                        message::Content::BootCompleted => processor.boot_complete(),
                        message => interrupt::Event::push(interrupt::Event::interprocessor(processor, message)),
                    }
                }
            });
    }

    pub fn receiver(&self) -> &sync::spin::Lock<Option<message::Content>> {
        &self.receiver
    }

    pub fn send(&mut self, message: message::Content) {
        *self.sender.lock() = Some(message);
        x64::msr::ia32::ApicBase::get(Argument::get().cpuid())
            .unwrap()
            .registers_mut()
            .send_interrupt(self.local_apic_id(), interrupt::INTERPROCESSOR_INTERRUPT);
    }

    pub fn sender(&self) -> &sync::spin::Lock<Option<message::Content>> {
        &self.sender
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

