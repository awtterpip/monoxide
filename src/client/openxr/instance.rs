use crate::client::openxr::{util::copy_str_to_buffer, XrResult};
use openxr_sys::{Instance, InstanceCreateInfo, InstanceProperties, StructureType, Version};

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrCreateInstance
#[no_mangle]
pub unsafe extern "system" fn xrCreateInstance(
    _info: &InstanceCreateInfo,
    _instance: &mut Instance,
) -> XrResult {
    wrap_oxr! {
        todo!()
    }
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrDestroyInstance
#[no_mangle]
pub unsafe extern "system" fn xrDestroyInstance(_instance: Instance) -> XrResult {
    wrap_oxr! {
        todo!()
    }
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrGetInstanceProperties
#[no_mangle]
pub unsafe extern "system" fn xrGetInstanceProperties(
    _instance: Instance,
    instance_properties: &mut InstanceProperties,
) -> XrResult {
    wrap_oxr! {
        instance_properties.ty = StructureType::INSTANCE_PROPERTIES;
        copy_str_to_buffer("Stardust XR", &mut instance_properties.runtime_name);
        instance_properties.runtime_version = Version::new(
            env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        );
    }
}
