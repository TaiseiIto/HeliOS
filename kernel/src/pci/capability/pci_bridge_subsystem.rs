use {
    core::fmt,
    super::Header,
};

/// # Pci Bridge Subsystem Vendor ID
/// ## References
/// * [Reference: PCI Configuration Space](http://arbor.mindshare.com/arbor/refview?pane=index)
/// * [Populate subsystem vendor and device IDs for PCI-Bridges](https://patchwork.kernel.org/project/linux-pci/patch/1254843919-13809-1-git-send-email-gabe.black@ni.com/)
#[repr(packed)]
pub struct Structure {
    header: Header,
    __: u16,
    vendor_id: u16,
    device_id: u16,
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let capability_id: u8 = header.capability_id();
        let next_pointer: u8 = header.next_pointer();
        let vendor_id: u16 = self.vendor_id;
        let device_id: u16 = self.device_id;
        formatter
            .debug_struct("Structure")
            .field("capability_id", &capability_id)
            .field("next_pointer", &next_pointer)
            .field("vendor_id", &vendor_id)
            .field("device_id", &device_id)
            .finish()
    }
}

