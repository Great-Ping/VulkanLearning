use vulkanalia::vk::{
    HasBuilder,
    DeviceQueueCreateInfo,
    QueueFlags,
    DeviceCreateInfo,
    PhysicalDevice,
    PhysicalDeviceFeatures
};
use vulkanalia::{
    Entry,
    Instance,
    Device
};
use super::CreateLogicalDeviceError::CreateDeviceError;
use super::{
    CreateLogicalDeviceError,
    PhysicalDeviceInfo
};

pub unsafe fn create_logical_device(
    entry: &Entry,
    instance: &Instance,
    physical_device: PhysicalDevice,
    physical_device_info: &PhysicalDeviceInfo
) -> Result<Device, CreateLogicalDeviceError> {
    let queue_index = physical_device_info
        .get_queue_index(QueueFlags::GRAPHICS).unwrap();

    let queue_priorities = [1.0];
    let queue_info = DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_index)
        .queue_priorities(&queue_priorities)
        .build();
    let queue_infos = [queue_info];

    let layers = [];
    let extensions = [];
    let feauters = PhysicalDeviceFeatures::builder()
        .build();

    let device_info = DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&feauters);

    let device = instance.create_device(physical_device, &device_info, None)
        .map_err(|err|CreateDeviceError(err))?;

    Result::Ok(device)
}