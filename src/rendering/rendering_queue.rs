use vulkanalia::Entry;
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::Instance;
use vulkanalia::vk::InstanceV1_0;
use winit::raw_window_handle::HasWindowHandle;
use crate::rendering::RenderingQueueError::EntryCreateError;

use super::{CreateInstanceError, RenderingQueueError};
use super::vulkan_tools::{
    create_instance
};

#[derive(Debug)]
pub struct RenderingQueue{
    entry: Entry,
    instance: Instance,
}

impl RenderingQueue {
    pub unsafe fn new(
        window: &dyn HasWindowHandle
    ) -> Result<RenderingQueue, RenderingQueueError> {

        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader);
        if entry.is_err(){
            return Result::Err(EntryCreateError());
        }
        let entry = entry.unwrap();

        let instance = create_instance(window, &entry)?;

        Result::Ok(RenderingQueue{
            entry, instance
        })
    }
}

impl Drop for RenderingQueue{
    fn drop(&mut self){
        unsafe {
            self.instance.destroy_instance(None);
        }
        println!("droped");
    }
}