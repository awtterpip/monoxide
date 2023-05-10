mod handle;
mod util;
mod system;
mod prelude {
    pub use proc_macros::*;
    pub use openxr_sys::Result as XrErr;
    pub type XrResult = Result<(), XrErr>;
    pub use crate::util::*;
    pub use crate::handle::Handle;
}

fn main() {
    println!("Hello, world!");
}
