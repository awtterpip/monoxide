use crate::client::openxr::{
    oxr::{Instance, Session, SessionCreateInfo},
    XrResult,
};
use proc_macros::openxr;

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSession
#[openxr(xrCreateSession)]
pub unsafe fn xr_create_session(
    _oxr_instance: Instance,
    _create_info: &SessionCreateInfo,
    _session: &mut Session,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySession
#[openxr(xrDestroySession)]
pub unsafe fn xr_destroy_session(_session: Session) -> Result<(), XrResult> {
    todo!()
}

