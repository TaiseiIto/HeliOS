use {
    core::{
        cmp,
        fmt,
    },
    super::{
        firmware_acpi_control,
        system_description,
    },
};

/// # FADT
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.9 Fixed ACPI Description Table (FADT)
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
    flags: u32,
    reset_reg: [u8; 12],
    reser_value: u8,
    arm_boot_arch: u16,
    fadt_minor_version: u8,
    x_firmware_ctrl: u64,
    x_dsdt: u64,
    x_pm1a_evt_blk: [u8; 12],
    x_pm1b_evt_blk: [u8; 12],
    x_pm1a_cnt_blk: [u8; 12],
    x_pm1b_cnt_blk: [u8; 12],
    x_pm2_cnt_blk: [u8; 12],
    x_pm_tmr_blk: [u8; 12],
    x_gpe0_blk: [u8; 12],
    x_gpe1_blk: [u8; 12],
    sleep_control_reg: [u8; 12],
    sleep_status_reg: [u8; 12],
    hypervisor_vendor_identity: u64,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn firmware_ctrl(&self) -> Option<&firmware_acpi_control::Structure> {
        let firmware_ctrl: usize = self.firmware_ctrl as usize;
        let x_firmware_ctrl: usize = self.x_firmware_ctrl as usize;
        let firmware_ctrl: usize = cmp::max(firmware_ctrl, x_firmware_ctrl);
        (firmware_ctrl != 0).then(|| {
            let firmware_ctrl: *const firmware_acpi_control::Structure = firmware_ctrl as *const firmware_acpi_control::Structure;
            unsafe {
                &*firmware_ctrl
            }
        })
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let firmware_ctrl: Option<&firmware_acpi_control::Structure> = self.firmware_ctrl();
        let dsdt: u32 = self.dsdt;
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
        let flags: u32 = self.flags;
        let reset_reg: [u8; 12] = self.reset_reg;
        let reser_value: u8 = self.reser_value;
        let arm_boot_arch: u16 = self.arm_boot_arch;
        let fadt_minor_version: u8 = self.fadt_minor_version;
        let x_dsdt: u64 = self.x_dsdt;
        let x_pm1a_evt_blk: [u8; 12] = self.x_pm1a_evt_blk;
        let x_pm1b_evt_blk: [u8; 12] = self.x_pm1b_evt_blk;
        let x_pm1a_cnt_blk: [u8; 12] = self.x_pm1a_cnt_blk;
        let x_pm1b_cnt_blk: [u8; 12] = self.x_pm1b_cnt_blk;
        let x_pm2_cnt_blk: [u8; 12] = self.x_pm2_cnt_blk;
        let x_pm_tmr_blk: [u8; 12] = self.x_pm_tmr_blk;
        let x_gpe0_blk: [u8; 12] = self.x_gpe0_blk;
        let x_gpe1_blk: [u8; 12] = self.x_gpe1_blk;
        let sleep_control_reg: [u8; 12] = self.sleep_control_reg;
        let sleep_status_reg: [u8; 12] = self.sleep_status_reg;
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
            .field("x_dsdt", &x_dsdt)
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

