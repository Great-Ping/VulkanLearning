use vulkanalia::prelude::v1_0::*;
use crate::rendering::queue_builder::command_pool::CommandPoolBuildStage;
use crate::rendering::RenderingError::CreateFrameBufferError;
use crate::rendering::RqResult;

pub struct FramebuffersBuildStage{
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<vk::DebugUtilsMessengerEXT>,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: Box<Device>,
    pub queue_families: super::QueueFamilyIndices,
    pub surface: vk::SurfaceKHR,
    pub swap_chain: Box<super::SwapChainData>,
    pub render_pass: vk::RenderPass,
    pub pipeline: vk::Pipeline
}

impl FramebuffersBuildStage {
    pub fn create_framebuffers(self) -> RqResult<CommandPoolBuildStage>{
        let mut framebuffers = Vec::with_capacity(self.swap_chain.image_views.len());

        for imageView in &self.swap_chain.image_views {
            let attachments = &[imageView.clone()];
            let framebuffer_info = vk::FramebufferCreateInfo::builder()
                .render_pass(self.render_pass)
                .attachments(attachments)
                .width(self.swap_chain.extent.width)
                .height(self.swap_chain.extent.height)
                .layers(1);

            let framebuffer = unsafe {
                self.logical_device.create_framebuffer(
                    &framebuffer_info,
                    None
                ).map_err(|err| CreateFrameBufferError(err))?
            };

            framebuffers.push(framebuffer);
        }

        Result::Ok(CommandPoolBuildStage{
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
            framebuffers
        })
    }
}