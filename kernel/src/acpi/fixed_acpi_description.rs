pub mod pm1;

use {
    alloc::vec,
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
    },
    crate::{
        com2_println,
        io,
        x64,
    },
    super::{
        firmware_acpi_control,
        generic_address,
        machine_language::{
            interpreter::Evaluator,
            self,
            syntax::{
                FirstReader,
                ReaderOutsideMethod,
            },
        },
        system_description,
    },
};

/// # FADT
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.9 Fixed ACPI Description Table (FADT)
#[derive(repr_packed_struct::OffsetGetter)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    firmware_ctrl: u32,
    dsdt: u32,
    reserved0: u8,
    preferred_pm_profile: u8,
    sci_int: u16,
    smi_cmd: u32,
    acpi_enable: u8,
    acpi_disable: u8,
    s4bios_req: u8,
    pstate_cnt: u8,
    pm1a_evt_blk: u32,
    pm1b_evt_blk: u32,
    pm1a_cnt_blk: u32,
    pm1b_cnt_blk: u32,
    pm2_cnt_blk: u32,
    pm_tmr_blk: u32,
    gpe0_blk: u32,
    gpe1_blk: u32,
    pm1_evt_len: u8,
    pm1_cnt_len: u8,
    pm2_cnt_len: u8,
    pm_tmr_len: u8,
    gpe0_blk_len: u8,
    gpe1_blk_len: u8,
    gpe1_base: u8,
    cst_cnt: u8,
    p_lvl2_lat: u16,
    p_lvl3_lat: u16,
    flush_size: u16,
    flush_stride: u16,
    duty_offset: u8,
    duty_width: u8,
    day_alrm: u8,
    mon_alrm: u8,
    century: u8,
    iapc_boot_arch: u16,
    reserved1: u8,
    flags: Flags,
    reset_reg: generic_address::Structure,
    reser_value: u8,
    arm_boot_arch: u16,
    fadt_minor_version: u8,
    x_firmware_ctrl: u64,
    x_dsdt: u64,
    x_pm1a_evt_blk: generic_address::Structure,
    x_pm1b_evt_blk: generic_address::Structure,
    x_pm1a_cnt_blk: generic_address::Structure,
    x_pm1b_cnt_blk: generic_address::Structure,
    x_pm2_cnt_blk: generic_address::Structure,
    x_pm_tmr_blk: generic_address::Structure,
    x_gpe0_blk: generic_address::Structure,
    x_gpe1_blk: generic_address::Structure,
    sleep_control_reg: generic_address::Structure,
    sleep_status_reg: generic_address::Structure,
    hypervisor_vendor_identity: u64,
}

impl Table {
    pub fn timer(&self) -> Option<io::Mapped> {
        (self.pm_tmr_len == 4).then(|| {
            let self_address: *const Self = self as *const Self;
            let self_address: usize = self_address as usize;
            let x_pm_tmr_blk_address: *const generic_address::Structure = (&self.x_pm_tmr_blk) as *const generic_address::Structure;
            let x_pm_tmr_blk_address: usize = x_pm_tmr_blk_address as usize;
            if x_pm_tmr_blk_address + mem::size_of::<generic_address::Structure>() <= self_address + self.header.table_size() {
                let x_pm_tmr_blk: generic_address::Structure = self.x_pm_tmr_blk;
                if x_pm_tmr_blk.is_null() {
                    io::Mapped::port(self.pm_tmr_blk as u16)
                } else {
                    (&x_pm_tmr_blk).into()
                }
            } else {
                io::Mapped::port(self.pm_tmr_blk as u16)
            }
        })
    }

    pub fn century(&self) -> u8 {
        self.century
    }

    pub fn is_correct(&self) -> bool {
        self.header.is_correct() && self.dsdt().map_or(true, |dsdt| dsdt.is_correct())
    }

