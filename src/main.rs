use application::ApplicationWindow;
use crate::rendering::RenderingQueue;
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

    let rendering_queue = unsafe {
        RenderingQueue::new(&window, window.inner_size())
            .expect("rendering exception")
    };

    window.run()
        .expect("main loop exception");

} // drop(str2);
