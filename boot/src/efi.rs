//! # UEFI
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)

mod allocator;
mod boot_services;
mod char16;
mod configuration;
mod event;
mod guid;
mod handle;
pub mod memory;
mod protocol;
mod runtime_services;
mod status;
mod system_table;
mod table_header;
mod time;
mod void;

pub use {
    boot_services::BootServices,
    char16::Char16,
    event::Event,
    guid::Guid,
    handle::Handle,
    runtime_services::RuntimeServices,
    status::Status,
    system_table::SystemTable,
    table_header::TableHeader,
    time::Time,
    void::{
        VOID,
        Void,
        null,
    },
    protocol::{
        simple_text,
        mp_services,
    },
};

