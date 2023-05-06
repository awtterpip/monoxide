use crate::openxr::prelude::*;

use std::ffi::c_char;
use std::ptr;

/// Used to determine extensions that are available to an application 
/// # Docs
/// See [xrEnumerateInstanceExtensionProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateInstanceExtensionProperties)
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