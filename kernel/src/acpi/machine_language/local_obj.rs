/// # Local Objects Encoding
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.2 Local Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[encoding_value_min = 0x60]
#[encoding_value_max = 0x67]
pub struct LocalObj(u8);

