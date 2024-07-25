use {
    core::fmt,
    super::{
        BuffPkgStrObj,
        IndexOp,
        IndexValue,
        Reader,
        Target,
    },
};

/// # DefIndex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefIndex {
    index_op: IndexOp,
    buff_pkg_str_obj: BuffPkgStrObj,
    index_value: IndexValue,
    target: Target,
}

impl fmt::Debug for DefIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            index_op,
            buff_pkg_str_obj,
            index_value,
            target,
        } = self;
        formatter
            .debug_tuple("DefIndex")
            .field(index_op)
            .field(buff_pkg_str_obj)
            .field(index_value)
            .field(target)
            .finish()
    }
}

impl From<&[u8]> for DefIndex {
    fn from(aml: &[u8]) -> Self {
        let (index_op, aml): (IndexOp, &[u8]) = IndexOp::read(aml);
        let (buff_pkg_str_obj, aml): (BuffPkgStrObj, &[u8]) = BuffPkgStrObj::read(aml);
        let (index_value, aml): (IndexValue, &[u8]) = IndexValue::read(aml);
        let (target, _aml): (Target, &[u8]) = Target::read(aml);
        Self {
            index_op,
            buff_pkg_str_obj,
            index_value,
            target,
        }
    }
}

impl Reader<'_> for DefIndex {
    fn length(&self) -> usize {
        let Self {
            index_op,
            buff_pkg_str_obj,
            index_value,
            target,
        } = self;
        index_op.length()
        + buff_pkg_str_obj.length()
        + index_value.length()
        + target.length()
    }

    fn matches(aml: &[u8]) -> bool {
        IndexOp::matches(aml)
    }
}