    pub fn shutdown(&mut self) {
        let dsdt: system_description::Table = self
            .dsdt()
            .unwrap();
        let iasl_input: &[u8] = (&dsdt).into();
        com2_println!("iasl_input = {:02x?}", iasl_input);
        let dsdt: &[u8] = dsdt.definition_block();
        let mut semantic_tree = machine_language::name::Node::default();
        let current = machine_language::name::Path::root();
        let (mut syntax_tree, unread_dsdt): (machine_language::syntax::TermList, &[u8]) = machine_language::syntax::TermList::first_read(dsdt, &mut semantic_tree, &current);
        assert!(unread_dsdt.is_empty());
        syntax_tree.read_outside_method(&mut semantic_tree, &current);
        let reference_tree: machine_language::reference::Node = (&syntax_tree).into();
        let stack_frame = machine_language::interpreter::StackFrame::default().set_arguments(vec![machine_language::interpreter::Value::Byte(0x05)]);
        let tts: machine_language::name::Path = "\\_TTS".into();
        let tts: Option<&machine_language::syntax::DefMethod> = reference_tree.get_method(&tts);
        if let Some(tts) = tts {
            tts.evaluate(&mut stack_frame.clone(), &reference_tree, &current);
        }
        let pts: machine_language::name::Path = "\\_PTS".into();
        let pts: Option<&machine_language::syntax::DefMethod> = reference_tree.get_method(&pts);
        if let Some(pts) = pts {
            pts.evaluate(&mut stack_frame.clone(), &reference_tree, &current);
        }
        let pm1a_status: Option<pm1::status::Register> = self.read_pm1a_status();
        let pm1b_status: Option<pm1::status::Register> = self.read_pm1b_status();
        let pm1a_enable: Option<pm1::enable::Register> = self.read_pm1a_enable();
        let pm1b_enable: Option<pm1::enable::Register> = self.read_pm1b_enable();
        let pm1a_control: Option<pm1::control::Register> = self.read_pm1a_control();
        let pm1b_control: Option<pm1::control::Register> = self.read_pm1b_control();
        let s5: machine_language::name::Path = "\\_S5".into();
        let s5: Option<&machine_language::syntax::DefName> = reference_tree.get_name(&s5);
        let s5: Option<machine_language::interpreter::Value> = s5.and_then(|s5| s5.evaluate(&mut machine_language::interpreter::StackFrame::default(), &reference_tree, &current));
        let pm1a_cnt_slp_typ: Option<u8> = s5
            .as_ref()
            .and_then(|s5| s5.get_element(0))
            .and_then(|pm1a_cnt_slp_typ| pm1a_cnt_slp_typ.get_byte());
        let pm1b_cnt_slp_typ: Option<u8> = s5
            .as_ref()
            .and_then(|s5| s5.get_element(1))
            .and_then(|pm1b_cnt_slp_typ| pm1b_cnt_slp_typ.get_byte());
        com2_println!("syntax_tree = {:#x?}", syntax_tree);
        com2_println!("semantic_tree = {:#x?}", semantic_tree);
        com2_println!("reference_tree = {:#x?}", reference_tree);
        com2_println!("stack_frame = {:#x?}", stack_frame);
        com2_println!("tts = {:#x?}", tts);
        com2_println!("pts = {:#x?}", pts);
        com2_println!("s5 = {:#x?}", s5);
        com2_println!("pm1a_status = {:#x?}", pm1a_status);
        com2_println!("pm1b_status = {:#x?}", pm1b_status);
        com2_println!("pm1a_enable = {:#x?}", pm1a_enable);
        com2_println!("pm1b_enable = {:#x?}", pm1b_enable);
        com2_println!("pm1a_control = {:#x?}", pm1a_control);
        com2_println!("pm1b_control = {:#x?}", pm1b_control);
        com2_println!("pm1a_cnt_slp_typ = {:#x?}", pm1a_cnt_slp_typ);
        com2_println!("pm1b_cnt_slp_typ = {:#x?}", pm1b_cnt_slp_typ);
        if let Some((pm1a_control, pm1a_cnt_slp_typ)) = pm1a_control.zip(pm1a_cnt_slp_typ) {
            self.write_pm1a_control(pm1a_control.sleep(pm1a_cnt_slp_typ));
        }
        if let Some((pm1b_control, pm1b_cnt_slp_typ)) = pm1b_control.zip(pm1b_cnt_slp_typ) {
            self.write_pm1b_control(pm1b_control.sleep(pm1b_cnt_slp_typ));
        }
        loop {
            x64::pause();
        }
    }

