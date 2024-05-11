use vulkanalia::{Device, Entry, Instance, vk};
use vulkanalia::vk::RenderPass;

pub struct CommandBufferBuildStage {
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
    pub physical_device: Box<vk::PhysicalDevice>,
    pub logical_device: Box<Device>,
    pub queue_families: super::QueueFamilyIndices,
    pub surface: Box<vk::SurfaceKHR>,
    pub swap_chain: Box<super::SwapChainData>,
    pub render_pass: Box<RenderPass>,
    pub pipelines: Vec<vk::Pipeline>,
    pub framebuffers: Vec<vk::Framebuffer>
}

