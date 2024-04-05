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

pub enum CreateSwapchainSupportError{
    VulkanError(ErrorCode),
}