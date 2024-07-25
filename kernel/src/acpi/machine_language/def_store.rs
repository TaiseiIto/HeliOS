use {
    alloc::boxed::Box,
    core::fmt,
    super::{
        Reader,
        StoreOp,
        SuperName,
        TermArg,
    },
};

/// # DefStore
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefStore {
    store_op: StoreOp,
    term_arg: Box<TermArg>,
    super_name: SuperName,
}

impl fmt::Debug for DefStore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            store_op,
            term_arg,
            super_name,
        } = self;
        formatter
            .debug_tuple("DefStore")
            .field(store_op)
            .field(term_arg)
            .field(super_name)
            .finish()
    }
}

impl From<&[u8]> for DefStore {
    fn from(aml: &[u8]) -> Self {
        let (store_op, aml): (StoreOp, &[u8]) = StoreOp::read(aml);
        let (term_arg, aml): (TermArg, &[u8]) = TermArg::read(aml);
        let term_arg: Box<TermArg> = Box::new(term_arg);
        let (super_name, _aml): (SuperName, &[u8]) = SuperName::read(aml);
        Self {
            store_op,
            term_arg,
            super_name,
        }
    }
}

impl Reader<'_> for DefStore {
    fn length(&self) -> usize {
        let Self {
            store_op,
            term_arg,
            super_name,
        } = self;
        store_op.length()
        + term_arg.length()
        + super_name.length()
    }

    fn matches(aml: &[u8]) -> bool {
        StoreOp::matches(aml)
    }
}

