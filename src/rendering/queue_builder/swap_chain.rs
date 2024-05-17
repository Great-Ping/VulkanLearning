use std::collections::HashSet;

use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::{
    HasBuilder,
    KhrSurfaceExtension,
    KhrSwapchainExtension
};
use winit::dpi::PhysicalSize;

use super::QueueFamilyIndices;
use crate::rendering::{ RenderPassBuildStage, RqResult};
use crate::rendering::RenderingError::{CreateSwapChainError, SupportError};

#[derive(Debug)]
pub struct SwapChainData{
    pub swap_chain: vk::SwapchainKHR,
    pub extent: vk::Extent2D,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub format: vk::Format
}

pub struct RenderingResolution {
    pub width: u32,
    pub height: u32
}

impl From<PhysicalSize<u32>> for RenderingResolution{
    fn from(size: PhysicalSize<u32>) -> Self {
        Self {
            width: size.width,
            height: size.height,
        }
    }
}

pub struct SwapChainBuildStage {
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<vk::DebugUtilsMessengerEXT>,
    pub surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: Box<Device>,
    pub queue_families: QueueFamilyIndices,
    pub swap_chain_support: Box<SwapСhainSupport>,
}

impl SwapChainBuildStage  {
    pub fn create_swap_chain(
        self,
        rendering_resolution: RenderingResolution,
        old_swapchain: vk::SwapchainKHR
    ) -> RqResult<RenderPassBuildStage>
    {
        let support = &self.swap_chain_support;
        let format = choose_swap_chain_surface_format(&support.formats)
            .ok_or(SupportError("suitable format was not found"))?;
        let present_mode = choose_present_mode(&support.present_modes);
        let extent = choose_swap_chain_extent(rendering_resolution, &support.capabilities);

        let image_count = (support.capabilities.min_image_count + 1).clamp(
            support.capabilities.min_image_count,
            support.capabilities.max_image_count
        );

        let queue_family_indices = self.queue_families.get_unique_indices();
        let sharing_mode = if queue_family_indices.iter().count() > 1 {
            vk::SharingMode::CONCURRENT
        } else {
            vk::SharingMode::EXCLUSIVE
        };

        let swap_chain_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(self.surface)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(sharing_mode)
            .queue_family_indices(&queue_family_indices)
            .pre_transform(support.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(old_swapchain)
            .build();

        let swap_chain = unsafe{
            self.logical_device.create_swapchain_khr(&swap_chain_info, None)
                .map_err(|err| CreateSwapChainError(err))?
        };

        let swap_chain_images = unsafe {
            self.logical_device.get_swapchain_images_khr(swap_chain)
                .map_err(|err| CreateSwapChainError(err))?
        };

        let swap_chain_image_views = create_swap_chain_image_views(
            &self.logical_device,
            &swap_chain_images,
            &format.format
        )?;

        let swap_chain_data  = SwapChainData{
            swap_chain,
            extent,
            images: swap_chain_images,
            image_views: swap_chain_image_views,
            format: format.format
        };

        return Result::Ok(RenderPassBuildStage {
            entry: self.entry,
            instance: self.instance,
            messenger: self.messenger,
            physical_device: self.physical_device,
            logical_device: self.logical_device,
            queue_families: self.queue_families,
            surface: self.surface,
            swap_chain: Box::new(swap_chain_data),
        });
    }
}


#[derive(Clone)]
pub struct SwapСhainSupport {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>
}

impl SwapСhainSupport {
    pub fn create(
        instance: &Instance,
        surface: &vk::SurfaceKHR,
        physical_device: &vk::PhysicalDevice
    ) -> RqResult<Self>
    {
        let capabilities = unsafe {
            instance
                .get_physical_device_surface_capabilities_khr(physical_device.clone(), surface.clone())
                .map_err(|err|CreateSwapChainError(err))?
        };

        let formats = unsafe {
            instance
                .get_physical_device_surface_formats_khr(physical_device.clone(), surface.clone())
                .map_err(|err|CreateSwapChainError(err))?
        };

        let present_modes = unsafe {
            instance
                .get_physical_device_surface_present_modes_khr(physical_device.clone(), surface.clone())
                .map_err(|err|CreateSwapChainError(err))?
        };

        Result::Ok(Self{
            capabilities,
            formats,
            present_modes
        })
    }
}

fn choose_swap_chain_extent(
    rendering_resolution: RenderingResolution,
    capabilities: &vk::SurfaceCapabilitiesKHR
) -> vk::Extent2D
{
    if capabilities.current_extent.width != u32::MAX{
        return  capabilities.current_extent
    }

   let extent = vk::Extent2D::builder()
        .width(rendering_resolution.width.clamp(
            capabilities.min_image_extent.width,
            capabilities.max_image_extent.width
        ))
        .height(rendering_resolution.height.clamp(
            capabilities.min_image_extent.height,
            capabilities.max_image_extent.height
        ))
       .build();

    return extent;
}

fn choose_present_mode(
    supported_present_modes: &Vec<vk::PresentModeKHR>
) -> vk::PresentModeKHR
{
    let supported_present_modes = supported_present_modes
        .iter()
        .collect::<HashSet<_>>();

    if supported_present_modes.contains(&vk::PresentModeKHR::MAILBOX) {
        return vk::PresentModeKHR::MAILBOX;
    }

    return vk::PresentModeKHR::FIFO;
}

fn choose_swap_chain_surface_format(
    formats: &Vec<vk::SurfaceFormatKHR>
) -> Option<vk::SurfaceFormatKHR>
{
    for availableFormat in formats {
        if availableFormat.format == vk::Format::B8G8R8A8_SRGB{
            return Some(availableFormat.clone());
        }
    }
    return None;
}

fn create_swap_chain_image_views(
    device: &Device,
    images: &Vec<vk::Image>,
    format: &vk::Format,
) -> RqResult<Vec<vk::ImageView>>
{
    let mut image_views = Vec::with_capacity(images.len());

    for image in images {
        let components = vk::ComponentMapping::builder()
            .r(vk::ComponentSwizzle::IDENTITY)
            .g(vk::ComponentSwizzle::IDENTITY)
            .b(vk::ComponentSwizzle::IDENTITY)
            .a(vk::ComponentSwizzle::IDENTITY);

        let subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build();

        let view_info = vk::ImageViewCreateInfo::builder()
            .image(image.clone())
            .view_type(vk::ImageViewType::_2D)
            .format(format.clone())
            .components(components)
            .subresource_range(subresource_range)
            .build();

        unsafe {
            let image_view = device.create_image_view(&view_info, None)
                .map_err(|err| CreateSwapChainError(err))?;

            image_views.push(image_view);
        }
    }

    Result::Ok(image_views)
}
