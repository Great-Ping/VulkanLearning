use log::debug;
use winit::raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle
};
use vulkanalia::Entry;
use vulkanalia::loader::{
    LibloadingLoader,
    LIBRARY
};

use vulkanalia::{
    Instance,
    Device
};
use vulkanalia::window::create_surface;
use vulkanalia::vk::{
    DebugUtilsMessengerEXT,
    InstanceV1_0,
    DeviceV1_0,
    ExtDebugUtilsExtension,
    Handle,
    KhrSurfaceExtension,
    KhrSwapchainExtension,
    PhysicalDevice,
    SurfaceKHR,
    SwapchainKHR,
    Image,
};
use winit::dpi::PhysicalSize;

use super::{
    RenderingPipelineConfig,
    PipelineBuildError,
    QueueFamilyIndices,
    get_debug_info,
};

#[derive(Debug)]
pub struct RenderingPipeline {
    entry: Entry,
    instance: Instance,
    messenger: Option<DebugUtilsMessengerEXT>,
    physical_device: PhysicalDevice,
    logical_device: Device,
    queue_families: QueueFamilyIndices,
    surface: SurfaceKHR,
    swap_chain: SwapchainKHR,
    swap_chain_images: Vec<Image>
}

//Todo RenderingQueueBuilder, RenderingQueueConfig

impl RenderingPipeline {

    pub fn new (
        entry: Entry,
        instance: Instance,
        messenger: Option<DebugUtilsMessengerEXT>,
        physical_device: PhysicalDevice,
        logical_device: Device,
        queue_families: QueueFamilyIndices,
        surface: SurfaceKHR,
        swap_chain: SwapchainKHR,
        swap_chain_images: Vec<Image>
    ) -> RenderingPipeline {
        return RenderingPipeline {
            entry,
            instance,
            messenger,
            physical_device,
            logical_device,
            queue_families,
            surface,
            swap_chain,
            swap_chain_images,
        }
    }

    pub fn create<TWindow>(
        config: &RenderingPipelineConfig<&TWindow>
    ) -> Result<RenderingPipeline, PipelineBuildError>
    where TWindow: HasWindowHandle+HasDisplayHandle {
        let pipeline = Self::builder(config)
            .create_entry()?
            .create_instance()?
            .choose_physical_device()?
            .create_logical_device()?
            .create_swap_chain(SwapchainKHR::null())?
            .build();
        
        Result::Ok(pipeline)
    }
}

impl Drop for RenderingPipeline {
    fn drop(&mut self){
        unsafe {
            if let Some(messenger) = self.messenger {
                self.instance.destroy_debug_utils_messenger_ext(messenger, None);
            }
            self.logical_device.destroy_swapchain_khr(self.swap_chain, None);
            self.instance.destroy_surface_khr(self.surface, None);
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
        debug!("instance destroyed");
    }
}