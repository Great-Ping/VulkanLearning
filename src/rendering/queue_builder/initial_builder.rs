use vulkanalia::{
    Device,
    Entry,
    Instance
};
use vulkanalia::loader::{
    LibloadingLoader,
    LIBRARY
};
use vulkanalia::vk;

use crate::rendering::{
    RenderingQueue,
    RqResult
};
use crate::rendering::RenderingError::{
    CreateEntryError,
    LoadLibraryError
};

use super::{
    InstanceBuildStage,
    QueueFamilyIndices,
    SwapChainData
};

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