use crate::{openxr::prelude::*};

/// # Docs
/// See [xrCreateInstance](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateInstance)
#[openxr(xrCreateInstance)]
pub unsafe fn xr_create_instance(
    _info: &InstanceCreateInfo,
    _instance: &mut Instance,
) -> Result<(), XrResult> {
    todo!()
}

/// # Docs
/// See [xrDestroyInstance](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyInstance)
#[openxr(xrDestroyInstance)]
pub unsafe fn xr_destroy_instance(_instance: Instance) -> Result<(), XrResult> {
    todo!()
}

/// # Docs
/// See [xrGetInstanceProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetInstanceProperties)
#[openxr(xrGetInstanceProperties)]
pub unsafe fn xr_get_instance_properties(
    _instance: Instance,
    instance_properties: &mut InstanceProperties,
) -> Result<(), XrResult> {
    instance_properties.ty = StructureType::INSTANCE_PROPERTIES;
    copy_str_to_buffer("Stardust XR", &mut instance_properties.runtime_name);
    instance_properties.runtime_version = Version::new(
        env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
    );
    Ok(())
}
