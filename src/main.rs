use libloading::Error;
use application::ApplicationWindow;
use rendering::RenderingQueue;
use crate::application::ApplicationError;

mod application;
mod rendering;

fn main(){
    let window = ApplicationWindow::new().unwrap();
    //window.run();

} // drop(str2);
