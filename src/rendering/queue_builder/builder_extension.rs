use vulkanalia::prelude::v1_0::*;
use vulkanalia::loader::{
    LibloadingLoader,
    LIBRARY
};
use vulkanalia::vk::{PipelineLayout, Semaphore};

use crate::rendering::{DeviceQueues, RenderingQueue, RqResult};
use crate::rendering::RenderingError::{
    CreateEntryError,
    LoadLibraryError
};
use super::build_stages::InstanceBuildStage;

pub struct RenderingQueueBuilder;

impl RenderingQueueBuilder {

    pub fn new() -> RenderingQueueBuilder {
        Self
    }

    pub fn create_entry(self) -> RqResult<InstanceBuildStage>{
        let entry = unsafe {
            let loader = LibloadingLoader::new(LIBRARY)
                .map_err(|err| LoadLibraryError(err))?;

            Entry::new(loader)
                .map_err(|err| CreateEntryError(err))?
        };

        Result::Ok( InstanceBuildStage {
            entry: Box::new(entry)
        })
    }
}

pub struct EndBuildStage {
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<vk::DebugUtilsMessengerEXT>,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: Box<Device>,
    pub queue_families: super::QueueFamilyIndices,
    pub surface: vk::SurfaceKHR,
    pub swap_chain: Box<super::SwapChainData>,
    pub render_pass: vk::RenderPass,
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: PipelineLayout,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub image_available_semaphores: Vec<Semaphore>,
    pub render_finished_semaphores: Vec<Semaphore>,
    pub frame_in_flight_fences: Vec<vk::Fence>,
    pub flight_frames_count: u8
}

impl EndBuildStage {
    pub fn build(self) -> RenderingQueue {

        let graphics = unsafe {
            self.logical_device.get_device_queue(self.queue_families.graphics, 0)
        };
        let present = unsafe {
            self.logical_device.get_device_queue(self.queue_families.present, 0)
        };

        let queues = Box::new(DeviceQueues{
            indices: self.queue_families,
            graphics,
            present
        });

        return RenderingQueue::new(
            self.entry,
            self.instance,
            self.messenger,
            self.physical_device,
            self.logical_device,
            queues,
            self.surface,
            self.swap_chain,
            self.render_pass,
            self.pipeline,
            self.pipeline_layout,
            self.framebuffers,
            self.command_pool,
            self.command_buffers,
            self.image_available_semaphores,
            self.render_finished_semaphores,
            self.frame_in_flight_fences,
            self.flight_frames_count
        )
    }
}

impl RenderingQueue {

    pub fn builder() -> RenderingQueueBuilder {
        return RenderingQueueBuilder::new();
    }
}