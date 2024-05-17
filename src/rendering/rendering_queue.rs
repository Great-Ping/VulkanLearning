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

use vulkanalia::prelude::v1_0::*;
use vulkanalia::bytecode::Bytecode;
use vulkanalia::window::create_surface;
use vulkanalia::vk;
use vulkanalia::vk::{ExtDebugUtilsExtension, KhrSurfaceExtension, KhrSwapchainExtension, Semaphore};
use winit::dpi::PhysicalSize;
use crate::rendering::RenderingError::{LoadShadersError, SupportError};

use super::shaders::Shader;
use super::{ QueueFamilyIndices, RenderingError, SwapChainData, get_debug_info, RqResult};

#[derive(Debug)]
pub struct RenderingQueue {
    entry: Box<Entry>,
    instance: Box<Instance>,
    messenger: Option<vk::DebugUtilsMessengerEXT>,
    physical_device: vk::PhysicalDevice,
    logical_device: Box<Device>,
    queue_families: QueueFamilyIndices,
    surface: vk::SurfaceKHR,
    swap_chain: Box<SwapChainData>,
    render_pass: vk::RenderPass,
    pipeline: vk::Pipeline,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
    image_available_semaphore: Semaphore,
    render_finished_semaphore: Semaphore


}

impl RenderingQueue {

    pub fn new (
        entry: Box<Entry>,
        instance: Box<Instance>,
        messenger: Option<vk::DebugUtilsMessengerEXT>,
        physical_device: vk::PhysicalDevice,
        logical_device: Box<Device>,
        queue_families: super::QueueFamilyIndices,
        surface: vk::SurfaceKHR,
        swap_chain: Box<super::SwapChainData>,
        render_pass: vk::RenderPass,
        pipeline: vk::Pipeline,
        framebuffers: Vec<vk::Framebuffer>,
        command_pool: vk::CommandPool,
        command_buffers: Vec<vk::CommandBuffer>,
        image_available_semaphore: Semaphore,
        render_finished_semaphore: Semaphore
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
            swap_chain,
            render_pass,
            pipeline,
            framebuffers,
            command_pool,
            command_buffers,
            image_available_semaphore,
            render_finished_semaphore
        }
    }

    pub fn render(){

    }
}

impl Drop for RenderingQueue {
    fn drop(&mut self){
        unsafe {
            if let Some(messenger) = &self.messenger {
                self.instance.destroy_debug_utils_messenger_ext(*messenger, None);
            }

            // self.logical_device.destroy_pipeline();

            for image_view in &self.swap_chain.image_views{
                self.logical_device.destroy_image_view(*image_view, None);
            }

            self.logical_device.destroy_swapchain_khr(self.swap_chain.swap_chain, None);

            self.instance.destroy_surface_khr(self.surface, None);
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
        debug!("instance destroyed");
    }
}