use crate::client::openxr::XrResult;
use openxr_sys::{
    EnvironmentBlendMode, Instance, SystemGetInfo, SystemId, SystemProperties,
    ViewConfigurationProperties, ViewConfigurationType, ViewConfigurationView,
};

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystem
#[no_mangle]
pub extern "system" fn xrGetSystem(
    _instance: Instance,
    _get_info: &SystemGetInfo,
    _system_id: &mut SystemId,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystemProperties
#[no_mangle]
pub unsafe extern "system" fn xrGetSystemProperties(
    _instance: Instance,
    _system_id: SystemId,
    _properties: &mut SystemProperties,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateViewConfigurations
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateViewConfigurations(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type_capacity_input: u32,
    _view_configuration_type_count_output: &mut u32,
    _view_configuration_types: &mut ViewConfigurationType,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetViewConfigurationProperties
#[no_mangle]
pub unsafe extern "system" fn xrGetViewConfigurationProperties(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type: ViewConfigurationType,
    _configuration_properties: &mut ViewConfigurationProperties,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateViewConfigurationViews
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateViewConfigurationViews(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type: ViewConfigurationType,
    _view_capacity_input: u32,
    _view_count_output: &mut Option<u32>,
    _views_ptr: *mut ViewConfigurationView,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateEnvironmentBlendModes
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateEnvironmentBlendModes(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type: ViewConfigurationType,
    _environment_blend_mode_capacity_input: u32,
    _environment_blend_mode_count_output: &mut u32,
    _environment_blend_modes: &mut EnvironmentBlendMode,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}
