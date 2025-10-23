pub mod boot;
pub mod message;

use {
    crate::{acpi, com2_println, elf, interrupt, memory, sync, timer, x64, Argument},
    alloc::{collections::BTreeMap, string::String, vec::Vec},
    core::{
        cell::OnceCell,
        mem::MaybeUninit,
        sync::atomic::{AtomicBool, Ordering},
    },
};

static mut MANAGER: OnceCell<Manager> = OnceCell::new();

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
    pub fn boot(
        &self,
        boot_loader: &mut boot::Loader,
        local_apic_registers: &mut interrupt::apic::local::Registers,
        hpet: &timer::hpet::Registers,
        bsp_local_apic_id: u8,
        bsp_heap_start: usize,
    ) {
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
        Self::get_mut_all().for_each(|controller| {
            *controller.receiver.lock() = None;
        });
    }

    pub fn get_all() -> impl Iterator<Item = &'static Self> {
        unsafe { MANAGER.get() }.unwrap().controllers.iter()
    }

    pub fn get_mut_all() -> impl Iterator<Item = &'static mut Self> {
        unsafe { MANAGER.get_mut() }.unwrap().controllers.iter_mut()
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

    pub fn new(
        local_apic_structure: acpi::multiple_apic_description::processor_local_apic::Structure,
        mut paging: memory::Paging,
        kernel: &elf::File,
        heap: Vec<MaybeUninit<u8>>,
    ) -> Self {
        let boot_completed: AtomicBool = AtomicBool::new(false);
        let initialized: bool = false;
        let kernel_writable_pages: Vec<memory::Page> = kernel.deploy_writable_segments(&mut paging);
        let kernel_stack_pages: usize = 0x10;
        let kernel_stack_floor_inclusive: usize = !0;
        let kernel_stack: memory::Stack = memory::Stack::new(
            &mut paging,
            kernel_stack_floor_inclusive,
            kernel_stack_pages,
        );
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
        Self::get_mut_all().for_each(|processor| {
            let message: Option<message::Content> = processor.receiver.lock().clone();
            if let Some(message) = message {
                match message {
                    message::Content::BootCompleted => processor.boot_complete(),
                    message => {
                        interrupt::Event::push(interrupt::Event::interprocessor(processor, message))
                    }
                }
            }
        });
    }

    pub fn receiver(&self) -> &sync::spin::Lock<Option<message::Content>> {
        &self.receiver
    }

    pub fn send(&mut self, message: message::Content) {
        *self.sender.lock() = Some(message);
        x64::msr::ia32::ApicBase::get()
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
}

#[derive(Debug)]
pub struct Manager {
    controllers: Vec<Controller>,
    #[allow(dead_code)]
    kernel: elf::File,
    #[allow(dead_code)]
    kernel_read_only_pages: Vec<memory::Page>,
    #[allow(dead_code)]
    paging: memory::Paging,
}

impl Manager {
    pub fn initialize(
        local_apic_id: u8,
        local_apic_registers: &mut interrupt::apic::local::Registers,
        heap_size: usize,
        hpet: &timer::hpet::Registers,
    ) {
        let mut paging: memory::Paging = Argument::get().paging().clone();
        let kernel: elf::File = Argument::get().processor_kernel().to_vec().into();
        let kernel_read_only_pages: Vec<memory::Page> =
            kernel.deploy_unwritable_segments(&mut paging);
        let processors: Vec<acpi::multiple_apic_description::processor_local_apic::Structure> =
            Argument::get()
                .efi_system_table()
                .rsdp()
                .xsdt()
                .madt()
                .processor_local_apic_structures()
                .into_iter()
                .filter(|local_apic| local_apic.is_enabled())
                .collect();
        let number_of_processors: usize = processors.len();
        com2_println!("number_of_processors = {:#x?}", number_of_processors);
        let heap_size: usize = (heap_size / number_of_processors + 1).next_power_of_two();
        let heap_size: usize = heap_size
            / if heap_size / 2 + (number_of_processors - 1) * heap_size < heap_size {
                1
            } else {
                2
            };
        com2_println!("heap_size = {:#x?}", heap_size);
        let controllers: Vec<Controller> = processors
            .into_iter()
            .filter(|local_apic| local_apic.apic_id() != local_apic_id)
            .map(|local_apic| {
                let mut heap: Vec<MaybeUninit<u8>> = Vec::with_capacity(heap_size);
                unsafe {
                    heap.set_len(heap_size);
                }
                Controller::new(local_apic.clone(), paging.clone(), &kernel, heap)
            })
            .collect();
        let manager = Self {
            controllers,
            kernel,
            kernel_read_only_pages,
            paging,
        };
        unsafe { MANAGER.set(manager) }.unwrap();
        Controller::get_all().for_each(|processor| {
            processor.boot(
                Argument::get().processor_boot_loader_mut(),
                local_apic_registers,
                hpet,
                local_apic_id,
                Argument::get().heap_start(),
            )
        });
    }

    pub fn finalize() {
        let local_apic_id2log: BTreeMap<u8, &str> = Controller::get_all()
            .map(|processor| (processor.local_apic_id(), processor.log()))
            .collect();
        local_apic_id2log
            .into_iter()
            .for_each(|(local_apic_id, log)| {
                com2_println!("Application processor log");
                com2_println!("Local APIC ID = {:#x?}", local_apic_id);
                com2_println!("{}", log);
            });
    }
}
