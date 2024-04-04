use vulkanalia::vk;
use super::vulkan_tools::CreateInstanceError;
use super::vulkan_tools::PickPhysicalDeviceError;

#[derive(Debug)]
pub enum RenderingQueueError{
    LibLoadingError(libloading::Error),
    EntryCreateError,
    CreateInstanceError(CreateInstanceError),
    PeckPhysicalDeviceError(PickPhysicalDeviceError)
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