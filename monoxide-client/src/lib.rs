#[macro_use]
pub mod util;
pub mod extensions;
pub mod input;
pub mod instance;
pub mod session;
pub mod system;
pub mod wip;
pub mod string;

pub(crate) mod prelude {
    pub use openxr_sys::Result as XrResult;
    pub use openxr_sys::*;
    pub use std::result::Result;
    pub use proc_macros::openxr;
    pub use crate::util::*;
}

