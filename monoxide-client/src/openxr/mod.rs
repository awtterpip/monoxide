mod instance;
mod system;
mod string;
mod spaces;
mod views;
mod session;
mod swapchain;
mod frame;
mod input;
mod extensions;
mod prelude {
    pub use crate::prelude::*;
    pub use openxr_sys::*;
    pub use openxr_sys::platform::*;
    pub use openxr_sys::pfn::*;
    pub use openxr_sys::loader::*;
    pub use std::result::Result;
    pub use std::ffi::c_char;
}

use std::mem::size_of;

pub use prelude::*;

/// # Docs
/// [xrNegotiateLoaderRuntimeInterface](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrNegotiateLoaderRuntimeInterface)
#[openxr(xrNegotiateLoaderRuntimeInterface)]
pub fn xr_negotiate_loader_runtime_interface(
    loader_info: &XrNegotiateLoaderInfo,
    runtime_request: &mut XrNegotiateRuntimeRequest,
) -> XrResult {
    if loader_info.ty != XrNegotiateLoaderInfo::TYPE
        || loader_info.struct_version != XrNegotiateLoaderInfo::VERSION
        || loader_info.struct_size != size_of::<XrNegotiateLoaderInfo>()
    {
        return Err(XrErr::ERROR_INITIALIZATION_FAILED);
    }
    if runtime_request.ty != XrNegotiateRuntimeRequest::TYPE
        || runtime_request.struct_version != XrNegotiateRuntimeRequest::VERSION
        || loader_info.struct_size != size_of::<XrNegotiateRuntimeRequest>()
    {
        return Err(XrErr::ERROR_INITIALIZATION_FAILED);
    }

    if CURRENT_API_VERSION > loader_info.max_api_version
        || CURRENT_API_VERSION < loader_info.min_api_version
    {
        eprintln!(
            "OpenXR Runtime doesn't support major version {} < {} < {}",
            loader_info.max_api_version, CURRENT_API_VERSION, loader_info.min_api_version
        );
        return Err(XrErr::ERROR_INITIALIZATION_FAILED);
    }

    runtime_request.runtime_interface_version = CURRENT_LOADER_RUNTIME_VERSION;
    runtime_request.runtime_api_version = CURRENT_API_VERSION;
    runtime_request.get_instance_proc_addr = Some(xrGetInstanceProcAddr);
        //Some(unsafe { transmute(xrGetInstanceProcAddr as usize) });

    Ok(())
}

/// # Docs
/// [xrGetInstanceProcAddr](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetInstanceProcAddr)
#[openxr(xrGetInstanceProcAddr)]
pub unsafe fn xr_get_instance_proc_addr(
    _instance: Instance,
    _name: *const c_char,
    _function: *mut Option<VoidFunction>,
) -> XrResult {
    todo!()
}

/// # Docs
/// [xrEnumerateApiLayerProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateApiLayerProperties)
#[openxr(xrEnumerateApiLayerProperties)]
pub unsafe fn xr_enumerate_api_layer_properties(
    property_capacity_input: u32,
    property_count_output: &mut Option<u32>,
    properties: *mut ApiLayerProperties,
) -> XrResult {
        let api_layers = [];
        enumerate(
            property_capacity_input,
            property_count_output,
            properties,
            &api_layers,
        )?;
        Ok(())
}