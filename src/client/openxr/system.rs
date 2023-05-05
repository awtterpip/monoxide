use crate::client::openxr::XrResult;
use openxr_sys::{
    EnvironmentBlendMode, Instance, SystemGetInfo, SystemId, SystemProperties,
    ViewConfigurationProperties, ViewConfigurationType, ViewConfigurationView,
};

use oxr_proc_macros::openxr;

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystem
#[openxr(xrGetSystem)]
pub unsafe fn xr_get_system(
    _instance: Instance,
    _get_info: &SystemGetInfo,
    _system_id: &mut SystemId,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystemProperties
#[openxr(xrGetSystemProperties)]
pub unsafe fn xr_get_system_properties(
    _instance: Instance,
    _system_id: SystemId,
    _properties: &mut SystemProperties,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateViewConfigurations
#[openxr(xrEnumerateViewConfigurations)]
pub unsafe fn xr_enumerate_view_configurations(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type_capacity_input: u32,
    _view_configuration_type_count_output: &mut u32,
    _view_configuration_types: &mut ViewConfigurationType,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetViewConfigurationProperties
#[openxr(xrGetViewConfigurationProperties)]
pub unsafe fn xr_get_view_configuration_properties(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type: ViewConfigurationType,
    _configuration_properties: &mut ViewConfigurationProperties,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateViewConfigurationViews
#[openxr(xrEnumerateViewConfigurationViews)]
pub unsafe fn xr_enumerate_view_configuration_views(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type: ViewConfigurationType,
    _view_capacity_input: u32,
    _view_count_output: &mut Option<u32>,
    _views_ptr: *mut ViewConfigurationView,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateEnvironmentBlendModes
#[openxr(xrEnumerateEnvironmentBlendModes)]
pub unsafe fn xr_enumerate_environment_blend_modes(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type: ViewConfigurationType,
    _environment_blend_mode_capacity_input: u32,
    _environment_blend_mode_count_output: &mut u32,
    _environment_blend_modes: &mut EnvironmentBlendMode,
) -> Result<(), XrResult> {
    todo!()
}