    pub fn timer_bits(&self) -> usize {
        let flags: Flags = self.flags;
        flags.timer_bits()
    }

    fn dsdt(&self) -> Option<system_description::Table> {
        let dsdt: Option<usize> = (44 <= self.header.table_size()).then_some(self.dsdt as usize);
        let x_dsdt: Option<usize> = (148 <= self.header.table_size()).then_some(self.x_dsdt as usize);
        dsdt
            .iter()
            .chain(x_dsdt.iter())
            .max()
            .filter(|dsdt| **dsdt != 0)
            .map(|dsdt| {
                let header: *const system_description::Header = (*dsdt) as *const system_description::Header;
                let header: &system_description::Header = unsafe {
                    &*header
                };
                header.into()
            })
    }

    fn firmware_ctrl(&self) -> Option<&firmware_acpi_control::Structure> {
        let firmware_ctrl: Option<usize> = (40 <= self.header.table_size()).then_some(self.firmware_ctrl as usize);
        let x_firmware_ctrl: Option<usize> = (140 <= self.header.table_size()).then_some(self.x_firmware_ctrl as usize);
        firmware_ctrl
            .iter()
            .chain(x_firmware_ctrl.iter())
            .max()
            .filter(|firmware_ctrl| **firmware_ctrl != 0)
            .map(|firmware_ctrl| {
                let firmware_ctrl: *const firmware_acpi_control::Structure = (*firmware_ctrl) as *const firmware_acpi_control::Structure;
                unsafe {
                    &*firmware_ctrl
                }
            })
    }

    fn pm1a_cnt_blk(&self) -> Option<generic_address::Structure> {
        (Self::pm1a_cnt_blk_offset() < self.header.table_size())
            .then_some(self.pm1a_cnt_blk)
            .filter(|pm1a_cnt_blk| *pm1a_cnt_blk != 0)
            .map(|pm1a_cnt_blk| {
                let access_size: usize = mem::size_of::<u16>();
                generic_address::Structure::system_io(pm1a_cnt_blk as u16, access_size)
            })
    }

    fn pm1b_cnt_blk(&self) -> Option<generic_address::Structure> {
        (Self::pm1b_cnt_blk_offset() < self.header.table_size())
            .then_some(self.pm1b_cnt_blk)
            .filter(|pm1b_cnt_blk| *pm1b_cnt_blk != 0)
            .map(|pm1b_cnt_blk| {
                let access_size: usize = mem::size_of::<u16>();
                generic_address::Structure::system_io(pm1b_cnt_blk as u16, access_size)
            })
    }

    fn pm1a_evt_blk(&self) -> Option<generic_address::Structure> {
        (Self::pm1a_evt_blk_offset() < self.header.table_size())
            .then_some(self.pm1a_evt_blk)
            .filter(|pm1a_evt_blk| *pm1a_evt_blk != 0)
            .map(|pm1a_evt_blk| {
                let access_size: usize = mem::size_of::<u16>();
                generic_address::Structure::system_io(pm1a_evt_blk as u16, access_size)
            })
    }

    fn pm1b_evt_blk(&self) -> Option<generic_address::Structure> {
        (Self::pm1b_evt_blk_offset() < self.header.table_size())
            .then_some(self.pm1b_evt_blk)
            .filter(|pm1b_evt_blk| *pm1b_evt_blk != 0)
            .map(|pm1b_evt_blk| {
                let access_size: usize = mem::size_of::<u16>();
                generic_address::Structure::system_io(pm1b_evt_blk as u16, access_size)
            })
    }

