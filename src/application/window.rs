use log::debug;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{
    Event,
    WindowEvent
};
use winit::event_loop::{
    EventLoop,
    EventLoopWindowTarget
};
use winit::window::{
    Window,
    WindowBuilder
};
use winit::raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};

use super::ApplicationError;
use crate::pipeline_builder::RenderingQueue;

#[derive(Debug)]
pub struct ApplicationWindow {
    window: Window,
    event_loop: EventLoop<()>
}

impl ApplicationWindow {
    pub fn new() -> Result<Self, ApplicationError>{
        let event_loop = EventLoop::new()?;

        let window = WindowBuilder::new()
            .with_title("VulkanLearning")
            .with_inner_size(LogicalSize::new(1024, 768))
            .build(&event_loop)?;

        Result::Ok(Self {
            window,
            event_loop
        })
    }

    pub fn inner_size(&self) -> PhysicalSize<u32>{
        return self.window.inner_size() ;
    }

    pub fn run(self) -> Result<(), ApplicationError>{
        debug!("Starting main loop");
        self.event_loop.run(|event: Event<()>, target_window:&EventLoopWindowTarget<()>|{
            match event {
                Event::AboutToWait => self.window.request_redraw(),
                Event::WindowEvent {event, ..}  =>
                    processing_window_event(event, target_window),
                _ => {}
            }
        })?;
        return Result::Ok(());
    }
}

fn processing_window_event(
    event: WindowEvent,
    target_window: &EventLoopWindowTarget<()>
){
    match event {
        WindowEvent::RedrawRequested => {
            if !target_window.exiting() {
                //println!("redraw");
            }
        }
        WindowEvent::CloseRequested => {
            target_window.exit();
        }
        _ => {}
    }
}

impl HasWindowHandle for ApplicationWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.window.window_handle()
    }
}

impl HasDisplayHandle for ApplicationWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.window.display_handle()
    }
}