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
    platform_class: u16,
    reserved0: u16,
    address_of_crb_control_area_or_fifo_base_address: u64,
    start_method: u32,
    start_method_specific_paremeters: StartMethodSpecificParameters,
    log_area_minimum_length: u32,
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
    interrupt: u32,
    flags: Flags,
    operation_flags: OperationFlags,
    attributes: Attributes,
    reserved0: u8,
    smc_hvc_function_id: u32,
}

#[bitfield(u8)]
struct Flags {
    interrupt_support: bool,
    hypervisor_call: bool,
    attribute_field_valid: bool,
    #[bits(5, access = RO)]
    reserved0: u8,
}

#[bitfield(u8)]
struct OperationFlags {
    tpm_idle_support: bool,
    #[bits(7, access = RO)]
    reserved0: u8,
}

#[bitfield(u8)]
struct Attributes {
    #[bits(2)]
    memory_type: u8,
    #[bits(6, access = RO)]
    reserved0: u8,
}

