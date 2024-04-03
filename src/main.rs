use application::ApplicationWindow;
use crate::rendering::RenderingQueue;

mod application;
mod rendering;

fn main(){
    let window =
        ApplicationWindow::new()
            .unwrap();

    let rendering_queue = unsafe {
        RenderingQueue::new(&window)
            .unwrap()
    };

    window.run().unwrap();

} // drop(str2);
