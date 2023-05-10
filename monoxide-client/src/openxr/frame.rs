pub use super::prelude::*;

/// # Docs
/// See [xrWaitFrame](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrWaitFrame)
#[openxr(xrWaitFrame)]
pub unsafe fn xr_wait_frame(
    _session: Session,
    _frame_wait_info: &FrameWaitInfo,
    _frame_state: &mut FrameState,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrBeginFrame](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrBeginFrame)
#[openxr(xrBeginFrame)]
pub unsafe fn xr_begin_frame(
    _session: Session,
    _frame_begin_info: &FrameBeginInfo,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrEndFrame](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEndFrame)
#[openxr(xrEndFrame)]
pub unsafe fn xr_end_frame(
    _session: Session,
    _frame_end_info: &FrameEndInfo,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrEnumerateEnvironmentBlendModes](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateEnvironmentBlendModes)
#[openxr(xrEnumerateEnvironmentBlendModes)]
pub unsafe fn xr_enumerate_environment_blend_modes(
    _instance: Instance,
    _system_id: SystemId,
    _view_configuration_type: ViewConfigurationType,
    _environment_blend_mode_capacity_input: u32,
    _environment_blend_mode_count_output: &mut u32,
    _environment_blend_modes: &mut EnvironmentBlendMode,
) -> XrResult {
    todo!()
}
