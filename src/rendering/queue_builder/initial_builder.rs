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
    InstanceBuilder,
    RenderingQueueBuildError,
    QueueFamilyIndices
};

pub struct InitialBuilder;

impl InitialBuilder {

    pub fn new() -> InitialBuilder {
        Self
    }

    pub fn create_entry(self) -> Result<InstanceBuilder, RenderingQueueBuildError>{
        let entry = unsafe {
            let loader = LibloadingLoader::new(LIBRARY)
                .map_err(|err| ErrorMessage("Load library error"))?;

            Entry::new(loader)
                .map_err(|_| ErrorMessage("Entry create error"))?
        };

        Result::Ok( InstanceBuilder {
            entry: Box::new(entry)
        })
    }
}

pub struct EndBuilder{
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
    pub physical_device: Box<vk::PhysicalDevice>,
    pub logical_device: Box<Device>,
    pub queue_families:QueueFamilyIndices,
    pub surface: Box<vk::SurfaceKHR>,
    pub swap_chain: Box<SwapChainData>
}

impl EndBuilder{
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
    ) -> InitialBuilder {
        return InitialBuilder::new();
    }
}