use vulkanalia::vk;
use vulkanalia::vk::ErrorCode;

#[derive(Debug)]
pub enum CreateInstanceError {
    LayersError,
    EntryError,
    CreateDebuggerError
}

#[derive(Debug)]
pub enum PickPhysicalDeviceError {
    SuitableDeviceNotFound,
    SuitabilityError (&'static str)
}

#[derive(Debug)]
pub enum CreateLogicalDeviceError {
    CreateDeviceError(ErrorCode),
    CreateQueueError
}

#[derive(Debug)]
pub enum CreateSwapchainSupportError{
    VulkanError(ErrorCode),
}

#[derive(Debug)]
pub enum PickSwapchainError{
    CreateSwapchainSupportError(CreateSwapchainSupportError),
    CreateSwapchainError(ErrorCode),
    ChooseFormatError,
}

impl From<CreateSwapchainSupportError> for PickSwapchainError{
    fn from(error: CreateSwapchainSupportError) -> Self {
        Self::CreateSwapchainSupportError(error)
    }
}