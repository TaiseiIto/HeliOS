/// # HSCPARAMS3
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.3.5 Structural Parameters 3 (HCSPARAMS3)
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Register {
    #[allow(dead_code)]
    u1_device_exit_latency: u8,
    #[allow(dead_code)]
    u2_device_exit_latency: u8,
    __: u16,
}
