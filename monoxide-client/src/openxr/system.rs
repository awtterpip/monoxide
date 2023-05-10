pub use super::prelude::*;

/// # Docs
/// See [xrGetSystem](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystem)
#[openxr(xrGetSystem)]
pub unsafe fn xr_get_system(
    _instance: Instance,
    _get_info: &SystemGetInfo,
    _system_id: &mut SystemId,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrGetSystemProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystemProperties)
#[openxr(xrGetSystemProperties)]
pub unsafe fn xr_get_system_properties(
    _instance: Instance,
    _system_id: SystemId,
    _properties: &mut SystemProperties,
) -> XrResult {
    todo!()
}