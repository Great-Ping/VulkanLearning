use winit::dpi::LogicalSize;
use winit::error::{EventLoopError, OsError};
use winit::event;
use winit::event::{Event, WindowEvent};
use winit::event::WindowEvent::RedrawRequested;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};
use crate::rendering::RenderingQueue;

use super::ApplicationError;

#[derive(Debug)]
pub struct ApplicationWindow {
    window: Window,
    event_loop: EventLoop<()>,
    rendering_queue: RenderingQueue
}

impl ApplicationWindow {
    pub fn new() -> Result<Self, ApplicationError>{
        let event_loop = EventLoop::new()?;

        let window = WindowBuilder::new()
            .with_title("VulkanLearning")
            .with_inner_size(LogicalSize::new(1024, 768))
            .build(&event_loop)?;

        let rendering_queue = unsafe {
            RenderingQueue::new(&window)?
        };

        Result::Ok(Self {
            window,
            event_loop,
            rendering_queue
        })
    }

    pub fn run(self) -> Result<(), ApplicationError>{
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
                println!("redraw");
            }
        }
        WindowEvent::CloseRequested => {
            target_window.exit();
        }
        _ => {}
    }
}