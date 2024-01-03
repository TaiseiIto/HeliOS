//! # UEFI
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)

mod boot_services;
mod char16;
mod event;
mod guid;
mod handle;
mod runtime_services;
mod simple_text_input_protocol;
mod simple_text_output_protocol;
mod status;
mod system_table;
mod table_header;

pub use boot_services::MemoryDescriptor;
pub use char16::Char16;
pub use event::Event;
pub use guid::Guid;
pub use handle::Handle;
pub use runtime_services::RuntimeServices;
pub use simple_text_input_protocol::SimpleTextInputProtocol;
pub use simple_text_output_protocol::SimpleTextOutputProtocol;
pub use status::Status;
pub use status::ABORTED;
pub use system_table::SystemTable;
pub use table_header::TableHeader;

