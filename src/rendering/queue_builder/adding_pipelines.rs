use std::collections::LinkedList;
use std::io::SeekFrom;
use vulkanalia::{Device, Entry, Instance, vk};
use vulkanalia::vk::{DeviceV1_0, GraphicsPipelineCreateInfo, Handle};
use crate::rendering::{PipelineBuilder, QueueFamilyIndices, RqResult, SwapChainData};
use crate::rendering::queue_builder::initial_builder::EndBuildStage;

struct PipelineAddingStage{
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
    pub physical_device: Box<vk::PhysicalDevice>,
    pub logical_device: Box<Device>,
    pub queue_families:QueueFamilyIndices,
    pub surface: Box<vk::SurfaceKHR>,
    pub swap_chain: Box<SwapChainData>,
    pub pipelines_info: LinkedList<GraphicsPipelineCreateInfo>
}


impl PipelineAddingStage{
    fn add_pipeline<T>(
        mut self,
        pipeline_build: T
    ) -> RqResult<Self>
        where T: FnMut(PipelineBuilder, usize) -> PipelineBuilder
    {
        let mut pipeline_builder = PipelineBuilder::default(&self.swap_chain);
        pipeline_builder = pipeline_build(
            pipeline_builder,
            self.pipelines_info.iter().count()
        );

        let pipeline_info = pipeline_builder.build(
            &self.logical_device
        )?;

        self.pipelines_info.push_back(pipeline_info);

        Result::Ok(self)
    }

    fn build_pipelines(self) -> RqResult<EndBuildStage> {
        let pipelines = unsafe {
            self.logical_device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                self.pipelines_info.as_ref(),
                None)?
        };

        todo!()
    }
}