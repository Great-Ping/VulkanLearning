use vulkanalia::{
    Device,
    Entry,
    Instance
};
use vulkanalia::loader::{
    LibloadingLoader,
    LIBRARY
};
use vulkanalia::vk::{DebugUtilsMessengerEXT, Image, ImageView, PhysicalDevice, SurfaceKHR, SwapchainKHR};
use winit::raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle
};


use crate::rendering::{
    RenderingPipeline,
    RenderingPipelineConfig,
    PipelineBuildError::ErrorMessage
};

use super::{
    InstanceBuilder,
    PipelineBuildError,
    QueueFamilyIndices
};

pub struct InitialBuilder<'config, TWindow>
    where TWindow: HasWindowHandle + HasDisplayHandle {
    pub config: &'config RenderingPipelineConfig<TWindow>,
}

impl<'config, TWindow> InitialBuilder<'config, TWindow>
    where TWindow: HasWindowHandle+HasDisplayHandle {

    pub fn new(
        config: &'config RenderingPipelineConfig<TWindow>
    ) -> InitialBuilder<'config, TWindow> {
        Self {
            config
        }
    }

    pub fn create_entry(self) -> Result<InstanceBuilder<'config, TWindow>, PipelineBuildError>{
        let entry = unsafe {
            let loader = LibloadingLoader::new(LIBRARY)
                .map_err(|err| ErrorMessage("Load library error"))?;
            Entry::new(loader)
                .map_err(|_| ErrorMessage("Entry create error"))?
        };

        Result::Ok( InstanceBuilder {
            config: self.config,
            entry
        })
    }
}

pub struct EndBuilder {
    pub entry: Entry,
    pub instance: Instance,
    pub messenger: Option<DebugUtilsMessengerEXT>,
    pub physical_device: PhysicalDevice,
    pub logical_device: Device,
    pub queue_families: QueueFamilyIndices,
    pub surface: SurfaceKHR,
    pub swap_chain: SwapchainKHR,
    pub swap_chain_images: Vec<Image>,
    pub swap_chain_image_views: Vec<ImageView>
}

impl EndBuilder{
    pub fn build(self) -> RenderingPipeline {
        return RenderingPipeline::new (
            self.entry,
            self.instance,
            self.messenger,
            self.physical_device,
            self.logical_device,
            self.queue_families,
            self.surface,
            self.swap_chain,
            self.swap_chain_images,
            self.swap_chain_image_views
        )
    }
}

impl RenderingPipeline {

    pub fn builder<TWindow>(
        config: &RenderingPipelineConfig<TWindow>
    ) -> InitialBuilder<TWindow>
    where TWindow: HasWindowHandle+HasDisplayHandle {
        return InitialBuilder::new(config);
    }
}