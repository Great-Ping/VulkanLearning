use std::ffi::c_char;

use vulkanalia::prelude::v1_0::*;
use crate::rendering::{RenderingError, RqResult};
use crate::rendering::RenderingError::CreateLogicalDeviceError;

use super::{
    QueueFamilyIndices,
    SwapChainBuildStage,
    SwapСhainSupport,
};

use crate::rendering::{
    VALIDATION_LAYER
};

pub const REQUIRED_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];


#[derive(Debug)]
pub struct DeviceQueues{
    pub indices: QueueFamilyIndices,
    pub graphics: vk::Queue,
    pub present: vk::Queue
}

pub struct LogicalDeviceBuildStage {
    pub entry: Box<Entry>,
    pub messenger: Option<vk::DebugUtilsMessengerEXT>,
    pub instance: Box<Instance>,
    pub surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub queue_families: QueueFamilyIndices,
    pub swap_chain_support: Box<SwapСhainSupport>,
}

impl LogicalDeviceBuildStage {
    pub fn create_logical_device(
        self,
        use_validation_layer: bool
    ) -> RqResult<SwapChainBuildStage> {
        let queue_infos = unsafe {
            create_queue_infos(
                &self.queue_families
            )
        };

        let layers = get_layers(use_validation_layer);
        let extensions = get_extensions();
        let features = vk::PhysicalDeviceFeatures::builder();

        let device_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions)
            .enabled_features(&features)
            .build();

        let logical_device = unsafe {
            self.instance.create_device(self.physical_device, &device_info, None)
                .map_err(|err| CreateLogicalDeviceError(err))?
        };

        Result::Ok(SwapChainBuildStage {
            entry: self.entry,
            instance: self.instance,
            messenger: self.messenger,
            surface: self.surface,
            logical_device: Box::new(logical_device),
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
) -> Vec<vk::DeviceQueueCreateInfo> {
    let unique_indices = queue_indices.get_unique_indices();

    let queue_priorities = &[1.0];
    let queue_infos = unique_indices
        .iter()
        .map(|queue_index|{
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*queue_index)
                .queue_priorities(queue_priorities)
                .build()
        })
        .collect::<Vec<_>>();

    return queue_infos;
}