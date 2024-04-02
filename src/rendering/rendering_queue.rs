use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::vk::Instance;

pub struct RenderingQueue{
    instance: Instance
}

impl RenderingQueue {
    pub fn new() {
        let loader = unsafe {
            LibloadingLoader::new(LIBRARY);
        };

    }
}