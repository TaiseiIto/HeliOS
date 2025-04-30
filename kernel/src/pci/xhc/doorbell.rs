/// # Doorbell Registers
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.6 Doorbell Registers
#[derive(Debug)]
#[repr(packed)]
pub struct Register {
    #[allow(dead_code)]
    db_target: u8,
    __: u16,
    #[allow(dead_code)]
    db_task_id: u16,
}