    fn read_pm1a_control(&self) -> Option<pm1::control::Register> {
        self.x_pm1a_cnt_blk()
            .or(self.pm1a_cnt_blk())
            .map(|pm1a_cnt_blk| pm1a_cnt_blk
                .read_word()
                .into())
    }

    fn read_pm1b_control(&self) -> Option<pm1::control::Register> {
        self.x_pm1b_cnt_blk()
            .or(self.pm1b_cnt_blk())
            .map(|pm1b_cnt_blk| pm1b_cnt_blk
                .read_word()
                .into())
    }

    fn read_pm1a_enable(&self) -> Option<pm1::enable::Register> {
        self.x_pm1a_evt_blk()
            .or(self.pm1a_evt_blk())
            .map(|pm1a_evt_blk| pm1a_evt_blk
                .add((self.pm1_evt_len as usize) / 2)
                .read_word()
                .into())
    }

    fn read_pm1b_enable(&self) -> Option<pm1::enable::Register> {
        self.x_pm1b_evt_blk()
            .or(self.pm1b_evt_blk())
            .map(|pm1b_evt_blk| pm1b_evt_blk
                .add((self.pm1_evt_len as usize) / 2)
                .read_word()
                .into())
    }

    fn read_pm1a_status(&self) -> Option<pm1::status::Register> {
        self.x_pm1a_evt_blk()
            .or(self.pm1a_evt_blk())
            .map(|pm1a_evt_blk| pm1a_evt_blk
                .read_word()
                .into())
    }

    fn read_pm1b_status(&self) -> Option<pm1::status::Register> {
        self.x_pm1b_evt_blk()
            .or(self.pm1b_evt_blk())
            .map(|pm1b_evt_blk| pm1b_evt_blk
                .read_word()
                .into())
    }

    fn write_pm1a_control(&mut self, pm1a_cnt: pm1::control::Register) {
        if let Some(mut pm1a_cnt_blk) = self
            .x_pm1a_cnt_blk()
            .or(self.pm1a_cnt_blk()) {
            pm1a_cnt_blk.write_word(pm1a_cnt.into());
        }
    }

    fn write_pm1b_control(&mut self, pm1b_cnt: pm1::control::Register) {
        if let Some(mut pm1b_cnt_blk) = self
            .x_pm1b_cnt_blk()
            .or(self.pm1b_cnt_blk()) {
            pm1b_cnt_blk.write_word(pm1b_cnt.into());
        }
    }

    fn x_pm1a_cnt_blk(&self) -> Option<generic_address::Structure> {
        (Self::x_pm1a_cnt_blk_offset() < self.header.table_size())
            .then_some(self.x_pm1a_cnt_blk)
            .filter(|x_pm1a_cnt_blk| x_pm1a_cnt_blk.address() != 0)
    }

    fn x_pm1b_cnt_blk(&self) -> Option<generic_address::Structure> {
        (Self::x_pm1b_cnt_blk_offset() < self.header.table_size())
            .then_some(self.x_pm1b_cnt_blk)
            .filter(|x_pm1b_cnt_blk| x_pm1b_cnt_blk.address() != 0)
    }

    fn x_pm1a_evt_blk(&self) -> Option<generic_address::Structure> {
        (Self::x_pm1a_evt_blk_offset() < self.header.table_size())
            .then_some(self.x_pm1a_evt_blk)
            .filter(|x_pm1a_evt_blk| x_pm1a_evt_blk.address() != 0)
    }

