use vulkanalia::Entry;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::vk::Instance;
use winit::raw_window_handle::HasWindowHandle;

use super::RenderingQueueError;
use super::vulkan_tools::{
    create_instance
};

pub struct RenderingQueue{
    entry: Entry,
    instance: Instance,
}

impl RenderingQueue {
    pub unsafe fn new(
        window: &dyn HasWindowHandle
    ) -> Result<RenderingQueue, RenderingQueueError> {

        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader)?;
        let instance = create_instance(window, &entry)?;

        Result::Ok(RenderingQueue{
            entry, instance
        })
    }
}