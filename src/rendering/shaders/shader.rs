use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::rendering::RenderingError;
use crate::rendering::RenderingError::LoadShadersError;

pub struct Shader {

}

impl Shader{
    pub fn read_file(path: &PathBuf, buffer: &mut Vec<u8>) -> Result<usize, RenderingError>{
        let mut shader_file = fs::File::open(path)
            .map_err(|err| LoadShadersError {
                path_to_shader: String::from(path.to_str().unwrap()), error: err
            })?;

        let size = shader_file.read_to_end(buffer)
            .map_err(|err| LoadShadersError {
                path_to_shader: String::from(path.to_str().unwrap()), error: err
            })?;

        Result::Ok(size)
    }
}