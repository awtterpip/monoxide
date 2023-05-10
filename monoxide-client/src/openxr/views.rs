pub use super::prelude::*;

/// # Docs
/// See [xrEnumerateViewConfigurations](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateViewConfigurations)
#[openxr(xrEnumerateViewConfigurations)]
pub unsafe fn xr_enumerate_view_configurations(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type_capacity_input: u32,
    _view_configuration_type_count_output: &mut u32,
    _view_configuration_types: &mut ViewConfigurationType,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrGetViewConfigurationProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetViewConfigurationProperties)
#[openxr(xrGetViewConfigurationProperties)]
pub unsafe fn xr_get_view_configuration_properties(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type: ViewConfigurationType,
    _configuration_properties: &mut ViewConfigurationProperties,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrEnumerateViewConfigurationViews](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateViewConfigurationViews)
#[openxr(xrEnumerateViewConfigurationViews)]
pub unsafe fn xr_enumerate_view_configuration_views(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type: ViewConfigurationType,
    _view_capacity_input: u32,
    _view_count_output: &mut Option<u32>,
    _views_ptr: *mut ViewConfigurationView,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrLocateViews](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrLocateViews)
#[openxr(xrLocateViews)]
pub unsafe fn xr_locate_views(
    _session: Session,
    _view_locate_info: &ViewLocateInfo,
    _view_state: &mut ViewState,
    _view_capacity_input: u32,
    _view_count_output: &mut u32,
    _views: &mut View,
) -> XrResult {
    todo!()
}