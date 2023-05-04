use crate::client::openxr::{
    oxr::{Instance, Session, SessionCreateInfo},
    XrResult,
};

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSession
#[no_mangle]
pub unsafe extern "system" fn xrCreateSession(
    _oxr_instance: Instance,
    _create_info: &SessionCreateInfo,
    _session: &mut Session,
) -> XrResult {
    wrap_oxr! {
        todo!()
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySession
#[no_mangle]
pub unsafe extern "system" fn xrDestroySession(_session: Session) -> XrResult {
    wrap_oxr! {
        todo!()
    }
}
