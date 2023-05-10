use crate::openxr::prelude::*;

pub const PROPERTIES: ExtensionProperties = ExtensionProperties {
    ty: ExtensionProperties::TYPE,
    next: std::ptr::null_mut(),
    extension_name: [0; 128],
    extension_version: 2,
};

/// # Docs
/// See [xrCreateVulkanInstanceKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateVulkanInstanceKHR)
#[openxr(xrCreateVulkanInstanceKHR)]
pub unsafe fn xr_create_vulkan_instance_khr(
    _instance: Instance,
    _create_info: *const VulkanInstanceCreateInfoKHR,
    _vulkan_instance: &mut VkInstance,
    _vulkan_result: &mut VkResult,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrCreateVulkanDeviceKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateVulkanDeviceKHR)
#[openxr(xrCreateVulkanDeviceKHR)]
pub unsafe fn xr_create_vulkan_device_khr(
    _instance: Instance,
    _create_info: *const VulkanDeviceCreateInfoKHR,
    _vulkan_device: &mut VkDevice,
    _vulkan_result: &mut VkResult,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrGetVulkanGraphicsDevice2KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetVulkanGraphicsDevice2KHR)
#[openxr(xrGetVulkanGraphicsDevice2KHR)]
pub unsafe fn xr_get_vulkan_graphics_device_2_khr(
    _instance: Instance,
    _get_info: *const VulkanGraphicsDeviceGetInfoKHR,
    _vulkan_physical_device: &mut VkPhysicalDevice,
) -> XrResult {
    todo!()
}

/// # Docs
/// See [xrGetVulkanGraphicsRequirements2KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetVulkanGraphicsRequirements2KHR)
#[openxr(xrGetVulkanGraphicsRequirements2KHR)]
pub unsafe fn xr_get_vulkan_graphics_requirements_2_khr(
    _instance: Instance,
    _system_id: SystemId,
    _graphics_requirements: &mut GraphicsRequirementsVulkanKHR,
) -> XrResult {
    todo!()
}
