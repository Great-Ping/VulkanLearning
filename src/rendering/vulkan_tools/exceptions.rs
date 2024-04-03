use vulkanalia::vk;

#[derive(Debug)]
pub enum CreateInstanceError{
    VulkanError(vk::ErrorCode),
    LayersError,
    CreateDebuggerError
}

impl From<vk::ErrorCode> for CreateInstanceError{
    fn from (error: vk::ErrorCode) -> Self {
        CreateInstanceError::VulkanError(error)
    }
}
