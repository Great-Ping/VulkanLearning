use application::ApplicationWindow;
use rendering::RenderingQueue;

mod application;
mod rendering;

fn main() {
    let window = ApplicationWindow::new();

    if let Err(err) = window{
        println!("{:?}", err);
        return;
    }

    RenderingQueue::new();

    let window = window.unwrap();
    //window.run();

} // drop(str2);
