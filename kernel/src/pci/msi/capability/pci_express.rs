pub mod capability;
pub mod device;

use {
    core::fmt,
    super::Header,
};

/// # PCI Express Capability Structure
/// ## Referneces
/// * [PCI Express Capability Structure](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html)
#[repr(packed)]
pub struct Structure {
    header: Header,
    capability: capability::Register,
    device_capabilities: device::capabilities::Register,
    device_control: u16,
    device_status: u16,
    link_capabilities: u32,
    link_control: u16,
    link_status: u16,
    slot_capabilities: u32,
    slot_control: u16,
    slot_status: u16,
    root_control: u16,
    root_capabilities: u16,
    root_status: u32,
    device_capabilities_2: u32,
    device_control_2: u16,
    device_status_2: u16,
    link_capabilities_2: u32,
    link_control_2: u16,
    link_status_2: u16,
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let capability_id: u8 = header.capability_id();
        let next_pointer: u8 = header.next_pointer();
        let capability: capability::Register = self.capability;
        let device_capabilities: device::capabilities::Register = self.device_capabilities;
        let device_control: u16 = self.device_control;
        let device_status: u16 = self.device_status;
        let link_capabilities: u32 = self.link_capabilities;
        let link_control: u16 = self.link_control;
        let link_status: u16 = self.link_status;
        let slot_capabilities: u32 = self.slot_capabilities;
        let slot_control: u16 = self.slot_control;
        let slot_status: u16 = self.slot_status;
        let root_control: u16 = self.root_control;
        let root_capabilities: u16 = self.root_capabilities;
        let root_status: u32 = self.root_status;
        let device_capabilities_2: u32 = self.device_capabilities_2;
        let device_control_2: u16 = self.device_control_2;
        let device_status_2: u16 = self.device_status_2;
        let link_capabilities_2: u32 = self.link_capabilities_2;
        let link_control_2: u16 = self.link_control_2;
        let link_status_2: u16 = self.link_status_2;
        formatter
            .debug_struct("Structure")
            .field("capability_id", &capability_id)
            .field("next_pointer", &next_pointer)
            .field("capability", &capability)
            .field("device_capabilities", &device_capabilities)
            .field("device_control", &device_control)
            .field("device_status", &device_status)
            .field("link_capabilities", &link_capabilities)
            .field("link_control", &link_control)
            .field("link_status", &link_status)
            .field("slot_capabilities", &slot_capabilities)
            .field("slot_control", &slot_control)
            .field("slot_status", &slot_status)
            .field("root_control", &root_control)
            .field("root_capabilities", &root_capabilities)
            .field("root_status", &root_status)
            .field("device_capabilities_2", &device_capabilities_2)
            .field("device_control_2", &device_control_2)
            .field("device_status_2", &device_status_2)
            .field("link_capabilities_2", &link_capabilities_2)
            .field("link_control_2", &link_control_2)
            .field("link_status_2", &link_status_2)
            .finish()
    }
}

