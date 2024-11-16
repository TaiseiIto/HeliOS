use {
    bitfield_struct::bitfield,
    super::system_description,
};

/// # Trusted Platform Module 2.0 Table
/// ## References
/// * [TCG ACPI Specification](https://trustedcomputinggroup.org/wp-content/uploads/TCG_ACPIGeneralSpec_v1p3_r8_pub.pdf) 8.3 ACPI Table for TPM 2.0
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    #[allow(dead_code)]
    platform_class: u16,
    __: u16,
    #[allow(dead_code)]
    address_of_crb_control_area_or_fifo_base_address: u64,
    #[allow(dead_code)]
    start_method: u32,
    #[allow(dead_code)]
    start_method_specific_paremeters: StartMethodSpecificParameters,
    #[allow(dead_code)]
    log_area_minimum_length: u32,
    #[allow(dead_code)]
    log_area_start_address: u64,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}

/// # Start Method Specific Parameters for Arm SMC Start Method
/// ## References
/// * [TCG ACPI Specification](https://trustedcomputinggroup.org/wp-content/uploads/TCG_ACPIGeneralSpec_v1p3_r8_pub.pdf) 8.3.1 Start Method Specific Parameters for Arm SMC Start Method
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
struct StartMethodSpecificParameters {
    #[allow(dead_code)]
    interrupt: u32,
    #[allow(dead_code)]
    flags: Flags,
    #[allow(dead_code)]
    operation_flags: OperationFlags,
    #[allow(dead_code)]
    attributes: Attributes,
    __: u8,
    #[allow(dead_code)]
    smc_hvc_function_id: u32,
}

#[bitfield(u8)]
struct Flags {
    interrupt_support: bool,
    hypervisor_call: bool,
    attribute_field_valid: bool,
    #[bits(5)]
    __: u8,
}

#[bitfield(u8)]
struct OperationFlags {
    tpm_idle_support: bool,
    #[bits(7)]
    __: u8,
}

#[bitfield(u8)]
struct Attributes {
    #[bits(2)]
    memory_type: u8,
    #[bits(6)]
    __: u8,
}

