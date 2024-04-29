use vulkanalia::loader::{
    LibloadingLoader,
    LIBRARY
};
use vulkanalia::{
    Device,
    Entry,
    Instance
};
use vulkanalia::vk;

use crate::rendering::{RenderingQueue, RenderingQueueBuildError::ErrorMessage, SwapChainData};

use super::{
    InstanceBuildStage,
    RenderingQueueBuildError,
    QueueFamilyIndices
};

pub struct RenderingQueueBuilder;

impl RenderingQueueBuilder {

    pub fn new() -> RenderingQueueBuilder {
        Self
    }

    pub fn create_entry(self) -> Result<InstanceBuildStage, RenderingQueueBuildError>{
        let entry = unsafe {
            let loader = LibloadingLoader::new(LIBRARY)
                .map_err(|err| ErrorMessage("Load library error"))?;

            Entry::new(loader)
                .map_err(|_| ErrorMessage("Entry create error"))?
        };

        Result::Ok( InstanceBuildStage {
            entry: Box::new(entry)
        })
    }
}

pub struct EndBuildStage {
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
    pub physical_device: Box<vk::PhysicalDevice>,
    pub logical_device: Box<Device>,
    pub queue_families:QueueFamilyIndices,
    pub surface: Box<vk::SurfaceKHR>,
    pub swap_chain: Box<SwapChainData>
}

impl EndBuildStage {
    pub fn build(self) -> RenderingQueue {
        return RenderingQueue::new(
            self.entry,
            self.instance,
            self.messenger,
            self.physical_device,
            self.logical_device,
            self.queue_families,
            self.surface,
            self.swap_chain
        )
    }
}

impl RenderingQueue {

    pub fn builder(
    ) -> RenderingQueueBuilder {
        return RenderingQueueBuilder::new();
    }
}