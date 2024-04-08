use vulkanalia::vk::ExtensionName;
use winit::dpi::PhysicalSize;
use winit::raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle
};

pub struct RenderingResolution {
    pub width: u32,
    pub height: u32
}

impl From<PhysicalSize<u32>> for RenderingResolution{
    fn from(size: PhysicalSize<u32>) -> Self {
        Self {
            width: size.width,
            height: size.height,
        }
    }
}

pub struct RenderingPipelineConfig<TWindow>
    where TWindow: HasWindowHandle + HasDisplayHandle {
    pub window: TWindow,
    pub use_validation_layer: bool,
    pub rendering_resolution: RenderingResolution,
}