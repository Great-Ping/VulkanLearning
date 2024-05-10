use std::collections::LinkedList;
use vulkanalia::{Device, Entry, Instance, vk};
use vulkanalia::vk::{DeviceV1_0, Handle};
use crate::rendering::{FramebuffersBuildStage, PipelineBuilder, QueueFamilyIndices, RqResult, SwapChainData};

pub struct PipelineAddingStage{
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
    pub physical_device: Box<vk::PhysicalDevice>,
    pub logical_device: Box<Device>,
    pub queue_families:QueueFamilyIndices,
    pub surface: Box<vk::SurfaceKHR>,
    pub swap_chain: Box<SwapChainData>,
    pub render_pass: Box<vk::RenderPass>,
    pub pipelines_info: LinkedList<vk::GraphicsPipelineCreateInfo>
}


impl PipelineAddingStage{
    fn add_pipeline<T>(
        mut self,
        pipeline_build: T
    ) -> RqResult<Self>
        where T: FnMut(PipelineBuilder, usize) -> PipelineBuilder
    {
        let mut pipeline_builder = PipelineBuilder::default(
            &self.swap_chain
        );
        pipeline_builder = pipeline_build(
            pipeline_builder,
            self.pipelines_info.iter().count()
        );

        let pipeline_info = pipeline_builder.build(
            &self.logical_device,
            &self.render_pass
        )?;

        self.pipelines_info.push_back(pipeline_info);

        Result::Ok(self)
    }

    fn build_pipelines(self) -> RqResult<FramebuffersBuildStage> {
        let pipelines = unsafe {
            self.logical_device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                self.pipelines_info.iter().collect::<Vec<_>>().as_ref(),
                None)?
        };

        Result::Ok(FramebuffersBuildStage{
            entry: self.entry,
            instance: self.instance,
            messenger: self.messenger,
            physical_device: self.physical_device,
            logical_device: self.logical_device,
            queue_families: self.queue_families,
            surface: self.surface,
            swap_chain: self.swap_chain,
            render_pass: self.render_pass,
            pipelines,
        })
    }
}