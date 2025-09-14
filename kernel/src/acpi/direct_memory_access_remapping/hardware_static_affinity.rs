/// # Remapping Hardware Static Affinity Structure
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.6 Remapping Hardware Static Affinity Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u16,
    length: u16,
    __: u32,
    #[allow(dead_code)]
    base_address: u64,
    #[allow(dead_code)]
    proximity_domain: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}
