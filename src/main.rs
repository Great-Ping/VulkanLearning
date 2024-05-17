mod application;
mod rendering;

use std::env;
use log::LevelFilter;
use application::ApplicationWindow;
use simple_logger::SimpleLogger;

use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::{
    HasBuilder,
    KhrSurfaceExtension,
    KhrSwapchainExtension
};

use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use crate::rendering::RenderingError::LoadShadersError;
use crate::rendering::{RenderingResolution, RqResult};

use self::rendering::{
    RenderingQueue
};


pub fn create_rendering_queue<TWindow>(
    window: &TWindow,
    use_validation_layer: bool,
    rendering_resolution: RenderingResolution,
) -> RqResult<RenderingQueue>
    where TWindow: HasWindowHandle + HasDisplayHandle
{
    let rendering_queue = RenderingQueue::builder()
        .create_entry()?
        .create_instance(
            window,
            use_validation_layer
        )?
        .choose_physical_device(
            vk::PhysicalDeviceType::INTEGRATED_GPU
        )?
        .create_logical_device(
            use_validation_layer
        )?
        .create_swap_chain(
            rendering_resolution,
            vk::SwapchainKHR::null()
        )?
        .create_render_pass()?;

    let mut path_to = env::current_exe()
        .map_err(|err|LoadShadersError(String::from("Unable to get the path")))?;
    path_to.pop();
    path_to.push("assets/shaders");
    path_to.push("Example.frag.spv");
    let mut buffer = Vec::with_capacity(4096);
    let fragShader = crate::rendering::Shader::read_file(&path_to, &rendering_queue.logical_device, &mut buffer)?;
    path_to.pop();
    path_to.push("Example.vert.spv");
    let vertShader = crate::rendering::Shader::read_file(&path_to, &rendering_queue.logical_device, &mut buffer)?;

    let renedering_queue = rendering_queue
        .add_pipeline(&vertShader, &fragShader)?
        .create_framebuffers()?
        .create_command_pool()?
        .create_command_buffer()?
        .create_sync_objects()?
        .build();

    Result::Ok(renedering_queue)
}

fn main(){
    SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .init()
        .expect("logger initialization exception");

    log::set_max_level(LevelFilter::Trace);

    let window =
        ApplicationWindow::new()
            .expect("window creation exception");

    let rendering_queue = create_rendering_queue(
        &window,
        true,
        RenderingResolution::from(window.inner_size())
    ).expect("rendering queue create exception");

    window.run()
        .expect("main loop exception");

} // drop(str2);
