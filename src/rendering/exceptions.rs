use std::path::PathBuf;
use libloading::Error;
use vulkanalia::loader::LoaderError;
use vulkanalia::vk::{ErrorCode};


pub type RqResult<T> = Result<T, RenderingError>;

#[derive(Debug)]
pub enum RenderingError {
    LoadLibraryError(Error),
    CreateEntryError(Box<dyn LoaderError>),

    SupportError(&'static str),
    CreateInstanceError(ErrorCode),
    CreateLogicalDeviceError(ErrorCode),
    ChoosePhysicalDeviceError(ErrorCode),
    CreateSwapChainError(ErrorCode),
    CreatePipelineError(ErrorCode),
    LoadShadersError(String),

    CreateRenderPassError(ErrorCode),
    CreatePipelineLayoutError(ErrorCode),
    BuildPipelinesError(ErrorCode),
    CreateFrameBufferError(ErrorCode),
}