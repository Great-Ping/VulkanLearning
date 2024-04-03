use vulkanalia::vk;
use super::vulkan_tools::CreateInstanceError;

#[derive(Debug)]
pub enum RenderingQueueError{
    LibLoadingError(libloading::Error),
    EntryCreateError,
    CreateInstanceError(CreateInstanceError),
}

impl From<libloading::Error> for RenderingQueueError {
    fn from(error: libloading::Error) -> Self {
        RenderingQueueError::LibLoadingError(error)
    }
}

impl From<CreateInstanceError> for RenderingQueueError {
    fn from(error: CreateInstanceError) -> Self {
        RenderingQueueError::CreateInstanceError(error)
    }
}
