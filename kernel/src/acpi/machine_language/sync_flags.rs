use {
    bitfield_struct::bitfield,
    super::Reader,
};

/// # SyncFlags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[bitfield(u8)]
pub struct SyncFlags {
    #[bits(4)]
    sync_level: u8,
    #[bits(4, access = RO)]
    reserved0: u8,
}

impl From<&[u8]> for SyncFlags {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        (*aml.first().unwrap()).into()
    }
}

impl Reader<'_> for SyncFlags {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        aml
            .first()
            .is_some_and(|sync_flags| {
                let sync_flags: SyncFlags = (*sync_flags).into();
                sync_flags.reserved0() == 0
            })
    }
}

