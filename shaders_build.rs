use std::collections::VecDeque;
use std::process::{Command};
use std::fs;
use std::env;
use std::fs::DirEntry;


struct ShadersCompiler {
    compiler_path: String,
    shaders_directory: String,
    output_directory: String
}

impl ShadersCompiler {
    fn compile_shader(
        &self,
        file: DirEntry,
        additional_output: &str
    ){
        let shader_path = file.path();
        let shader_path = shader_path.to_str().unwrap();

        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();

        let output_directory = concat_paths(
            &self.output_directory,
            additional_output
        );

        let output_file = concat_paths(
            &output_directory,
            &format!("{}.spv", file_name)
        );

        let output = Command::new(&self.compiler_path)
            .arg(shader_path)
            .args(&["-o", &output_file])
            .output()
            .expect(
                &format!(
                    "shader compilation failed. {}",
                    shader_path)
            );

        if output.stderr.len() > 0 {
            panic!("{:?}", output);
        }
    }

    fn compile_shaders(
        &self
    ) {
        let mut stack = VecDeque::new();
        stack.push_back(String::from(""));

        while stack.len() > 0 {
            let current_dir = stack.pop_back().unwrap();

            let read_directory = fs::read_dir(
                concat_paths(&self.shaders_directory, &current_dir)
            ).unwrap();

            create_output_directory_if_not_exists(
                concat_paths(
                    &self.output_directory,
                    &current_dir
                )
            );

            for dir_entry in read_directory{
                let dir_entry = dir_entry.unwrap();
                let file_type = dir_entry.file_type().unwrap();

                if file_type.is_file() {
                    self.compile_shader(dir_entry, &current_dir);
                } else {
                    let dir_name = dir_entry.file_name();
                    let dir_name = dir_name.to_str().unwrap();
                    stack.push_back(
                        concat_paths(&current_dir, dir_name)
                    )
                }
            }
        }
    }
}

fn concat_paths(path1: &str, path2: &str) -> String{
    if path1.ends_with("\\") || path2.starts_with("\\")
    {   format!("{0}{1}", path1, path2)}
    else
    {   format!("{0}\\{1}", path1, path2)}

}

fn create_output_directory_if_not_exists(directory_path: String) {
    fs::create_dir_all(directory_path).unwrap();
}


fn main(){

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile_dir = env::var("PROFILE").unwrap();
    let out_dir = concat_paths(&manifest_dir, "target");
    let out_dir = concat_paths(&out_dir, &profile_dir);


    let vulkan_sdk_path = env::var("VK_SDK_PATH").unwrap();
    let compiler_path = vulkan_sdk_path + "\\Bin\\glslc.exe";

    let shaders_directory = manifest_dir + "\\src\\assets\\shaders";
    let shaders_out_dir = out_dir + "\\assets\\shaders";

    println!("cargo::rerun-if-changed={}", shaders_directory);

    fs::remove_dir_all(&shaders_out_dir);

    let compiler = ShadersCompiler {
        compiler_path,
        shaders_directory,
        output_directory: shaders_out_dir
    };
    compiler.compile_shaders();
}
