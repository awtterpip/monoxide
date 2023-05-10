pub use super::prelude::*;

#[cfg(feature = "XR_MND_headless")]
pub mod headless;
#[cfg(feature = "XR_KHR_vulkan_enable2")]
pub mod vulkan_enable2;

use std::ffi::c_char;

/// Used to determine extensions that are available to an application 
/// # Docs
/// See [xrEnumerateInstanceExtensionProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateInstanceExtensionProperties)
#[openxr(xrEnumerateInstanceExtensionProperties)]
pub unsafe fn xr_enumerate_instance_extension_properties(
    layer_name: *const c_char,
    input_count: u32,
    output_count: &mut Option<u32>,
    items_ptr: *mut ExtensionProperties,
) -> XrResult {
    if layer_name.is_null() {
        let extensions = [
            #[cfg(feature = "XR_MND_headless")]
            headless::PROPERTIES,
            #[cfg(feature = "XR_KHR_vulkan_enable2")]
            vulkan_enable2::PROPERTIES,
        ];
        enumerate(input_count, output_count, items_ptr, &extensions)?;
        Ok(())
    } else {
        todo!("implement searching for extensions by api layer")
    }
}