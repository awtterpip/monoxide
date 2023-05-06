use crate::prelude::*;
use openxr_sys::{Instance, InstanceCreateInfo};

#[handle(Instance)]
pub struct XrInstance {

}

impl XrInstance {
    pub fn new(info: InstanceCreateInfo) -> Result<Self, XrResult> {
        todo!()
    }
}

