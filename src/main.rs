use crate::application::ApplicationWindow;

mod application;
mod rendering;

fn main() {
    let window = ApplicationWindow::new();

    if let Err(err) = window{
        println!("{:?}", err);
        return;
    }
    println!("window created!");
    return;
    let window = window.unwrap();
    window.run();

} // drop(str2);
