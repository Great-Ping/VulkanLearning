use std::ffi::OsStr;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk;
use vulkanalia::bytecode::Bytecode;
use vulkanalia::vk::{HasBuilder, ShaderModule};

use crate::rendering::RenderingError::LoadShadersError;
use crate::rendering::RqResult;

pub struct Shader {
    pub bytecode: Bytecode,
    pub name: Vec<u8>
}

impl Shader{
    pub fn read_file(
        path: &PathBuf,
        buffer: &mut Vec<u8>,
    ) -> RqResult<Self>{
        let mut shader_file = fs::File::open(path)
            .map_err(|err| LoadShadersError(format!("{}", err))
        )?;

        let file_size = shader_file.read_to_end(buffer)
            .map_err(|err| LoadShadersError(format!("{}", err)))?;

        let bytecode = Bytecode::new(&buffer[..file_size])
            .map_err(|err|
                LoadShadersError(format!("Bytecode error {}", err)))?;

        let file_name = get_file_name_or_default(path, "unknown");

        Result::Ok(Self{
            bytecode,
            name: file_name,
        })
    }
}



fn get_file_name_or_default<'b>(
    path_to_shader: &'b PathBuf,
    default: &str
) -> Vec<u8> {
    let default = OsStr::new(default);
    let shader_file_name = path_to_shader.file_name()
        .map_or(default, | value | default);

    let bytes_name = shader_file_name.as_encoded_bytes();
    return bytes_name.to_vec();
}
