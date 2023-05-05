use crate::prelude::*;

/// # Docs
/// See [xrCreateSession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSession)
#[openxr(xrCreateSession)]
pub unsafe fn xr_create_session(
    _oxr_instance: Instance,
    _create_info: &SessionCreateInfo,
    _session: &mut Session,
) -> Result<(), XrResult> {
    todo!()
}

/// # Docs
/// See [xrDestroySession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySession)
#[openxr(xrDestroySession)]
pub unsafe fn xr_destroy_session(_session: Session) -> Result<(), XrResult> {
    todo!()
}
