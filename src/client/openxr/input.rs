use crate::client::openxr::XrResult;
use openxr_sys::{
    Action, ActionCreateInfo, ActionSet, ActionSetCreateInfo, ActionStateBoolean, ActionStateFloat,
    ActionStateGetInfo, ActionStatePose, ActionStateVector2f, ActionsSyncInfo,
    BoundSourcesForActionEnumerateInfo, HapticActionInfo, HapticBaseHeader,
    InputSourceLocalizedNameGetInfo, Instance, InteractionProfileState,
    InteractionProfileSuggestedBinding, Path, Session, SessionActionSetsAttachInfo,
};
use std::ffi::c_char;
use oxr_proc_macros::openxr;

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateActionSet
#[openxr(xrCreateActionSet)]
pub unsafe fn xr_create_action_set(
    _instance: Instance,
    _create_info: &ActionSetCreateInfo,
    _action_set: &mut ActionSet,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyActionSet
#[openxr(xrDestroyActionSet)]
pub unsafe fn xr_destroy_action_set(_action_set: ActionSet) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrApplyHapticFeedback
#[openxr(xrApplyHapticFeedback)]
pub unsafe fn xr_apply_haptic_feedback(
    _session: Session,
    _haptic_action_info: &HapticActionInfo,
    _haptic_feedback: &HapticBaseHeader,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrStopHapticFeedback
#[openxr(xrStopHapticFeedback)]
pub unsafe fn xr_stop_haptic_feedback(
    _session: Session,
    _haptic_action_info: &HapticActionInfo,
) -> Result<(), XrResult> {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateBoolean
#[no_mangle]
pub unsafe extern "system" fn xrGetActionStateBoolean(
    _session: Session,
    _get_info: &ActionStateGetInfo,
    _state: &mut ActionStateBoolean,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateFloat
#[no_mangle]
pub unsafe extern "system" fn xrGetActionStateFloat(
    _session: Session,
    _get_info: &ActionStateGetInfo,
    _state: &mut ActionStateFloat,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateVector2f
#[no_mangle]
pub unsafe extern "system" fn xrGetActionStateVector2f(
    _session: Session,
    _get_info: &ActionStateGetInfo,
    _state: &mut ActionStateVector2f,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStatePose
#[no_mangle]
pub unsafe extern "system" fn xrGetActionStatePose(
    _session: Session,
    _get_info: &ActionStateGetInfo,
    _state: &mut ActionStatePose,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

// impl Handle for Action {
//     type HandleType = ; = StardustAction;

//     fn raw(&self) -> u64 {
//         self.into_raw()
//     }
// }

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateAction
#[no_mangle]
pub unsafe extern "system" fn xrCreateAction(
    _action_set: ActionSet,
    _create_info: &ActionCreateInfo,
    _action: &mut Action,
) -> XrResult {
    wrap_oxr! {
        todo!()
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyAction
#[no_mangle]
pub unsafe extern "system" fn xrDestroyAction(_action: Action) -> XrResult {
    wrap_oxr! {
        todo!()
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSuggestInteractionProfileBindings
#[no_mangle]
pub unsafe extern "system" fn xrSuggestInteractionProfileBindings(
    _instance: Instance,
    _suggested_bindings: &InteractionProfileSuggestedBinding,
) -> XrResult {
    wrap_oxr! {
        todo!()
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrAttachSessionActionSets
#[no_mangle]
pub unsafe extern "system" fn xrAttachSessionActionSets(
    _session: Session,
    _attach_info: &SessionActionSetsAttachInfo,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetCurrentInteractionProfile
#[no_mangle]
pub unsafe extern "system" fn xrGetCurrentInteractionProfile(
    _session: Session,
    _top_level_user_path: Path,
    _interaction_profile: &mut InteractionProfileState,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSyncActions
#[no_mangle]
pub unsafe extern "system" fn xrSyncActions(
    _session: Session,
    _sync_info: &ActionsSyncInfo,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateBoundSourcesForAction
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateBoundSourcesForAction(
    _session: Session,
    _enumerate_info: &BoundSourcesForActionEnumerateInfo,
    _source_capacity_input: u32,
    _source_count_output: &mut u32,
    _sources: &mut Path,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetInputSourceLocalizedName
#[no_mangle]
pub unsafe extern "system" fn xrGetInputSourceLocalizedName(
    _session: Session,
    _get_info: &InputSourceLocalizedNameGetInfo,
    _buffer_capacity_input: u32,
    _buffer_count_output: &mut u32,
    _buffer: &mut c_char,
) -> XrResult {
    wrap_oxr! {
        todo!();
    }
}