    fn x_pm1b_evt_blk(&self) -> Option<generic_address::Structure> {
        (Self::x_pm1b_evt_blk_offset() < self.header.table_size())
            .then_some(self.x_pm1b_evt_blk)
            .filter(|x_pm1b_evt_blk| x_pm1b_evt_blk.address() != 0)
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let firmware_ctrl: Option<&firmware_acpi_control::Structure> = self.firmware_ctrl();
        let dsdt: Option<system_description::Table> = self.dsdt();
        let reserved0: u8 = self.reserved0;
        let preferred_pm_profile: u8 = self.preferred_pm_profile;
        let sci_int: u16 = self.sci_int;
        let smi_cmd: u32 = self.smi_cmd;
        let acpi_enable: u8 = self.acpi_enable;
        let acpi_disable: u8 = self.acpi_disable;
        let s4bios_req: u8 = self.s4bios_req;
        let pstate_cnt: u8 = self.pstate_cnt;
        let pm1a_evt_blk: u32 = self.pm1a_evt_blk;
        let pm1b_evt_blk: u32 = self.pm1b_evt_blk;
        let pm1a_cnt_blk: u32 = self.pm1a_cnt_blk;
        let pm1b_cnt_blk: u32 = self.pm1b_cnt_blk;
        let pm2_cnt_blk: u32 = self.pm2_cnt_blk;
        let pm_tmr_blk: u32 = self.pm_tmr_blk;
        let gpe0_blk: u32 = self.gpe0_blk;
        let gpe1_blk: u32 = self.gpe1_blk;
        let pm1_evt_len: u8 = self.pm1_evt_len;
        let pm1_cnt_len: u8 = self.pm1_cnt_len;
        let pm2_cnt_len: u8 = self.pm2_cnt_len;
        let pm_tmr_len: u8 = self.pm_tmr_len;
        let gpe0_blk_len: u8 = self.gpe0_blk_len;
        let gpe1_blk_len: u8 = self.gpe1_blk_len;
        let gpe1_base: u8 = self.gpe1_base;
        let cst_cnt: u8 = self.cst_cnt;
        let p_lvl2_lat: u16 = self.p_lvl2_lat;
        let p_lvl3_lat: u16 = self.p_lvl3_lat;
        let flush_size: u16 = self.flush_size;
        let flush_stride: u16 = self.flush_stride;
        let duty_offset: u8 = self.duty_offset;
        let duty_width: u8 = self.duty_width;
        let day_alrm: u8 = self.day_alrm;
        let mon_alrm: u8 = self.mon_alrm;
        let century: u8 = self.century;
        let iapc_boot_arch: u16 = self.iapc_boot_arch;
        let reserved1: u8 = self.reserved1;
        let flags: Flags = self.flags;
        let reset_reg: generic_address::Structure = self.reset_reg;
        let reser_value: u8 = self.reser_value;
        let arm_boot_arch: u16 = self.arm_boot_arch;
        let fadt_minor_version: u8 = self.fadt_minor_version;
        let x_pm1a_evt_blk: generic_address::Structure = self.x_pm1a_evt_blk;
        let x_pm1b_evt_blk: generic_address::Structure = self.x_pm1b_evt_blk;
        let x_pm1a_cnt_blk: generic_address::Structure = self.x_pm1a_cnt_blk;
        let x_pm1b_cnt_blk: generic_address::Structure = self.x_pm1b_cnt_blk;
        let x_pm2_cnt_blk: generic_address::Structure = self.x_pm2_cnt_blk;
        let x_pm_tmr_blk: generic_address::Structure = self.x_pm_tmr_blk;
        let x_gpe0_blk: generic_address::Structure = self.x_gpe0_blk;
        let x_gpe1_blk: generic_address::Structure = self.x_gpe1_blk;
        let sleep_control_reg: generic_address::Structure = self.sleep_control_reg;
        let sleep_status_reg: generic_address::Structure = self.sleep_status_reg;
        let hypervisor_vendor_identity: u64 = self.hypervisor_vendor_identity;
        formatter
            .debug_struct("Table")
            .field("header", &header)
            .field("firmware_ctrl", &firmware_ctrl)
            .field("dsdt", &dsdt)
            .field("reserved0", &reserved0)
            .field("preferred_pm_profile", &preferred_pm_profile)
            .field("sci_int", &sci_int)
            .field("smi_cmd", &smi_cmd)
            .field("acpi_enable", &acpi_enable)
            .field("acpi_disable", &acpi_disable)
            .field("s4bios_req", &s4bios_req)
            .field("pstate_cnt", &pstate_cnt)
            .field("pm1a_evt_blk", &pm1a_evt_blk)
            .field("pm1b_evt_blk", &pm1b_evt_blk)
            .field("pm1a_cnt_blk", &pm1a_cnt_blk)
            .field("pm1b_cnt_blk", &pm1b_cnt_blk)
            .field("pm2_cnt_blk", &pm2_cnt_blk)
            .field("pm_tmr_blk", &pm_tmr_blk)
            .field("gpe0_blk", &gpe0_blk)
            .field("gpe1_blk", &gpe1_blk)
            .field("pm1_evt_len", &pm1_evt_len)
            .field("pm1_cnt_len", &pm1_cnt_len)
            .field("pm2_cnt_len", &pm2_cnt_len)
            .field("pm_tmr_len", &pm_tmr_len)
            .field("gpe0_blk_len", &gpe0_blk_len)
            .field("gpe1_blk_len", &gpe1_blk_len)
            .field("gpe1_base", &gpe1_base)
            .field("cst_cnt", &cst_cnt)
            .field("p_lvl2_lat", &p_lvl2_lat)
            .field("p_lvl3_lat", &p_lvl3_lat)
            .field("flush_size", &flush_size)
            .field("flush_stride", &flush_stride)
            .field("duty_offset", &duty_offset)
            .field("duty_width", &duty_width)
            .field("day_alrm", &day_alrm)
            .field("mon_alrm", &mon_alrm)
            .field("century", &century)
            .field("iapc_boot_arch", &iapc_boot_arch)
            .field("reserved1", &reserved1)
            .field("flags", &flags)
            .field("reset_reg", &reset_reg)
            .field("reser_value", &reser_value)
            .field("arm_boot_arch", &arm_boot_arch)
            .field("fadt_minor_version", &fadt_minor_version)
            .field("x_pm1a_evt_blk", &x_pm1a_evt_blk)
            .field("x_pm1b_evt_blk", &x_pm1b_evt_blk)
            .field("x_pm1a_cnt_blk", &x_pm1a_cnt_blk)
            .field("x_pm1b_cnt_blk", &x_pm1b_cnt_blk)
            .field("x_pm2_cnt_blk", &x_pm2_cnt_blk)
            .field("x_pm_tmr_blk", &x_pm_tmr_blk)
            .field("x_gpe0_blk", &x_gpe0_blk)
            .field("x_gpe1_blk", &x_gpe1_blk)
            .field("sleep_control_reg", &sleep_control_reg)
            .field("sleep_status_reg", &sleep_status_reg)
            .field("hypervisor_vendor_identity", &hypervisor_vendor_identity)
            .finish()
    }
}

/// # Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.9 Table 5.10 Fixed ACPI Description Table Fixed Feature Flags
#[bitfield(u32)]
struct Flags {
    wbinvd: bool,
    wbinvd_flush: bool,
    proc_c1: bool,
    p_lvl2_up: bool,
    pwr_button: bool,
    slp_button: bool,
    fix_rtc: bool,
    rtc_s4: bool,
    tmr_val_ext: bool,
    dck_cap: bool,
    reser_reg_sup: bool,
    sealed_case: bool,
    headless: bool,
    cpu_sw_slp: bool,
    pci_exp_wak: bool,
    use_platform_clock: bool,
    s4_rtc_sts_valid: bool,
    remote_power_on_capable: bool,
    force_apic_cluster_model: bool,
    force_apic_physical_destination_mode: bool,
    hw_reduced_acpi: bool,
    low_power_s0_idle_capable: bool,
    #[bits(2)]
    persistent_cpu_caches: u8,
    #[bits(access = RO)]
    reserved0: u8,
}

impl Flags {
    fn timer_bits(&self) -> usize {
        if self.tmr_val_ext() {
            32
        } else {
            24
        }
    }
}

