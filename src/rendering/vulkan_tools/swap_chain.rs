use std::collections::{HashMap, HashSet};
use std::i32;
use winit::dpi::{PhysicalSize, Size};
use vulkanalia::{
    Device,
    Instance
};
use vulkanalia::vk::{HasBuilder, KhrSurfaceExtension, PhysicalDevice, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR, Format, Extent2D, Extent3D, SharingMode, ImageUsageFlags, QueueFlags, CompositeAlphaFlagsKHR, Handle, KhrSwapchainExtension};


use crate::rendering::vulkan_tools::{CreateSwapchainSupportError, PhysicalDeviceInfo, PickSwapchainError};
use crate::rendering::vulkan_tools::CreateSwapchainSupportError::VulkanError;
use crate::rendering::vulkan_tools::PickSwapchainError::{ChooseFormatError, CreateSwapchainError};

pub struct SwapСhainSupport {
    pub capabilities: SurfaceCapabilitiesKHR,
    pub formats: Vec<SurfaceFormatKHR>,
    pub present_modes: Vec<PresentModeKHR>
}

impl SwapСhainSupport {
    pub unsafe fn create(
        instance: &Instance,
        surface: &SurfaceKHR,
        physical_device: &PhysicalDevice
    ) -> Result<Self, CreateSwapchainSupportError> {
        let capabilities = instance
            .get_physical_device_surface_capabilities_khr(physical_device.clone(), surface.clone())
            .map_err(|err|VulkanError(err))?;

        let formats = instance
            .get_physical_device_surface_formats_khr(physical_device.clone(), surface.clone())
            .map_err(|err|VulkanError(err))?;

        let present_modes = instance
            .get_physical_device_surface_present_modes_khr(physical_device.clone(), surface.clone())
            .map_err(|err|VulkanError(err))?;

        Result::Ok(Self{
            capabilities,
            formats,
            present_modes
        })
    }
}


pub unsafe fn pick_swap_chain(
    instance: &Instance,
    surface: &SurfaceKHR,
    device_info: &PhysicalDeviceInfo,
    logical_device: &Device,
    size: &PhysicalSize<u32>
) -> Result<SwapchainKHR, PickSwapchainError> {
    let support = SwapСhainSupport::create(instance, surface, &device_info.device)?;

    let format = choose_swap_chain_surface_format(support.formats)
        .ok_or(ChooseFormatError)?;
    let present_mode = choose_present_mode(support.present_modes);
    let extent = choose_swap_chain_extent(size, support.capabilities);

    let image_count = (support.capabilities.min_image_count + 1).clamp(
        support.capabilities.min_image_count,
        support.capabilities.max_image_count
    );

    let sharing_mode = SharingMode::EXCLUSIVE;

    //TODO
    let queue_family_indices = [
        device_info.get_queue_index(QueueFlags::GRAPHICS).unwrap(),
        device_info.get_present_queue_index(surface).unwrap()
    ];
    //TODO
    // let sharing_mode = if indices.graphics != indices.present {
    //     queue_family_indices.push(indices.graphics);
    //     queue_family_indices.push(indices.present);
    //      Разрешено использовать изображение в нескольких семействах очередей
    //     vk::SharingMode::CONCURRENT
    // } else {
    //     vk::SharingMode::EXCLUSIVE
    // };

    let swap_chain_info = SwapchainCreateInfoKHR::builder()
        .surface(surface.clone())
        .min_image_count(image_count)
        .image_format(format.format)
        .image_color_space(format.color_space)
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(sharing_mode)
        .queue_family_indices(&queue_family_indices)
        .pre_transform(support.capabilities.current_transform)
        .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(SwapchainKHR::null()) // TODO
        .build();

    let swap_chain = logical_device.create_swapchain_khr(&swap_chain_info, None)
        .map_err(|err|CreateSwapchainError(err))?;
    return Result::Ok(swap_chain);
}

fn choose_swap_chain_extent(
    window_size: &PhysicalSize<u32>,
    capabilities: SurfaceCapabilitiesKHR
) -> Extent2D {
    if capabilities.current_extent.width != u32::MAX{
        return  capabilities.current_extent
    }


   let extent = Extent2D::builder()
        .width(window_size.width.clamp(
            capabilities.min_image_count,
            capabilities.max_image_count
        ))
        .height(window_size.height.clamp(
            capabilities.min_image_count,
            capabilities.max_image_count
        ))
       .build();

    return extent;
}

fn choose_present_mode(supported_present_modes: Vec<PresentModeKHR>) -> PresentModeKHR {
    let supported_present_modes = supported_present_modes
        .iter()
        .collect::<HashSet<_>>();

    if supported_present_modes.contains(&PresentModeKHR::MAILBOX) {
        return PresentModeKHR::MAILBOX;
    }

    return PresentModeKHR::FIFO;
}

fn choose_swap_chain_surface_format(
    formats: Vec<SurfaceFormatKHR>
) -> Option<SurfaceFormatKHR> {
    for availableFormat in formats{
        if availableFormat.format == Format::B8G8R8A8_SRGB{
            return Some(availableFormat);
        }
    }
    return None;
}