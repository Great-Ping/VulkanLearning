use std::collections::LinkedList;
use std::{env, path};
use std::ops::{Deref, Range};
use log::{debug, info, set_logger_racy};

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
use vulkanalia::vk::{ExtDebugUtilsExtension, Fence, KhrSurfaceExtension, KhrSwapchainExtension, Semaphore};
use winit::dpi::PhysicalSize;
use crate::rendering::RenderingError::{AcquireImageError, LoadShadersError, PresentationError, QueueSubmitError, ResetFenceError, SupportError, WaitForFencesError};

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
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    frame_in_flight_fences: Vec<vk::Fence>,
    swapchain_image_fences: Vec<vk::Fence>,
    flight_frames_count: u8,
    current_frame_index: u8,
    current_image_index: u8
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
        image_available_semaphores: Vec<vk::Semaphore>,
        render_finished_semaphores: Vec<vk::Semaphore>,
        frame_in_flight_fences: Vec<vk::Fence>,
        flight_frames_count: u8,
    ) -> RenderingQueue
    {
        let images_count = swap_chain.images.len();
        let mut swapchain_image_fences = Vec::with_capacity(images_count);
        for _ in 0..images_count{
            swapchain_image_fences.push(vk::Fence::null());
        }


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
            image_available_semaphores,
            render_finished_semaphores,
            flight_frames_count,
            frame_in_flight_fences,
            swapchain_image_fences,
            current_frame_index: 0,
            current_image_index: 0
        }
    }

    pub fn render(&mut self)-> RqResult<()>{
        unsafe {
            let current_frame = self.current_frame_index;
            self.current_frame_index = (current_frame + 1) % self.flight_frames_count;

            let current_image = self.current_image_index;
            self.current_image_index = (current_image + 1) % (self.swap_chain.images.len() as u8);

            let image_semaphore = self.image_available_semaphores[current_frame as usize];
            let finished_semaphore = self.render_finished_semaphores[current_frame as usize];

            let frame_fence = self.frame_in_flight_fences[current_frame as usize];
            let image_fence = self.swapchain_image_fences[current_image as usize];

            self.swapchain_image_fences[current_image as usize] = frame_fence;


            debug!("frame: {0:?} image: {1:?}",
                current_frame,
                current_image
            );


            let result = self.render_one_frame(
                image_semaphore, finished_semaphore,
                frame_fence, image_fence
            )?;

            Result::Ok(result)
        }
    }

    unsafe fn render_one_frame(&self,
        image_available_semaphore: vk::Semaphore,
        render_finished_semaphore: vk::Semaphore,
        frame_fence: vk::Fence,
        image_fence: vk::Fence
    ) -> RqResult<()> {
        //timeout nanoseconds
        let one_minute = 10_000_000_000;
        self.logical_device.wait_for_fences(&[frame_fence], true, one_minute)
            .map_err(|err| WaitForFencesError(err))?;

        let image_index = self.logical_device
            .acquire_next_image_khr(
                self.swap_chain.swap_chain,
                u64::MAX,
                image_available_semaphore,
                vk::Fence::null()
            ).map_err(|err| AcquireImageError(err))?;
        let image_index = image_index.0 as usize;

        if !image_fence.is_null() {
            self.logical_device.wait_for_fences(&[image_fence], true, one_minute)
                .map_err(|err| WaitForFencesError(err))?;
        }

        let wait_semaphores = &[image_available_semaphore];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let command_buffers = &[self.command_buffers[image_index as usize]];
        let signal_semaphores = &[render_finished_semaphore];

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores)
            .build();

        self.logical_device.reset_fences(&[frame_fence])
            .map_err(|err| WaitForFencesError(err))?;

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
            self.render_finished_semaphores.iter().for_each(
                |semaphore: &Semaphore| self.logical_device.destroy_semaphore(*semaphore, None)
            );

            self.image_available_semaphores.iter().for_each(
                |semaphore: &Semaphore| self.logical_device.destroy_semaphore(*semaphore, None)
            );

            self.frame_in_flight_fences.iter().for_each(
                |fence| self.logical_device.destroy_fence(*fence, None)
            );
            self.logical_device.destroy_command_pool(self.command_pool, None);

            self.framebuffers.iter().for_each(
                |buffer| self.logical_device.destroy_framebuffer(*buffer, None)
            );

            self.logical_device.destroy_command_pool(self.command_pool, None);
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