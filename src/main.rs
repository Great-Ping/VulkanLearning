use std::env;
use std::fs::File;
use log::{debug, info, LevelFilter};
use application::ApplicationWindow;
use crate::rendering::{RenderingQueue, RenderingPipelineConfig, RenderingResolution};
use simple_logger::SimpleLogger;

mod application;
mod rendering;

fn main(){
    SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .init()
        .expect("logger initialization exception");

    log::set_max_level(LevelFilter::Info);

    let window =
        ApplicationWindow::new()
            .expect("window creation exception");

    let config = RenderingPipelineConfig{
        window: &window,
        use_validation_layer: cfg!(debug_assertions),
        rendering_resolution: RenderingResolution::from(window.inner_size())
    };

    let now = std::time::Instant::now();
    let rendering_queue = RenderingQueue::create(&config)
        .expect("rendering exception");

    let elapsed = now.elapsed();
    info!("Queue creation duration: {:?}", elapsed);

    rendering_queue.create_pipeline();

    window.run()
        .expect("main loop exception");

} // drop(str2);
