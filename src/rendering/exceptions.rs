use std::io::Error;
use crate::rendering::RenderingQueueBuildError;

#[derive(Debug)]
pub enum RenderingError {
    LoadShadersError {
        error: Error,
        path_to_shader: String
    },
}