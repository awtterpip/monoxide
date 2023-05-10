pub use super::prelude::*;

/// # Docs
/// See [xrCreateSession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSession)
#[openxr(xrCreateSession)]
pub unsafe fn xr_create_session(
    _oxr_instance: Instance,
    _create_info: &SessionCreateInfo,
    _session: &mut Session,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrDestroySession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySession)
#[openxr(xrDestroySession)]
pub unsafe fn xr_destroy_session(_session: Session) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrBeginSession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrBeginSession)
#[openxr(xrBeginSession)]
pub unsafe fn xr_begin_session(
    _session: Session,
    _begin_info: &SessionBeginInfo,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrEndSession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEndSession)
#[openxr(xrEndSession)]
pub unsafe fn xr_end_session(_session: Session) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrRequestExitSession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrRequestExitSession)
#[openxr(xrRequestExitSession)]
pub unsafe fn xr_request_exit_session(_session: Session) -> XrResult {
    todo!()
}