/// # Remapping Hardware Static Affinity Structure
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.6 Remapping Hardware Static Affinity Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u16,
    length: u16,
    reserved0: u32,
    base_address: u64,
    proximity_domain: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

