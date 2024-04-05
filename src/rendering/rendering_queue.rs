use log::debug;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
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
use vulkanalia::vk::{DebugUtilsMessengerEXT, DeviceV1_0, ExtDebugUtilsExtension, InstanceV1_0, KhrSurfaceExtension, PhysicalDevice, SurfaceKHR};
use super::RenderingQueueError;
use super::RenderingQueueError::{
    EntryCreateError,
    CreateSurfaceError
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
    surface: SurfaceKHR
}

impl RenderingQueue {
    pub unsafe fn new<TWindow>(
        window: &TWindow
    ) -> Result<RenderingQueue, RenderingQueueError>
    where TWindow: HasWindowHandle+HasDisplayHandle{

        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader)
            .map_err(|_| EntryCreateError)?;

        let mut debug_info = get_debug_info();
        let instance = create_instance(window, &entry,&mut debug_info)?;
        let messenger = create_messenger(&instance, &debug_info);

        let window_surface = create_surface(&instance, &window, &window)
            .map_err(|err| CreateSurfaceError(err))?;

        let physical_device_info = pick_physical_device(&instance)?;
        let physical_device = physical_device_info.device;

        let logical_device = create_logical_device(
            &instance,
            &window_surface,
            &physical_device_info
        )?;


        Result::Ok(RenderingQueue{
            entry,
            instance,
            messenger,
            physical_device,
            logical_device,
            surface: window_surface
        })
    }
}

impl Drop for RenderingQueue{
    fn drop(&mut self){
        unsafe {
            if let Some(messenger) = self.messenger {
                self.instance.destroy_debug_utils_messenger_ext(messenger, None);
            }
            self.instance.destroy_surface_khr(self.surface, None);
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
        debug!("instance destroyed");
    }
}