use {
    alloc::boxed::Box,
    super::{
        StoreOp,
        SuperName,
        TermArg,
    },
};

/// # DefStore
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefStore(
    StoreOp,
    Box<TermArg>,
    SuperName,
);

