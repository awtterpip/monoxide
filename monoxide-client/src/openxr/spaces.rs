pub use super::prelude::*;

/// # Docs
/// See [xrGetReferenceSpaceBoundsRect](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetReferenceSpaceBoundsRect)
#[openxr(xrGetReferenceSpaceBoundsRect)]
pub unsafe fn xr_get_reference_space_bounds_rect(
    _session: Session,
    _reference_space_type: ReferenceSpaceType,
    _bounds: &mut Extent2Df,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrEnumerateReferenceSpaces](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateReferenceSpaces)
#[openxr(xrEnumerateReferenceSpaces)]
pub unsafe fn xr_enumerate_reference_spaces(
    _session: Session,
    _space_capacity_input: u32,
    _space_count_output: &mut u32,
    _spaces: &mut ReferenceSpaceType,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrCreateReferenceSpace](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateReferenceSpace)
#[openxr(xrCreateReferenceSpace)]
pub unsafe fn xr_create_reference_space(
    _session: Session,
    _create_info: &ReferenceSpaceCreateInfo,
    _space: &mut Space,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrCreateActionSpace](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateActionSpace)
#[openxr(xrCreateActionSpace)]
pub unsafe fn xr_create_action_space(
    _session: Session,
    _create_info: &ActionSpaceCreateInfo,
    _space: &mut Space,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrDestroySpace](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySpace)
#[openxr(xrDestroySpace)]
pub unsafe fn xr_destroy_space(_space: Space) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrLocateSpace](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrLocateSpace)
#[openxr(xrLocateSpace)]
pub unsafe fn xr_locate_space(
    _space: Space,
    _base_space: Space,
    _time: Time,
    _location: &mut SpaceLocation,
) -> XrResult {
    todo!()
}