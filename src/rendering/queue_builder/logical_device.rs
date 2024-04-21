use std::ffi::c_char;
use winit::raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle
};

use vulkanalia::{
    Entry,
    Instance,
    vk
};
use vulkanalia::vk::{
    HasBuilder,
    DeviceQueueCreateInfo,
    DeviceCreateInfo,
    PhysicalDeviceFeatures,
    SurfaceKHR,
    ExtensionName,
    PhysicalDevice,
    DebugUtilsMessengerEXT
};

use crate::rendering::RenderingQueueBuildError::ErrorCode;
use crate::rendering::RenderingPipelineConfig;

use super::{
    RenderingQueueBuildError,
    QueueFamilyIndices,
    SwapChainBuildStage,
    SwapСhainSupport,
    VALIDATION_LAYER
};

pub const REQUIRED_EXTENSIONS: &[ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

pub struct LogicalDeviceBuildStage {
    pub entry: Entry,
    pub messenger: Option<DebugUtilsMessengerEXT>,
    pub instance: Instance,
    pub surface: SurfaceKHR,
    pub physical_device: PhysicalDevice,
    pub queue_families: QueueFamilyIndices,
    pub swap_chain_support: SwapСhainSupport,
}

impl LogicalDeviceBuildStage {
    pub fn create_logical_device(self, use_validation_layer: bool) -> Result<SwapChainBuildStage, RenderingQueueBuildError>{
        let queue_infos = unsafe {
            create_queue_infos(
                &self.queue_families
            )
        };

        let layers = get_layers(use_validation_layer);
        let extensions = get_extensions();
        let features = PhysicalDeviceFeatures::builder()
            .build();

        let device_info = DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions)
            .enabled_features(&features)
            .build();

        let logical_device = unsafe {
            self.instance.create_device(self.physical_device, &device_info, None)
                .map_err(|err| ErrorCode(err))?
        };

        Result::Ok(SwapChainBuildStage {
            entry: self.entry,
            instance: self.instance,
            messenger: self.messenger,
            surface: self.surface,
            logical_device,
            queue_families: self.queue_families,
            physical_device: self.physical_device,
            swap_chain_support: self.swap_chain_support,
        })
    }
}

fn get_extensions() -> Vec<*const c_char> {
    REQUIRED_EXTENSIONS
        .iter()
        .map(|name| name.as_ptr())
        .collect::<Vec<_>>()
}

fn get_layers(use_validation_layer: bool) -> Vec<*const c_char> {
    if use_validation_layer {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    }
}


unsafe fn create_queue_infos(
    queue_indices: &QueueFamilyIndices
) -> Vec<DeviceQueueCreateInfo> {
    let unique_indices = queue_indices.get_unique_indices();

    let queue_priorities = [1.0];
    let queue_infos = unique_indices
        .iter()
        .map(|queue_index|{
            DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_index.clone())
                .queue_priorities(&queue_priorities)
                .build()
        })
        .collect::<Vec<_>>();

    return queue_infos;
}