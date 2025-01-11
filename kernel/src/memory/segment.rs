pub mod descriptor;
pub mod long;
pub mod short;

use {
    bitfield_struct::bitfield,
    core::arch::asm,
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

/// # Segment Selector
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 3.4.2 Segment Selectors, Figure 3-6. Segment Selector
#[bitfield(u16)]
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Selector {
    #[bits(2)]
    rpl: u8,
    ti: bool,
    #[bits(13)]
    index: u16,
}

impl Selector {
    pub fn create(index: u16, ti: bool, rpl: u8) -> Self {
        Self::default()
            .with_index(index)
            .with_ti(ti)
            .with_rpl(rpl)
    }

    #[inline(never)]
    pub fn cs() -> Self {
        let cs: u16;
        unsafe {
            asm!(
                "mov {0:x}, cs",
                out(reg) cs,
            );
        }
        cs.into()
    }

    #[inline(never)]
    pub fn ds() -> Self {
        let ds: u16;
        unsafe {
            asm!(
                "mov {0:x}, ds",
                out(reg) ds,
            );
        }
        ds.into()
    }

    #[inline(never)]
    pub fn es() -> Self {
        let es: u16;
        unsafe {
            asm!(
                "mov {0:x}, es",
                out(reg) es,
            );
        }
        es.into()
    }

    #[inline(never)]
    pub fn fs() -> Self {
        let fs: u16;
        unsafe {
            asm!(
                "mov {0:x}, fs",
                out(reg) fs,
            );
        }
        fs.into()
    }

    pub fn get_index(&self) -> u16 {
        self.index()
    }

    pub fn get_rpl(&self) -> u8 {
        self.rpl()
    }

    #[inline(never)]
    pub fn gs() -> Self {
        let gs: u16;
        unsafe {
            asm!(
                "mov {0:x}, gs",
                out(reg) gs,
            );
        }
        gs.into()
    }

    #[inline(never)]
    pub fn ss() -> Self {
        let ss: u16;
        unsafe {
            asm!(
                "mov {0:x}, ss",
                out(reg) ss,
            );
        }
        ss.into()
    }
}

