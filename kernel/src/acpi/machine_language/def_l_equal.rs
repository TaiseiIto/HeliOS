use {
    alloc::vec::Vec,
    core::fmt,
    super::{
        LEqualOp,
        Operand,
        Reader,
    },
};

/// # DefLEqual
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefLEqual(
    LEqualOp,
    [Operand; 2],
);

impl fmt::Debug for DefLEqual {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("DefLEqual");
        let Self(field0, field1) = self;
        debug_tuple.field(field0);
        field1
            .as_slice()
            .iter()
            .for_each(|element| {
                debug_tuple.field(element);
            });
        debug_tuple.finish()
    }
}

impl From<&[u8]> for DefLEqual {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (field0, aml): (LEqualOp, &[u8]) = LEqualOp::read(aml);
        let (elements, aml): (Vec<Operand>, &[u8]) = (0..2)
            .fold((Vec::new(), aml), |(mut elements, aml), _| {
                let (element, aml): (Operand, &[u8]) = Operand::read(aml);
                elements.push(element);
                (elements, aml)
            });
        let field1: [Operand; 2] = elements
            .try_into()
            .unwrap();
        Self(field0, field1)
    }
}

impl Reader<'_> for DefLEqual {
    fn length(&self) -> usize {
        let Self(field0, field1) = self;
        field0.length() + field1
            .as_slice()
            .iter()
            .map(|element| element.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        LEqualOp::matches(aml)
    }
}

