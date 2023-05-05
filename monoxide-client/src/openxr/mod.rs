#[macro_use]
pub mod util;
pub mod extensions;
pub mod input;
pub mod instance;
pub mod session;
pub mod system;
pub mod wip;
pub mod string;

pub use extensions::*;
pub use input::*;
pub use instance::*;
pub use session::*;
pub use system::*;
pub use wip::*;
pub use string::*;

mod prelude {
    pub use crate::prelude::*;
    pub use openxr_sys::*;
    pub use std::result::Result;
    pub use crate::openxr::util::*;
}
use std::mem::size_of;

use prelude::*;
use openxr_sys::loader::*;
use std::ffi::c_char;
use pfn::VoidFunction;

/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrNegotiateLoaderRuntimeInterface
#[openxr(xrNegotiateLoaderRuntimeInterface)]
pub fn xr_negotiate_loader_runtime_interface(
    loader_info: &XrNegotiateLoaderInfo,
    runtime_request: &mut XrNegotiateRuntimeRequest,
) -> Result<(), XrResult> {
    if loader_info.ty != XrNegotiateLoaderInfo::TYPE
        || loader_info.struct_version != XrNegotiateLoaderInfo::VERSION
        || loader_info.struct_size != size_of::<XrNegotiateLoaderInfo>()
    {
        return Err(XrResult::ERROR_INITIALIZATION_FAILED);
    }
    if runtime_request.ty != XrNegotiateRuntimeRequest::TYPE
        || runtime_request.struct_version != XrNegotiateRuntimeRequest::VERSION
        || loader_info.struct_size != size_of::<XrNegotiateRuntimeRequest>()
    {
        return Err(XrResult::ERROR_INITIALIZATION_FAILED);
    }

    if CURRENT_API_VERSION > loader_info.max_api_version
        || CURRENT_API_VERSION < loader_info.min_api_version
    {
        eprintln!(
            "OpenXR Runtime doesn't support major version {} < {} < {}",
            loader_info.max_api_version, CURRENT_API_VERSION, loader_info.min_api_version
        );
        return Err(XrResult::ERROR_INITIALIZATION_FAILED);
    }

    runtime_request.runtime_interface_version = CURRENT_LOADER_RUNTIME_VERSION;
    runtime_request.runtime_api_version = CURRENT_API_VERSION;
    runtime_request.get_instance_proc_addr = Some(xrGetInstanceProcAddr);
        //Some(unsafe { transmute(xrGetInstanceProcAddr as usize) });

    Ok(())
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrGetInstanceProcAddr
#[openxr(xrGetInstanceProcAddr)]
pub unsafe fn xr_get_instance_proc_addr(
    instance: Instance,
    name: *const c_char,
    function: *mut Option<VoidFunction>,
) -> Result<(), XrResult> {
        *function = Some(get_instance_proc_addr(instance, str_from_const_char(name)?)?);
        Ok(())
}

oxr_fns!{
    get_instance_proc_addr 
    [
        xrEnumerateInstanceExtensionProperties,
        xrEnumerateApiLayerProperties,
        xrCreateInstance,
    ]
    [
        xrNegotiateLoaderRuntimeInterface,
        xrGetInstanceProcAddr,
        xrEnumerateApiLayerProperties,
        xrCreateActionSet,
        xrDestroyActionSet,
        xrApplyHapticFeedback,
        xrStopHapticFeedback,
        xrGetActionStateBoolean,
        xrGetActionStateFloat,
        xrGetActionStateVector2f,
        xrGetActionStatePose,
        xrCreateAction,
        xrDestroyAction,
        xrSuggestInteractionProfileBindings,
        xrAttachSessionActionSets,
        xrGetCurrentInteractionProfile,
        xrSyncActions,
        xrEnumerateBoundSourcesForAction,
        xrGetInputSourceLocalizedName,
        xrCreateInstance,
        xrDestroyInstance,
        xrGetInstanceProperties,
        xrCreateSession,
        xrDestroySession,
        xrResultToString,
        xrStructureTypeToString,
        xrStringToPath,
        xrPathToString,
        xrGetSystem,
        xrGetSystemProperties,
        xrEnumerateViewConfigurations,
        xrGetViewConfigurationProperties,
        xrEnumerateViewConfigurationViews,
        xrEnumerateEnvironmentBlendModes,
        xrDestroySpace,
        xrEnumerateSwapchainFormats,
        xrCreateSwapchain,
        xrDestroySwapchain,
        xrEnumerateSwapchainImages,
        xrAcquireSwapchainImage,
        xrWaitSwapchainImage,
        xrReleaseSwapchainImage,
        xrBeginSession,
        xrEndSession,
        xrRequestExitSession,
        xrEnumerateReferenceSpaces,
        xrCreateReferenceSpace,
        xrCreateActionSpace,
        xrLocateSpace,
        xrBeginFrame,
        xrLocateViews,
        xrEndFrame,
        xrWaitFrame,
        xrPollEvent,
        xrGetReferenceSpaceBoundsRect,

    ]
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateApiLayerProperties
#[openxr(xrEnumerateApiLayerProperties)]
pub unsafe fn xr_enumerate_api_layer_properties(
    property_capacity_input: u32,
    property_count_output: &mut Option<u32>,
    properties: *mut ApiLayerProperties,
) -> Result<(), XrResult> {
        let api_layers = [];
        enumerate(
            property_capacity_input,
            property_count_output,
            properties,
            &api_layers,
        )?;
        Ok(())
}