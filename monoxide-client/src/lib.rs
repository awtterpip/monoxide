#[macro_use]
pub mod handle;
pub mod openxr;
pub mod instance;

pub(crate) mod prelude {
    pub use openxr_sys::Result as XrResult;
    pub use proc_macros::*;
    pub use crate::handle::Handle;
}