use log::debug;
use winit::raw_window_handle::HasWindowHandle;
use vulkanalia::Entry;
use vulkanalia::loader::{
    LibloadingLoader,
    LIBRARY
};

use vulkanalia::{
    Instance,
    Device
};
use vulkanalia::vk::{DebugUtilsMessengerEXT, DeviceV1_0, ExtDebugUtilsExtension, InstanceV1_0, PhysicalDevice};

use super::RenderingQueueError::{
    EntryCreateError
};
use super::{
    RenderingQueueError,
};
use super::vulkan_tools::{
    get_debug_info,
    create_messenger
};
use super::vulkan_tools::{
    PhysicalDeviceInfo,
    create_instance,
    pick_physical_device,
    create_logical_device,
};

#[derive(Debug)]
pub struct RenderingQueue{
    entry: Entry,
    instance: Instance,
    messenger: Option<DebugUtilsMessengerEXT>,
    physical_device: PhysicalDevice,
    logical_device: Device,
}

impl RenderingQueue {
    pub unsafe fn new(
        window: &dyn HasWindowHandle
    ) -> Result<RenderingQueue, RenderingQueueError> {

        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader)
            .map_err(|err| EntryCreateError)?;

        let mut debug_info = get_debug_info();
        let instance = create_instance(
            window,
            &entry,
            &mut debug_info
        )?;
        let messenger = create_messenger(
            &instance,
            &debug_info
        );

        let physical_device = pick_physical_device(&instance)?;
        let logical_device = create_logical_device(
            &entry,
            &instance,
            physical_device,
        )?;

        Result::Ok(RenderingQueue{
            entry,
            instance,
            messenger,
            physical_device,
            logical_device
        })
    }
}

impl Drop for RenderingQueue{
    fn drop(&mut self){
        unsafe {
            if let Some(messenger) = self.messenger {
                self.instance.destroy_debug_utils_messenger_ext(messenger, None);
            }
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
        debug!("instance destroyed");
    }
}