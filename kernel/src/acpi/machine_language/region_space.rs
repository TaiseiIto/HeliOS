use {
    core::fmt,
    super::Reader,
};

/// # RegionSpace
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub enum RegionSpace {
    SystemMemory,
    SystemIo,
    PciConfig,
    EmbeddedControl,
    SmBus,
    SystemCmos,
    PciBarTarget,
    Ipmi,
    GeneralPurposeIo,
    GenericSerialBus,
    Pcc,
    OemDefined(u8),
}

impl fmt::Debug for RegionSpace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SystemMemory => write!(formatter, "RegionSpace::"),
            Self::SystemIo => write!(formatter, "RegionSpace::"),
            Self::PciConfig => write!(formatter, "RegionSpace::"),
            Self::EmbeddedControl => write!(formatter, "RegionSpace::"),
            Self::SmBus => write!(formatter, "RegionSpace::"),
            Self::SystemCmos => write!(formatter, "RegionSpace::"),
            Self::PciBarTarget => write!(formatter, "RegionSpace::"),
            Self::Ipmi => write!(formatter, "RegionSpace::"),
            Self::GeneralPurposeIo => write!(formatter, "RegionSpace::"),
            Self::GenericSerialBus => write!(formatter, "RegionSpace::"),
            Self::Pcc => write!(formatter, "RegionSpace::"),
            Self::OemDefined(oem_defined) => formatter
                .debug_tuple("RegionSpace::OemDefined")
                .field(oem_defined)
                .finish(),
        }
    }
}

impl From<u8> for RegionSpace {
    fn from(region_space: u8) -> Self {
        match region_space {
            0x00 => Self::SystemMemory,
            0x01 => Self::SystemIo,
            0x02 => Self::PciConfig,
            0x03 => Self::EmbeddedControl,
            0x04 => Self::SmBus,
            0x05 => Self::SystemCmos,
            0x06 => Self::PciBarTarget,
            0x07 => Self::Ipmi,
            0x08 => Self::GeneralPurposeIo,
            0x09 => Self::GenericSerialBus,
            0x0a => Self::Pcc,
            region_space => Self::OemDefined(region_space),
        }
    }
}

impl From<&[u8]> for RegionSpace {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        (*aml.first().unwrap()).into()
    }
}

impl Reader<'_> for RegionSpace {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        true
    }
}

