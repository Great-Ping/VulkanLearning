use vulkanalia::{Device, Entry, Instance, vk};
use vulkanalia::vk::{CommandPool, DeviceV1_0, HasBuilder, RenderPass};
use crate::rendering::RqResult;

pub struct CommandBufferBuildStage {
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<vk::DebugUtilsMessengerEXT>,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: Box<Device>,
    pub queue_families: super::QueueFamilyIndices,
    pub surface: vk::SurfaceKHR,
    pub swap_chain: Box<super::SwapChainData>,
    pub render_pass: RenderPass,
    pub pipeline: vk::Pipeline,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub command_pool: CommandPool
}

impl CommandBufferBuildStage{
    fn create_command_buffer(self){

    }
}