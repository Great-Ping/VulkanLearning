use vulkanalia::{
    Device,
    Entry,
    Instance
};
use vulkanalia::loader::{
    LibloadingLoader,
    LIBRARY
};
use vulkanalia::vk::{DebugUtilsMessengerEXT, Extent2D, Image, ImageView, PhysicalDevice, SurfaceKHR, SwapchainKHR};
use winit::raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle
};


use crate::rendering::{
    RenderingQueue,
    RenderingPipelineConfig,
    RenderingQueueBuildError::ErrorMessage
};

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
            entry
        })
    }
}

pub struct EndBuildStage {
    pub entry: Entry,
    pub instance: Instance,
    pub messenger: Option<DebugUtilsMessengerEXT>,
    pub physical_device: PhysicalDevice,
    pub logical_device: Device,
    pub queue_families: QueueFamilyIndices,
    pub surface: SurfaceKHR,
    pub swap_chain: SwapchainKHR,
    pub swap_chain_extent: Extent2D,
    pub swap_chain_images: Vec<Image>,
    pub swap_chain_image_views: Vec<ImageView>
}

impl EndBuildStage {
    pub fn build(self) -> RenderingQueue {
        return RenderingQueue::new (
            self.entry,
            self.instance,
            self.messenger,
            self.physical_device,
            self.logical_device,
            self.queue_families,
            self.surface,
            self.swap_chain,
            self.swap_chain_extent,
            self.swap_chain_images,
            self.swap_chain_image_views
        )
    }
}

impl RenderingQueue {

    pub fn builder(
    ) -> RenderingQueueBuilder {
        return RenderingQueueBuilder::new();
    }
}