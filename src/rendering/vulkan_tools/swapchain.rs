use vulkanalia::Instance;
use vulkanalia::vk::{
    KhrSurfaceExtension,
    PhysicalDevice,
    PresentModeKHR,
    SurfaceCapabilitiesKHR,
    SurfaceFormatKHR,
    SurfaceKHR
};

use crate::rendering::vulkan_tools::CreateSwapchainSupportError;
use crate::rendering::vulkan_tools::CreateSwapchainSupportError::VulkanError;

pub struct SwapchainSupport {
    pub capabilities: SurfaceCapabilitiesKHR,
    pub formats: Vec<SurfaceFormatKHR>,
    pub present_modes: Vec<PresentModeKHR>
}

impl SwapchainSupport{
    pub unsafe fn create(
        instance: &Instance,
        surface: &SurfaceKHR,
        physical_device: PhysicalDevice
    ) -> Result<Self, CreateSwapchainSupportError> {
        let capabilities = instance
            .get_physical_device_surface_capabilities_khr(physical_device, surface.clone())
            .map_err(|err|VulkanError(err))?;

        let formats = instance
            .get_physical_device_surface_formats_khr(physical_device, surface.clone())
            .map_err(|err|VulkanError(err))?;

        let present_modes = instance
            .get_physical_device_surface_present_modes_khr(physical_device, surface.clone())
            .map_err(|err|VulkanError(err))?;

        Result::Ok(Self{
            capabilities,
            formats,
            present_modes
        })
    }
}