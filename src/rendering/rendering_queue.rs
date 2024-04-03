use winit::raw_window_handle::HasWindowHandle;
use vulkanalia::Entry;
use vulkanalia::loader::{
    LibloadingLoader,
    LIBRARY
};
use vulkanalia::Instance;
use vulkanalia::vk::{DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessengerEXT, ExtDebugUtilsExtension, InstanceV1_0};
use crate::rendering::CreateInstanceError::CreateDebuggerError;

use super::RenderingQueueError::{
    EntryCreateError
};
use super::{
    CreateInstanceError,
    RenderingQueueError,
    VALIDATION_ENABLED,
    get_debug_info,
    create_messenger
};
use super::vulkan_tools::{
    create_instance
};

#   [derive(Debug)]
pub struct RenderingQueue{
    entry: Entry,
    instance: Instance,
    messenger: Option<DebugUtilsMessengerEXT>
}

impl RenderingQueue {
    pub unsafe fn new(
        window: &dyn HasWindowHandle
    ) -> Result<RenderingQueue, RenderingQueueError> {

        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader)
            .map_err(|err| EntryCreateError)?;

        let mut debug_info = get_debug_info();
        let instance = create_instance(window, &entry, &mut debug_info)?;
        let messenger = create_messenger(&instance, &debug_info);

        Result::Ok(RenderingQueue{
            entry,
            instance,
            messenger
        })
    }
}

impl Drop for RenderingQueue{
    fn drop(&mut self){
        unsafe {
            if let Some(messenger) = self.messenger {
                self.instance.destroy_debug_utils_messenger_ext(messenger, None)
            }
            self.instance.destroy_instance(None);
        }
        println!("droped");
    }
}