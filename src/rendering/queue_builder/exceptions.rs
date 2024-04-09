use vulkanalia::vk::ErrorCode;

#[derive(Debug)]
pub enum RenderingQueueBuildError {
    ErrorMessage(&'static str),
    ErrorCode(ErrorCode)
}