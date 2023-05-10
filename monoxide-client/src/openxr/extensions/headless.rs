use crate::openxr::prelude::*;

pub const PROPERTIES: ExtensionProperties = ExtensionProperties {
    ty: ExtensionProperties::TYPE,
    next: std::ptr::null_mut(),
    extension_name: [0; 128],
    extension_version: 2,
};
