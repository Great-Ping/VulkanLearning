use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::{
    HasBuilder,
    KhrSurfaceExtension,
    KhrSwapchainExtension
};

use crate::rendering::queue_builder::builder_extension::EndBuildStage;
use crate::rendering::RenderingError::CreateSyncObjectsError;
use crate::rendering::RqResult;


pub struct SyncObjectsBuildStage{
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
    pub framebuffers: Vec<vk::Framebuffer>,
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>
}


impl SyncObjectsBuildStage{
    pub fn create_sync_objects(self, flight_frames_count: u8) -> RqResult<EndBuildStage>{
        let semaphore_info = vk::SemaphoreCreateInfo::default();
        let fence_info = vk::FenceCreateInfo::builder()
            .flags(vk::FenceCreateFlags::SIGNALED)
            .build();

        let mut image_available_semaphores = Vec::with_capacity(flight_frames_count as usize);
        let mut render_finished_semaphores = Vec::with_capacity(flight_frames_count as usize);
        let mut frame_in_flight_fences = Vec::with_capacity(flight_frames_count as usize);

        for _ in 0..flight_frames_count{
            let fence = unsafe {
                self.logical_device.create_fence(&fence_info, None)
                    .map_err(|err| CreateSyncObjectsError(err))?
            };

            let image = unsafe {
                self.logical_device.create_semaphore(&semaphore_info, None)
                    .map_err(|err| CreateSyncObjectsError(err))?
            };
            let render = unsafe{
                self.logical_device.create_semaphore(&semaphore_info, None)
                    .map_err(|err| CreateSyncObjectsError(err))?
            };

            frame_in_flight_fences.push(fence);
            image_available_semaphores.push(image);
            render_finished_semaphores.push(render);
        }

        Result::Ok(EndBuildStage{
            entry: self.entry,
            instance: self.instance,
            messenger: self.messenger,
            physical_device: self.physical_device,
            logical_device: self.logical_device,
            queue_families: self.queue_families,
            surface: self.surface,
            swap_chain: self.swap_chain,
            render_pass: self.render_pass,
            pipeline: self.pipeline,
            framebuffers: self.framebuffers,
            command_pool: self.command_pool,
            command_buffers: self.command_buffers,
            image_available_semaphores,
            render_finished_semaphores,
            frame_in_flight_fences,
            flight_frames_count
        })
    }
}