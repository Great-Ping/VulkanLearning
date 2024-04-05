use std::collections::HashSet;
use std::ffi::c_char;
use vulkanalia::vk::{
    HasBuilder,
    DeviceQueueCreateInfo,
    QueueFlags,
    DeviceCreateInfo,
    PhysicalDeviceFeatures,
    SurfaceKHR
};
use vulkanalia::{
    Entry,
    Instance,
    Device,
    vk
};
use super::CreateLogicalDeviceError::{CreateDeviceError, CreateQueueError};
use super::{
    CreateLogicalDeviceError,
    PhysicalDeviceInfo,
    VALIDATION_ENABLED,
    VALIDATION_LAYER
};

pub unsafe fn create_logical_device(
    entry: &Entry,
    instance: &Instance,
    surface: &SurfaceKHR,
    physical_device_info: &PhysicalDeviceInfo,
) -> Result<Device, CreateLogicalDeviceError> {
    let queue_infos = create_queue_infos(
        instance,
        physical_device_info,
        surface
    )?;

    let layers = get_layers();
    let extensions =[];
    let features = PhysicalDeviceFeatures::builder()
        .build();

    let device_info = DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features)
        .build();

    let device = instance.create_device(physical_device_info.device, &device_info, None)
        .map_err(|err|CreateDeviceError(err))?;

    Result::Ok(device)
}

unsafe fn get_layers(
) -> Vec<*const c_char>{
    if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    }
}


unsafe fn create_queue_infos(
    instance: &Instance,
    device_info: &PhysicalDeviceInfo,
    surface: &SurfaceKHR
) -> Result<Vec<DeviceQueueCreateInfo>, CreateLogicalDeviceError> {
    let graphics_queue_index = device_info
        .get_queue_index(QueueFlags::GRAPHICS)
        .ok_or(CreateQueueError)?;
    let present_queue_index = device_info
        .get_present_queue_index(instance, surface)
        .ok_or(CreateQueueError)?;

    let mut unique_indices = HashSet::new();
    unique_indices.insert(graphics_queue_index);
    unique_indices.insert(present_queue_index);

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


    Result::Ok(queue_infos)

}