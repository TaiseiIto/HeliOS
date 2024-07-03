use {
    bitfield_struct::bitfield,
    super::super::super::DeliveryMode,
};

/// # Redirection Table Entry
/// ## References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.1.3 Redirection Table Entry 0 (RTE0)
#[bitfield(u64)]
pub struct Entry {
    vector: u8,
    #[bits(3)]
    delivery_mode: u8,
    destination_mode: bool,
    #[bits(access = RO)]
    delivery_status: bool,
    polarity: bool,
    remote_irr: bool,
    trigger_mode: bool,
    mask: bool,
    #[bits(31, access = RO)]
    reserved0: u32,
    extended_destination_id: u8,
    destination_id: u8,
}

impl Entry {
    pub fn is_enabled(&self) -> bool {
        !self.mask()
    }

    pub fn with_redirection(self, local_apic_id: u8, interrupt_number: u8) -> Self {
        self.with_vector(interrupt_number)
            .with_delivery_mode(DeliveryMode::Fixed.into())
            .with_destination_mode(false)
            .with_polarity(false)
            .with_remote_irr(false)
            .with_trigger_mode(false)
            .with_mask(false)
            .with_extended_destination_id(0)
            .with_destination_id(local_apic_id)
    }
}

