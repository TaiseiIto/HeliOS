/// # ERSTSZ
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.5.2.3.1 Event Ring Segment Table Size Register (ERSTSZ)
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Register {
    #[allow(dead_code)]
    event_ring_segment_table_size: u16,
    __: u16,
}

