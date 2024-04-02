use vulkanalia::loader::LoaderError;
use vulkanalia::vk;
use winit::error::EventLoopError;
use crate::application::ApplicationError;
use vulkanalia::vk::ErrorCode;
use log::error;

#[derive(Debug)]
pub enum RenderingQueueError{
    LibLoadingError(libloading::Error),
    EntryCreateError(),
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

#[derive(Debug)]
pub enum CreateInstanceError{
    VulkanError(vk::ErrorCode),
    LayersError()
}

impl From<vk::ErrorCode> for CreateInstanceError{
    fn from (error: vk::ErrorCode) -> Self {
        CreateInstanceError::VulkanError(error)
    }
}
