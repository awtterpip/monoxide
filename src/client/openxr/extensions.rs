use openxr_sys::MND_HEADLESS_EXTENSION_NAME;
use proc_macros::openxr;

use crate::client::openxr::{
    oxr::ExtensionProperties,
    util::{copy_str_to_buffer, enumerate},
    XrResult,
};
use std::{ffi::c_char, ptr};

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateInstanceExtensionProperties
#[openxr(xrEnumerateInstanceExtensionProperties)]
pub unsafe fn xr_enumerate_instance_extension_properties(
    _layer_name: *const c_char,
    input_count: u32,
    output_count: &mut Option<u32>,
    items_ptr: *mut ExtensionProperties,
) -> Result<(), XrResult> {
    let mut headless = ExtensionProperties {
        ty: ExtensionProperties::TYPE,
        next: ptr::null_mut(),
        extension_name: [0; 128],
        extension_version: 2,
    };
    copy_str_to_buffer(
        &String::from_utf8(MND_HEADLESS_EXTENSION_NAME.to_vec()).unwrap(),
        &mut headless.extension_name,
    );
    let extensions = [headless];
    enumerate(input_count, output_count, items_ptr, &extensions)?;
    Ok(())
}
