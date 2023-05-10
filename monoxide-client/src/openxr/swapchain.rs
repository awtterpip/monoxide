pub use super::prelude::*;

/// # Docs
/// See [xrEnumerateSwapchainFormats](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateSwapchainFormats)
#[openxr(xrEnumerateSwapchainFormats)]
pub unsafe fn xr_enumerate_swapchain_formats(
    _session: Session,
    _format_capacity_input: u32,
    _format_count_output: &mut u32,
    _formats: &mut i64,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrCreateSwapchain](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSwapchain)
#[openxr(xrCreateSwapchain)]
pub unsafe fn xr_create_swapchain(
    _session: Session,
    _create_info: &SwapchainCreateInfo,
    _swapchain: &mut Swapchain,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrDestroySwapchain](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySwapchain)
#[openxr(xrDestroySwapchain)]
pub unsafe fn xr_destroy_swapchain(_swapchain: Swapchain) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrEnumerateSwapchainImages](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateSwapchainImages)
#[openxr(xrEnumerateSwapchainImages)]
pub unsafe fn xr_enumerate_swapchain_images(
    _swapchain: Swapchain,
    _image_capacity_input: u32,
    _image_count_output: &mut u32,
    _images: &mut SwapchainImageBaseHeader,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrAcquireSwapchainImage](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrAcquireSwapchainImage)
#[openxr(xrAcquireSwapchainImage)]
pub unsafe fn xr_acquire_swapchain_image(
    _swapchain: Swapchain,
    _acquire_info: &SwapchainImageAcquireInfo,
    _index: &mut u32,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrWaitSwapchainImage](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrWaitSwapchainImage)
#[openxr(xrWaitSwapchainImage)]
pub unsafe fn xr_wait_swapchain_image(
    _swapchain: Swapchain,
    _wait_info: &SwapchainImageWaitInfo,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrReleaseSwapchainImage](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrReleaseSwapchainImage)
#[openxr(xrReleaseSwapchainImage)]
pub unsafe fn xr_release_swapchain_image(
    _swapchain: Swapchain,
    _release_info: &SwapchainImageReleaseInfo,
) -> XrResult {
    todo!()
}