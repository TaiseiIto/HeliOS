//! # UEFI
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)

mod char16;
mod handle;
mod simple_text_input_protocol;
mod status;
mod system_table;
mod table_header;

pub use handle::Handle;
pub use simple_text_input_protocol::SimpleTextInputProtocol;
pub use status::Status;
pub use status::ABORTED;
pub use system_table::SystemTable;
pub use table_header::TableHeader;

