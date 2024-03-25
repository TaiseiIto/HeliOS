use bitfield_struct::bitfield;

/// # Timer N Configuration and Capabilities Register
/// ## References
/// * [IA-PC HPET (High Precision Event Timers Specification)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 2.3.8 Timer N Configuration and Capabilities Register
#[bitfield(u64)]
pub struct Register {
    #[bits(access = RO)]
    reserved0: bool,
    tn_int_type_cnf: bool,
    tn_int_enb_cnf: bool,
    tn_type_cnf: bool,
    tn_per_int_cap: bool,
    tn_size_cap: bool,
    tn_val_set_cnf: bool,
    #[bits(access = RO)]
    reserved1: bool,
    tn_32mode_cnf: bool,
    #[bits(5)]
    tn_int_route_cnf: u8,
    tn_fsb_en_cnf: bool,
    tn_fsb_int_del_cap: bool,
    #[bits(access = RO)]
    reserved2: u16,
    tn_int_route_cap: u32,
}

