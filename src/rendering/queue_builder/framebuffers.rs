use std::collections::LinkedList;
use vulkanalia::{Device, Entry, Instance, vk};
use vulkanalia::vk::{DeviceV1_0, HasBuilder, RenderPass};
use crate::rendering::queue_builder::command_buffer::CommandBufferBuildStage;
use crate::rendering::RenderingError::CreateFrameBufferError;
use crate::rendering::RqResult;

pub struct FramebuffersBuildStage{
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
    pub physical_device: Box<vk::PhysicalDevice>,
    pub logical_device: Box<Device>,
    pub queue_families: super::QueueFamilyIndices,
    pub surface: Box<vk::SurfaceKHR>,
    pub swap_chain: Box<super::SwapChainData>,
    pub render_pass: Box<RenderPass>,
    pub pipelines: Vec<vk::Pipeline>
}

impl FramebuffersBuildStage {
    fn create_framebuffers(self) -> RqResult<CommandBufferBuildStage>{
        let mut framebuffers = Vec::with_capacity(self.swap_chain.image_views.len());

        for imageView in self.swap_chain.image_views {
            let attachments = &[*imageView];
            let framebuffer_info = vk::FramebufferCreateInfo::builder()
                .render_pass(*self.render_pass)
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

        Result::Ok(CommandBufferBuildStage{
            entry: self.entry,
            instance: self.instance,
            messenger: self.messenger,
            physical_device: self.physical_device,
            logical_device: self.logical_device,
            queue_families: self.queue_families,
            surface: self.surface,
            swap_chain: self.swap_chain,
            render_pass: self.render_pass,
            pipelines: self.pipelines,
            framebuffers,
        })
    }
}