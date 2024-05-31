use std::collections::LinkedList;
use std::{env, path};
use std::ops::{Deref, Range};
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
use vulkanalia::vk::{
    ExtDebugUtilsExtension,
    KhrSurfaceExtension,
    KhrSwapchainExtension
};
use winit::dpi::PhysicalSize;
use crate::rendering::RenderingError::{AcquireImageError, LoadShadersError, PresentationError, QueueSubmitError, SupportError};

use super::shaders::Shader;
use super::{QueueFamilyIndices, RenderingError, SwapChainData, get_debug_info, RqResult, DeviceQueues};

#[derive(Debug)]
pub struct RenderingQueue {
    entry: Box<Entry>,
    instance: Box<Instance>,
    messenger: Option<vk::DebugUtilsMessengerEXT>,
    physical_device: vk::PhysicalDevice,
    logical_device: Box<Device>,
    queues: Box<DeviceQueues>,
    surface: vk::SurfaceKHR,
    swap_chain: Box<SwapChainData>,
    render_pass: vk::RenderPass,
    pipeline: vk::Pipeline,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
    image_available_semaphore: vk::Semaphore,
    render_finished_semaphore: vk::Semaphore,
    flight_frames_count: u8
}

impl RenderingQueue {

    pub fn new (
        entry: Box<Entry>,
        instance: Box<Instance>,
        messenger: Option<vk::DebugUtilsMessengerEXT>,
        physical_device: vk::PhysicalDevice,
        logical_device: Box<Device>,
        queues: Box<DeviceQueues>,
        surface: vk::SurfaceKHR,
        swap_chain: Box<super::SwapChainData>,
        render_pass: vk::RenderPass,
        pipeline: vk::Pipeline,
        framebuffers: Vec<vk::Framebuffer>,
        command_pool: vk::CommandPool,
        command_buffers: Vec<vk::CommandBuffer>,
        image_available_semaphore: vk::Semaphore,
        render_finished_semaphore: vk::Semaphore,
        flight_frames_count: u8
    ) -> RenderingQueue
    {
        return RenderingQueue {
            entry,
            instance,
            messenger,
            physical_device,
            logical_device,
            queues,
            surface,
            swap_chain,
            render_pass,
            pipeline,
            framebuffers,
            command_pool,
            command_buffers,
            image_available_semaphore,
            render_finished_semaphore,
            flight_frames_count
        }
    }

    pub fn render(&self)-> RqResult<()>{
        unsafe {
            self.render_unsafe()
        }
    }

    unsafe fn render_unsafe(&self) -> RqResult<()>{
        let image_index = self.logical_device
            .acquire_next_image_khr(
                self.swap_chain.swap_chain,
                u64::MAX,
                self.image_available_semaphore,
                vk::Fence::null(),
            ).map_err(|err| AcquireImageError(err))?;
        let image_index = image_index.0 as usize;

        let wait_semaphores = &[self.image_available_semaphore];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = &[self.command_buffers[image_index as usize]];
        let signal_semaphores = &[self.render_finished_semaphore];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores)
            .build();

        self.logical_device.queue_submit(
            self.queues.graphics, &[submit_info], vk::Fence::null()
        ).map_err(|err|QueueSubmitError(err))?;

        let swap_chain = &[self.swap_chain.swap_chain];
        let image_indices = &[image_index as u32];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swap_chain)
            .image_indices(image_indices)
            .build();

        unsafe {
            self.logical_device.queue_present_khr(self.queues.present, &present_info)
                .map_err(|err| PresentationError(err));
        }

        Result::Ok(())
    }



    pub fn device_wait_idle(&self){
        unsafe {
            self.logical_device.device_wait_idle().unwrap();
        }
    }
}

impl Drop for RenderingQueue {
    fn drop(&mut self){
        unsafe {
            self.logical_device.destroy_semaphore(self.render_finished_semaphore, None);
            self.logical_device.destroy_semaphore(self.image_available_semaphore, None);

            self.logical_device.destroy_command_pool(self.command_pool, None);

            self.framebuffers.iter().for_each(
                |buffer| self.logical_device.destroy_framebuffer(*buffer, None)
            );

            self.logical_device.destroy_pipeline(self.pipeline, None);

            self.logical_device.destroy_render_pass(self.render_pass, None);
           /// self.logical_device.destroy_pipeline_layout(self);

            for image_view in &self.swap_chain.image_views{
                self.logical_device.destroy_image_view(*image_view, None);
            }

            self.logical_device.destroy_swapchain_khr(self.swap_chain.swap_chain, None);

            self.instance.destroy_surface_khr(self.surface, None);
            self.logical_device.destroy_device(None);

            if let Some(messenger) = &self.messenger {
                self.instance.destroy_debug_utils_messenger_ext(*messenger, None);
            }
            self.instance.destroy_instance(None);

        }
        debug!("instance destroyed");
    }
}