use application::ApplicationWindow;
use crate::rendering::{RenderingPipeline, RenderingPipelineConfig, RenderingResolution};
use simple_logger::SimpleLogger;

mod application;
mod rendering;

fn main(){
    SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .init()
        .expect("logger initialization exception");

    let window =
        ApplicationWindow::new()
            .expect("window creation exception");

    let config = RenderingPipelineConfig{
        window: &window,
        use_validation_layer: cfg!(debug_assertions),
        rendering_resolution: RenderingResolution::from(window.inner_size())
    };

    let rendering_queue = RenderingPipeline::create(&config)
        .expect("rendering exception");

    window.run()
        .expect("main loop exception");

} // drop(str2);
