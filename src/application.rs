use winit::dpi::LogicalSize;
use winit::error::{EventLoopError, OsError};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use crate::application::InitException::{EventLoopInitException, WindowInitException};

#[derive(Debug)]
pub struct ApplicationWindow {
    window: Window,
    event_loop: EventLoop<()>
}

#[derive(Debug)]
pub enum InitException{
    EventLoopInitException(EventLoopError),
    WindowInitException(OsError)
}

impl ApplicationWindow {
    pub fn new() -> Result<Self, InitException>{
        let event_loop = EventLoop::new();

        if let Err(err) = event_loop {
            return Result::Err(EventLoopInitException(err));
        }

        let event_loop= event_loop.unwrap();
        let window = WindowBuilder::new()
            .with_title("Vulkan")
            .with_inner_size(LogicalSize::new(1024, 768))
            .build(&event_loop);

        if let Err(err) = window{
            return Result::Err(WindowInitException(err))
        }

        let window = window.unwrap();

        Result::Ok(Self {
            window,
            event_loop
        })
    }

    pub fn run(self){
        self.event_loop.run(|event, target_window| {
            match event {
                Event::AboutToWait => self.window.request_redraw(),
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::RedrawRequested if !target_window.exiting() => println!("redraw"),

                    WindowEvent::CloseRequested => {
                        target_window.exit();
                    }
                    _ => {}
                }
                _ => {}
        }}).unwrap();
    }
}
