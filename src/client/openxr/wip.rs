use crate::client::openxr::XrResult;
use openxr_sys::*;
use std::result::Result;
use proc_macros::openxr;

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySpace
#[openxr(xrDestroySpace)]
pub unsafe fn xr_destroy_space(_space: Space) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateSwapchainFormats
#[openxr(xrEnumerateSwapchainFormats)]
pub unsafe fn xr_enumerate_swapchain_formats(
    _session: Session,
    _format_capacity_input: u32,
    _format_count_output: &mut u32,
    _formats: &mut i64,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSwapchain
#[openxr(xrCreateSwapchain)]
pub unsafe fn xr_create_swapchain(
    _session: Session,
    _create_info: &SwapchainCreateInfo,
    _swapchain: &mut Swapchain,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySwapchain
#[openxr(xrDestroySwapchain)]
pub unsafe fn xr_destroy_swapchain(_swapchain: Swapchain) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateSwapchainImages
#[openxr(xrEnumerateSwapchainImages)]
pub unsafe fn xr_enumerate_swapchain_images(
    _swapchain: Swapchain,
    _image_capacity_input: u32,
    _image_count_output: &mut u32,
    _images: &mut SwapchainImageBaseHeader,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrAcquireSwapchainImage
#[openxr(xrAcquireSwapchainImage)]
pub unsafe fn xr_acquire_swapchain_image(
    _swapchain: Swapchain,
    _acquire_info: &SwapchainImageAcquireInfo,
    _index: &mut u32,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrWaitSwapchainImage
#[openxr(xrWaitSwapchainImage)]
pub unsafe fn xr_wait_swapchain_image(
    _swapchain: Swapchain,
    _wait_info: &SwapchainImageWaitInfo,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrReleaseSwapchainImage
#[openxr(xrReleaseSwapchainImage)]
pub unsafe fn xr_release_swapchain_image(
    _swapchain: Swapchain,
    _release_info: &SwapchainImageReleaseInfo,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrBeginSession
#[openxr(xrBeginSession)]
pub unsafe fn xr_begin_session(
    _session: Session,
    _begin_info: &SessionBeginInfo,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEndSession
#[openxr(xrEndSession)]
pub unsafe fn xr_end_session(_session: Session) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrRequestExitSession
#[openxr(xrRequestExitSession)]
pub unsafe fn xr_request_exit_session(_session: Session) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateReferenceSpaces
#[openxr(xrEnumerateReferenceSpaces)]
pub unsafe fn xr_enumerate_reference_spaces(
    _session: Session,
    _space_capacity_input: u32,
    _space_count_output: &mut u32,
    _spaces: &mut ReferenceSpaceType,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateReferenceSpace
#[openxr(xrCreateReferenceSpace)]
pub unsafe fn xr_create_reference_space(
    _session: Session,
    _create_info: &ReferenceSpaceCreateInfo,
    _space: &mut Space,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateActionSpace
#[openxr(xrCreateActionSpace)]
pub unsafe fn xr_create_action_space(
    _session: Session,
    _create_info: &ActionSpaceCreateInfo,
    _space: &mut Space,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrLocateSpace
#[openxr(xrLocateSpace)]
pub unsafe fn xr_locate_space(
    _space: Space,
    _base_space: Space,
    _time: Time,
    _location: &mut SpaceLocation,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrBeginFrame
#[openxr(xrBeginFrame)]
pub unsafe fn xr_begin_frame(
    _session: Session,
    _frame_begin_info: &FrameBeginInfo,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrLocateViews
#[openxr(xrLocateViews)]
pub unsafe fn xr_locate_views(
    _session: Session,
    _view_locate_info: &ViewLocateInfo,
    _view_state: &mut ViewState,
    _view_capacity_input: u32,
    _view_count_output: &mut u32,
    _views: &mut View,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEndFrame
#[openxr(xrEndFrame)]
pub unsafe fn xr_end_frame(
    _session: Session,
    _frame_end_info: &FrameEndInfo,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrWaitFrame
#[openxr(xrWaitFrame)]
pub unsafe fn xr_wait_frame(
    _session: Session,
    _frame_wait_info: &FrameWaitInfo,
    _frame_state: &mut FrameState,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPollEvent
#[openxr(xrPollEvent)]
pub unsafe fn xr_poll_event(
    _instance: Instance,
    _event_data: &mut EventDataBuffer,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetReferenceSpaceBoundsRect
#[openxr(xrGetReferenceSpaceBoundsRect)]
pub unsafe fn xr_get_reference_space_bounds_rect(
    _session: Session,
    _reference_space_type: ReferenceSpaceType,
    _bounds: &mut Extent2Df,
) -> Result<(), XrResult> {
    todo!()
}
