use std::io::Error;

#[derive(Debug)]
pub enum RenderingError {
    LoadShadersError {
        error: Error,
        path_to_shader: String
    },
    CreatePipeLineError(&'static str)
}