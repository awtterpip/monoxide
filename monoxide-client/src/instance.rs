use crate::prelude::*;
use openxr_sys::{Instance, InstanceCreateInfo};
use tokio::runtime::Runtime;

#[handle(Instance)]
pub struct XrInstance {
    pub extensions: Vec<String>,
    pub runtime: Runtime,
}

impl XrInstance {
    pub fn new(info: InstanceCreateInfo) -> Result<Self, XrResult> {
        let mut extensions = vec![];
        for str in str_slice_from_const_arr(info.enabled_extension_names, info.enabled_extension_count as usize) {
            let str = str_from_const_char(*str)?;
            extensions.push(String::from(str));
        }
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .map_err(|_| XrResult::ERROR_RUNTIME_UNAVAILABLE)?;
        Ok(Self {
            extensions,
            runtime,
        })
    }
}