//! # UEFI
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)

mod handle;
mod status;

pub use handle::Handle;
pub use status::Status;
pub use status::ABORTED;

