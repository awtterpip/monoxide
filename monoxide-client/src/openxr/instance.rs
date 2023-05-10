use crate::instance::XrInstance;

pub use super::prelude::*;

/// # Docs
/// See [xrCreateInstance](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateInstance)
#[openxr(xrCreateInstance)]
pub fn xr_create_instance(info: &InstanceCreateInfo, instance: &mut Instance) -> XrResult {
    *instance = Instance::create(XrInstance::new(*info)?);
    Ok(())
}

/// # Docs
/// See [xrDestroyInstance](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyInstance)
#[openxr(xrDestroyInstance)]
pub unsafe fn xr_destroy_instance(_instance: Instance) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrGetInstanceProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetInstanceProperties)
#[openxr(xrGetInstanceProperties)]
pub unsafe fn xr_get_instance_properties(
    _instance: Instance,
    instance_properties: &mut InstanceProperties,
) -> XrResult {
    instance_properties.ty = StructureType::INSTANCE_PROPERTIES;
    copy_str_to_buffer("monoxide", &mut instance_properties.runtime_name);
    instance_properties.runtime_version = Version::new(
        env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
    );
    Ok(())
}