use vulkanalia::vk::ErrorCode;

#[derive(Debug)]
pub enum PipelineBuildError {
    ErrorMessage(&'static str),
    ErrorCode(ErrorCode)
}