use std::collections::LinkedList;
use std::{env, path};
use std::ops::Range;
use log::{debug, info};
use winit::raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle
};
use vulkanalia::Entry;
use vulkanalia::loader::{
    LibloadingLoader,
    LIBRARY
};

use vulkanalia::{
    Instance,
    Device
};
use vulkanalia::bytecode::Bytecode;
use vulkanalia::window::create_surface;
use vulkanalia::vk;
use vulkanalia::vk::{DeviceV1_0, InstanceV1_0, ExtDebugUtilsExtension, Handle, HasBuilder, KhrSurfaceExtension, KhrSwapchainExtension, ShaderRequiredSubgroupSizeCreateInfoEXT};
use winit::dpi::PhysicalSize;
use crate::rendering::RenderingError::{LoadShadersError, SupportError};

use super::shaders::Shader;
use super::{RenderingPipelineConfig, QueueFamilyIndices, RenderingError, SwapChainData, get_debug_info, RqResult};

#[derive(Debug)]
pub struct RenderingQueue {
    entry: Box<Entry>,
    instance: Box<Instance>,
    messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
    physical_device: Box<vk::PhysicalDevice>,
    logical_device: Box<Device>,
    queue_families: QueueFamilyIndices,
    surface: Box<vk::SurfaceKHR>,
    swap_chain: Box<SwapChainData>
}

impl RenderingQueue {

    pub fn new (
        entry: Box<Entry>,
        instance: Box<Instance>,
        messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
        physical_device: Box<vk::PhysicalDevice>,
        logical_device: Box<Device>,
        queue_families:QueueFamilyIndices,
        surface: Box<vk::SurfaceKHR>,
        swap_chain: Box<SwapChainData>
    ) -> RenderingQueue
    {
        return RenderingQueue {
            entry,
            instance,
            messenger,
            physical_device,
            logical_device,
            queue_families,
            surface,
            swap_chain
        }
    }


    pub fn create<TWindow>(
        config: &RenderingPipelineConfig<&TWindow>
    ) -> RqResult<RenderingQueue>
    where TWindow: HasWindowHandle+HasDisplayHandle
    {

        let rendering_queue = Self::builder()
            .create_entry()?
            .create_instance(
                &config.window,
                config.use_validation_layer
            )?
            .choose_physical_device(
                vk::PhysicalDeviceType::INTEGRATED_GPU
            )?
            .create_logical_device(
                config.use_validation_layer
            )?
            .create_swap_chain(
                &config.rendering_resolution,
                vk::SwapchainKHR::null()
            )?
            .create_render_pass()?;

        let mut path_to = env::current_exe()
            .map_err(|err|LoadShadersError(String::from("Unable to get the path")))?;
        path_to.pop();
        path_to.push("assets/shaders");
        path_to.push("Example.frag.spv");
        let mut buffer = Vec::with_capacity(4096);
        let fragShader = Shader::read_file(&path_to, &rendering_queue.logical_device, &mut buffer)?;
        path_to.pop();
        path_to.push("Example.vert.spv");
        let vertShader = Shader::read_file(&path_to, &rendering_queue.logical_device, &mut buffer)?;

        let renedering_queue = rendering_queue
            .add_pipeline(&vertShader, &fragShader)?
            .create_framebuffers()?;

        Result::Err(SupportError("Not implemented"))
    }
}

impl Drop for RenderingQueue {
    fn drop(&mut self){
        unsafe {
            if let Some(messenger) = &self.messenger {
                self.instance.destroy_debug_utils_messenger_ext(**messenger, None);
            }

            // self.logical_device.destroy_pipeline();

            for image_view in &self.swap_chain.image_views{
                self.logical_device.destroy_image_view(*image_view, None);
            }

            self.logical_device.destroy_swapchain_khr(self.swap_chain.swap_chain, None);

            self.instance.destroy_surface_khr(*self.surface, None);
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
        debug!("instance destroyed");
    }
}