use std::fs;
use std::io::Read;
use crate::rendering::RenderingError;
use crate::rendering::RenderingError::LoadShadersError;

struct Shader {

}

impl Shader{
    fn read_as_bytes(path: &str, buffer: &mut Vec<u8>) -> Result<usize, RenderingError>{
        let mut shader_file = fs::File::open(path)
            .map_err(|err| LoadShadersError {
                path_to_shader: String::from(path), error: err
            })?;

        let file_metadata =  shader_file.metadata()
            .map_err(|err| LoadShadersError {
                path_to_shader: String::from(path), error: err
            })?;

        let file_size = file_metadata.len();

        let size = shader_file.read_to_end(buffer)
            .map_err(|err| LoadShadersError {
                path_to_shader: String::from(path), error: err
            })?;

        Result::Ok(size)
    }
}