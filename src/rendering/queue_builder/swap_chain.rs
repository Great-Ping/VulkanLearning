use std::collections::HashSet;
use vulkanalia::{
    Device,
    Entry,
    Instance
};
use vulkanalia::vk::{HasBuilder, KhrSurfaceExtension, PhysicalDevice, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR, Format, Extent2D, SharingMode, ImageUsageFlags, CompositeAlphaFlagsKHR, KhrSwapchainExtension, DebugUtilsMessengerEXT, Image, ComponentSwizzle, ComponentMapping, ImageSubresourceRange, ImageViewCreateInfo, ImageViewType, ImageAspectFlags, DeviceV1_0, ImageView};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};


use crate::rendering::queue_builder::{RenderingQueueBuildError, QueueFamilyIndices};
use crate::rendering::queue_builder::initial_builder::EndBuilder;
use crate::rendering::queue_builder::RenderingQueueBuildError::{ErrorCode, ErrorMessage};
use crate::rendering::rendering_queue_config::RenderingResolution;
use crate::rendering::RenderingPipelineConfig;

pub struct SwapChainBuilder {
    pub entry: Entry,
    pub instance: Instance,
    pub messenger: Option<DebugUtilsMessengerEXT>,
    pub surface: SurfaceKHR,
    pub physical_device: PhysicalDevice,
    pub logical_device: Device,
    pub queue_families: QueueFamilyIndices,
    pub swap_chain_support: SwapСhainSupport,
}

impl SwapChainBuilder {
    pub fn create_swap_chain(self, rendering_resolution: RenderingResolution, old_swapchain: SwapchainKHR) -> Result<EndBuilder, RenderingQueueBuildError> {

        let support = &self.swap_chain_support;
        let format = choose_swap_chain_surface_format(&support.formats)
            .ok_or(ErrorMessage("Choose format error"))?;
        let present_mode = choose_present_mode(&support.present_modes);
        let extent = choose_swap_chain_extent(&self.config.rendering_resolution, &support.capabilities);

        let image_count = (support.capabilities.min_image_count + 1).clamp(
            support.capabilities.min_image_count,
            support.capabilities.max_image_count
        );

        let queue_family_indices = self.queue_families.get_unique_indices();
        let sharing_mode = if queue_family_indices.iter().count() > 1 {
            SharingMode::CONCURRENT
        } else {
            SharingMode::EXCLUSIVE
        };

        let swap_chain_info = SwapchainCreateInfoKHR::builder()
            .surface(self.surface.clone())
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
            .old_swapchain(old_swapchain)
            .build();

        let swap_chain = unsafe{
            self.logical_device.create_swapchain_khr(&swap_chain_info, None)
                .map_err(|err| ErrorCode(err))?
        };

        let swap_chain_images = unsafe {
            self.logical_device.get_swapchain_images_khr(swap_chain)
                .map_err(|err| ErrorCode(err))?
        };

        let swap_chain_image_views = create_swap_chain_image_views(
            &self.logical_device,
            &swap_chain_images,
            &format.format
        )?;

        return Result::Ok(EndBuilder {
            entry: self.entry,
            instance: self.instance,
            messenger: self.messenger,
            physical_device: self.physical_device,
            logical_device: self.logical_device,
            queue_families: self.queue_families,
            surface: self.surface,
            swap_chain,
            swap_chain_extent: extent,
            swap_chain_images,
            swap_chain_image_views
        });
    }
}


#[derive(Clone)]
pub struct SwapСhainSupport {
    pub capabilities: SurfaceCapabilitiesKHR,
    pub formats: Vec<SurfaceFormatKHR>,
    pub present_modes: Vec<PresentModeKHR>
}

impl SwapСhainSupport {
    pub fn create(
        instance: &Instance,
        surface: &SurfaceKHR,
        physical_device: &PhysicalDevice
    ) -> Result<Self, RenderingQueueBuildError> {
        let capabilities = unsafe {
            instance
                .get_physical_device_surface_capabilities_khr(physical_device.clone(), surface.clone())
                .map_err(|err|ErrorCode(err))?
        };

        let formats = unsafe {
            instance
                .get_physical_device_surface_formats_khr(physical_device.clone(), surface.clone())
                .map_err(|err|ErrorCode(err))?
        };

        let present_modes = unsafe {
            instance
                .get_physical_device_surface_present_modes_khr(physical_device.clone(), surface.clone())
                .map_err(|err|ErrorCode(err))?
        };

        Result::Ok(Self{
            capabilities,
            formats,
            present_modes
        })
    }
}

fn choose_swap_chain_extent(
    rendering_resolution: &RenderingResolution,
    capabilities: &SurfaceCapabilitiesKHR
) -> Extent2D {
    if capabilities.current_extent.width != u32::MAX{
        return  capabilities.current_extent
    }

   let extent = Extent2D::builder()
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

fn choose_present_mode(supported_present_modes: &Vec<PresentModeKHR>) -> PresentModeKHR {
    let supported_present_modes = supported_present_modes
        .iter()
        .collect::<HashSet<_>>();

    if supported_present_modes.contains(&PresentModeKHR::MAILBOX) {
        return PresentModeKHR::MAILBOX;
    }

    return PresentModeKHR::FIFO;
}

fn choose_swap_chain_surface_format(
    formats: &Vec<SurfaceFormatKHR>
) -> Option<SurfaceFormatKHR> {
    for availableFormat in formats {
        if availableFormat.format == Format::B8G8R8A8_SRGB{
            return Some(availableFormat.clone());
        }
    }
    return None;
}

fn create_swap_chain_image_views(
    device: &Device,
    images: &Vec<Image>,
    format: &Format,
) -> Result<Vec<ImageView>, RenderingQueueBuildError> {
    let mut image_views = Vec::with_capacity(images.len());

    for image in images {
        let components = ComponentMapping::builder()
            .r(ComponentSwizzle::IDENTITY)
            .g(ComponentSwizzle::IDENTITY)
            .b(ComponentSwizzle::IDENTITY)
            .a(ComponentSwizzle::IDENTITY);

        let subresource_range = ImageSubresourceRange::builder()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build();

        let view_info = ImageViewCreateInfo::builder()
            .image(image.clone())
            .view_type(ImageViewType::_2D)
            .format(format.clone())
            .components(components)
            .subresource_range(subresource_range)
            .build();

        unsafe {
            let image_view = device.create_image_view(&view_info, None)
                .map_err(|err| ErrorCode(err))?;

            image_views.push(image_view);
        }
    }

    Result::Ok(image_views)
}
