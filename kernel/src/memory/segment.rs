pub mod descriptor;
pub mod long;
pub mod selector;
pub mod short;

pub use selector::Selector;

use {
    core::ops::Range,
    crate::x64,
};

const APPLICATION_PRIVILEGE_LEVEL: u8 = 3;
const KERNEL_PRIVILEGE_LEVEL: u8 = 0;

pub struct Gdt {
    gdt: descriptor::Table,
    application_code_segment_selector: Selector,
    application_data_segment_selector: Selector,
    kernel_code_segment_selector: Selector,
    kernel_data_segment_selector: Selector,
}

impl Gdt {
    pub fn new() -> Self {
        let mut gdt = descriptor::Table::get();
        let gdtr: descriptor::table::Register = (&gdt).into();
        gdtr.set();
        let cs = Selector::cs();
        let ds = Selector::ds();
        let kernel_code_segment_descriptor: descriptor::Interface = gdt
            .descriptor(&cs)
            .unwrap();
        let kernel_data_segment_descriptor: descriptor::Interface = gdt
            .descriptor(&ds)
            .unwrap();
        let application_code_segment_descriptor: descriptor::Interface = kernel_code_segment_descriptor
            .with_dpl(APPLICATION_PRIVILEGE_LEVEL);
        let application_data_segment_descriptor: descriptor::Interface = kernel_data_segment_descriptor
            .with_dpl(APPLICATION_PRIVILEGE_LEVEL);
        let segment_descriptors = [
            kernel_code_segment_descriptor,
            kernel_data_segment_descriptor,
            application_data_segment_descriptor,
            application_code_segment_descriptor,
        ];
        let segment_descriptors: &[descriptor::Interface] = segment_descriptors.as_slice();
        let mut segment_descriptor_indices: Range<usize> = gdt.continuous_free_descriptor_indices(segment_descriptors.len()).unwrap();
        segment_descriptor_indices
            .clone()
            .zip(segment_descriptors.iter())
            .for_each(|(index, descriptor)| gdt.set_descriptor(index, descriptor));
        let kernel_code_segment_index: usize = segment_descriptor_indices.next().unwrap();
        let kernel_data_segment_index: usize = segment_descriptor_indices.next().unwrap();
        let application_data_segment_index: usize = segment_descriptor_indices.next().unwrap();
        let application_code_segment_index: usize = segment_descriptor_indices.next().unwrap();
        let is_ldt: bool = false;
        let kernel_code_segment_selector = Selector::create(kernel_code_segment_index as u16, is_ldt, KERNEL_PRIVILEGE_LEVEL);
        let kernel_data_segment_selector = Selector::create(kernel_data_segment_index as u16, is_ldt, KERNEL_PRIVILEGE_LEVEL);
        let application_code_segment_selector = Selector::create(application_code_segment_index as u16, is_ldt, APPLICATION_PRIVILEGE_LEVEL);
        let application_data_segment_selector = Selector::create(application_data_segment_index as u16, is_ldt, APPLICATION_PRIVILEGE_LEVEL);
        x64::set_segment_registers(&kernel_code_segment_selector, &kernel_data_segment_selector); // Don't rewrite segment registers before exiting boot services.
        Self {
            gdt,
            application_code_segment_selector,
            application_data_segment_selector,
            kernel_code_segment_selector,
            kernel_data_segment_selector,
        }
    }

    pub fn application_code_segment_selector(&self) -> &Selector {
        &self.application_code_segment_selector
    }

    pub fn application_data_segment_selector(&self) -> &Selector {
        &self.application_data_segment_selector
    }

    pub fn kernel_code_segment_selector(&self) -> &Selector {
        &self.kernel_code_segment_selector
    }

    pub fn kernel_data_segment_selector(&self) -> &Selector {
        &self.kernel_data_segment_selector
    }

    pub fn set_task_state_segment_descriptor(&mut self, task_state_segment_descriptor: &long::Descriptor) -> Selector {
        self.gdt.set_task_state_segment_descriptor(task_state_segment_descriptor)
    }
}

