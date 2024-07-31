use {
    core::fmt,
    super::{
        MutexOp,
        NameString,
        Reader,
        SyncFlags,
    },
};

/// # DefMutex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct DefMutex {
    mutex_op: MutexOp,
    name_string: NameString,
    sync_flags: SyncFlags,
}

impl fmt::Debug for DefMutex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            mutex_op,
            name_string,
            sync_flags,
        } = self;
        formatter
            .debug_tuple("DefMutex")
            .field(mutex_op)
            .field(name_string)
            .field(sync_flags)
            .finish()
    }
}

impl From<&[u8]> for DefMutex {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (mutex_op, aml): (MutexOp, &[u8]) = MutexOp::read(aml);
        let (name_string, aml): (NameString, &[u8]) = NameString::read(aml);
        let (sync_flags, aml): (SyncFlags, &[u8]) = SyncFlags::read(aml);
        Self {
            mutex_op,
            name_string,
            sync_flags,
        }
    }
}

impl Reader<'_> for DefMutex {
    fn length(&self) -> usize {
        let Self {
            mutex_op,
            name_string,
            sync_flags,
        } = self;
        mutex_op.length()
        + name_string.length()
        + sync_flags.length()
    }

    fn matches(aml: &[u8]) -> bool {
        MutexOp::matches(aml)
    }
}

