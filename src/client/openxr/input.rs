use crate::client::openxr::XrResult;
use openxr_sys::{
    Action, ActionCreateInfo, ActionSet, ActionSetCreateInfo, ActionStateBoolean, ActionStateFloat,
    ActionStateGetInfo, ActionStatePose, ActionStateVector2f, ActionsSyncInfo,
    BoundSourcesForActionEnumerateInfo, HapticActionInfo, HapticBaseHeader,
    InputSourceLocalizedNameGetInfo, Instance, InteractionProfileState,
    InteractionProfileSuggestedBinding, Path, Session, SessionActionSetsAttachInfo,
};
use std::ffi::c_char;
use proc_macros::openxr;

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
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateBoolean
#[openxr(xrGetActionStateBoolean)]
pub unsafe fn xr_get_action_state_boolean(
    _session: Session,
    _get_info: &ActionStateGetInfo,
    _state: &mut ActionStateBoolean,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateFloat
#[openxr(xrGetActionStateFloat)]
pub unsafe fn xr_get_action_state_float(
    _session: Session,
    _get_info: &ActionStateGetInfo,
    _state: &mut ActionStateFloat,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateVector2f
#[openxr(xrGetActionStateVector2f)]
pub unsafe fn xr_get_action_state_vector_2f(
    _session: Session,
    _get_info: &ActionStateGetInfo,
    _state: &mut ActionStateVector2f,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStatePose
#[openxr(xrGetActionStatePose)]
pub unsafe fn xr_get_action_state_pose(
    _session: Session,
    _get_info: &ActionStateGetInfo,
    _state: &mut ActionStatePose,
) -> Result<(), XrResult> {
    todo!()
}

// impl Handle for Action {
//     type HandleType = ; = StardustAction;

//     fn raw(&self) -> u64 {
//         self.into_raw()
//     }
// }

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateAction([a-z](?![A-Z]))*(([a-z])([A-Z0-9])([a-z](?![A-Z]))*)+
#[openxr(xrCreateAction)]
pub unsafe fn xr_create_action(
    _action_set: ActionSet,
    _create_info: &ActionCreateInfo,
    _action: &mut Action,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyAction
#[openxr(xrDestroyAction)]
pub unsafe fn xr_destroy_action(_action: Action) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSuggestInteractionProfileBindings
#[openxr(xrSuggestInteractionProfileBindings)]
pub unsafe fn xr_suggest_interaction_profile_bindings(
    _instance: Instance,
    _suggested_bindings: &InteractionProfileSuggestedBinding,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrAttachSessionActionSets
#[openxr(xrAttachSessionActionSets)]
pub unsafe fn xr_attach_session_action_sets(
    _session: Session,
    _attach_info: &SessionActionSetsAttachInfo,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetCurrentInteractionProfile
#[openxr(xrGetCurrentInteractionProfile)]
pub unsafe fn xr_get_current_interaction_profile(
    _session: Session,
    _top_level_user_path: Path,
    _interaction_profile: &mut InteractionProfileState,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSyncActions
#[openxr(xrSyncActions)]
pub unsafe fn xr_sync_actions(
    _session: Session,
    _sync_info: &ActionsSyncInfo,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateBoundSourcesForAction
#[openxr(xrEnumerateBoundSourcesForAction)]
pub unsafe fn xr_enumerate_bound_sources_for_action(
    _session: Session,
    _enumerate_info: &BoundSourcesForActionEnumerateInfo,
    _source_capacity_input: u32,
    _source_count_output: &mut u32,
    _sources: &mut Path,
) -> Result<(), XrResult> {
    todo!()
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetInputSourceLocalizedName
#[openxr(xrGetInputSourceLocalizedName)]
pub unsafe fn xr_get_input_source_localized_name(
    _session: Session,
    _get_info: &InputSourceLocalizedNameGetInfo,
    _buffer_capacity_input: u32,
    _buffer_count_output: &mut u32,
    _buffer: &mut c_char,
) -> Result<(), XrResult> {
    todo!()
}
