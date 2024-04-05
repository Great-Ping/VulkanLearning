use vulkanalia::vk;
use vulkanalia::vk::ErrorCode;
use super::vulkan_tools::{CreateInstanceError, CreateLogicalDeviceError};
use super::vulkan_tools::PickPhysicalDeviceError;

#[derive(Debug)]
pub enum RenderingQueueError{
    LibLoadingError(libloading::Error),
    EntryCreateError,
    CreateInstanceError(CreateInstanceError),
    PeckPhysicalDeviceError(PickPhysicalDeviceError),
    CreateLogicalDeviceError(CreateLogicalDeviceError),
    CreateSurfaceError(ErrorCode)
}

impl From<libloading::Error> for RenderingQueueError {
    fn from(error: libloading::Error) -> Self {
        Self::LibLoadingError(error)
    }
}

impl From<CreateInstanceError> for RenderingQueueError {
    fn from(error: CreateInstanceError) -> Self {

        Self::CreateInstanceError(error)
    }
}

impl From<PickPhysicalDeviceError> for RenderingQueueError{
    fn from(error: PickPhysicalDeviceError) -> Self {
        Self::PeckPhysicalDeviceError(error)
    }
}

impl From<CreateLogicalDeviceError> for RenderingQueueError{
    fn from(error: CreateLogicalDeviceError) -> Self {
        Self::CreateLogicalDeviceError(error)
    }
}