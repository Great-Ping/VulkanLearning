use std::collections::LinkedList;
use std::env;
use std::ops::Range;
use log::debug;
use winit::raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle
};
use vulkanalia::Entry;
use vulkanalia::loader::{
    LibloadingLoader,
    LIBRARY
};

use vulkanalia::{
    Instance,
    Device
};
use vulkanalia::bytecode::Bytecode;
use vulkanalia::window::create_surface;
use vulkanalia::vk;
use vulkanalia::vk::{
    DeviceV1_0,
    InstanceV1_0,
    ExtDebugUtilsExtension,
    Handle,
    HasBuilder,
    KhrSurfaceExtension,
    KhrSwapchainExtension
};
use winit::dpi::PhysicalSize;
use crate::rendering::RenderingError::CreatePipeLineError;

use super::shaders::Shader;
use super::{
    RenderingPipelineConfig,
    RenderingQueueBuildError,
    QueueFamilyIndices,
    RenderingError,
    get_debug_info
};

#[derive(Debug)]
pub struct RenderingQueue {
    entry: Entry,
    instance: Instance,
    messenger: Option<vk::DebugUtilsMessengerEXT>,
    physical_device: vk::PhysicalDevice,
    logical_device: Device,
    queue_families: QueueFamilyIndices,
    surface: vk::SurfaceKHR,
    swap_chain: vk::SwapchainKHR,
    swap_chain_extent: vk::Extent2D,
    swap_chain_images: Vec<vk::Image>,
    swap_chain_image_views: Vec<vk::ImageView>
}

impl RenderingQueue {

    pub fn new (
        entry: Entry,
        instance: Instance,
        messenger: Option<vk::DebugUtilsMessengerEXT>,
        physical_device: vk::PhysicalDevice,
        logical_device: Device,
        queue_families: QueueFamilyIndices,
        surface: vk::SurfaceKHR,
        swap_chain: vk::SwapchainKHR,
        swap_chain_extent: vk::Extent2D,
        swap_chain_images: Vec<vk::Image>,
        swap_chain_image_views: Vec<vk::ImageView>
    ) -> RenderingQueue {
        return RenderingQueue {
            entry,
            instance,
            messenger,
            physical_device,
            logical_device,
            queue_families,
            surface,
            swap_chain,
            swap_chain_images,
            swap_chain_extent,
            swap_chain_image_views
        }
    }

    pub fn create<TWindow>(
        config: &RenderingPipelineConfig<&TWindow>
    ) -> Result<RenderingQueue, RenderingQueueBuildError>
    where TWindow: HasWindowHandle+HasDisplayHandle {
        let pipeline = Self::builder()
            .create_entry()?
            .create_instance(&config.window, config.use_validation_layer)?
            .choose_physical_device()?
            .create_logical_device(config.use_validation_layer)?
            .create_swap_chain(&config.rendering_resolution, vk::SwapchainKHR::null())?
            .build();

        Result::Ok(pipeline)
    }

    fn create_shader_module(
        &self, bytecode: &[u8]
    ) -> Result<vk::ShaderModule, RenderingError>{
        let bytecode = Bytecode::new(bytecode)
            .map_err(|err|
                CreatePipeLineError("Bytecode error"))?;

        let shader_module_info = vk::ShaderModuleCreateInfo::builder()
            .code_size(bytecode.code_size())
            .code(bytecode.code())
            .build();

        let shader_module = unsafe {
            self.logical_device.create_shader_module(&shader_module_info, None)
                .map_err(|err| CreatePipeLineError("create shaders module error"))?
        };

        Result::Ok(shader_module)
    }

    pub fn create_pipeline(
        &self
    ) -> Result<vk::PipelineLayout, RenderingError>{
        let mut path_to_shaders = env::current_exe()
            .map_err(|err| CreatePipeLineError("Could not get path to executable file"))?;
        path_to_shaders.pop();
        path_to_shaders.push("assets\\shaders");

        let mut buffer = Vec::new();

        let mut path_to_shader  = path_to_shaders.clone();
        path_to_shader.push("Example.frag.spv");

        let file_size = Shader::read_file(&path_to_shader, &mut buffer)?;
        let fragment_shader = self.create_shader_module(&buffer[..file_size])?;

        let mut path_to_shader  = path_to_shaders.clone();
        path_to_shader.push("Example.vert.spv");

        let file_size = Shader::read_file(&path_to_shader, &mut buffer)?;
        let vertex_shader = self.create_shader_module(&buffer[..file_size])?;


        let vertex_shader_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vertex_shader)
            .name(b"main\0")
            .build();

        let fragment_shader_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(fragment_shader)
            .name(b"main\0")
            .build();

        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default();

        let vertex_input_stage = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
            .build();

        let viewport = vk::Viewport::builder()
            .x(0.0)
            .y(0.0)
            .width(self.swap_chain_extent.width as f32)
            .height(self.swap_chain_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .build();

        let rasteriation_state = vk::PipelineRasterizationStateCreateInfo::builder()
            //| if true Фрагменты за ближней и дальней областью не отбрасываются
            .depth_clamp_enable(false)
            //| if true скипаем этап растрирования
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            //| Обрабокта граней
            .cull_mode(vk::CullModeFlags::BACK)
            //| Порядок вершин по/против часовой
            .front_face(vk::FrontFace::CLOCKWISE)
            //  Разрешение изменение значений глубины
            .depth_bias_enable(false)
            .build();

        //Сглаживание
        let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::_1);


        // Настроить буфер глубины
        // PipelineDepthStencilStateCreateInfo

        //Смешивание цветов без учета альфа канала
        let attachment = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(vk::ColorComponentFlags::all())
            .blend_enable(false)
            .src_color_blend_factor(vk::BlendFactor::ONE)  // Optional
            .dst_color_blend_factor(vk::BlendFactor::ZERO) // Optional
            .color_blend_op(vk::BlendOp::ADD)              // Optional
            .src_alpha_blend_factor(vk::BlendFactor::ONE)  // Optional
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO) // Optional
            .alpha_blend_op(vk::BlendOp::ADD)              // Optional
            .build();

        let attachments = &[attachment];

        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0])
            .build();

        let dynamic_states = &[
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::LINE_WIDTH
        ];

        let dynamic_state = vk::PipelineDynamicStateCreateInfo::builder()
            .dynamic_states(dynamic_states)
            .build();

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default();

        let pipeline_layout = unsafe {
            self.logical_device.create_pipeline_layout(
                &pipeline_layout_info,
                None
            ).map_err(|err| CreatePipeLineError("Create layout error"))?
        };

        Result::Ok(pipeline_layout)
    }
}

impl Drop for RenderingQueue {
    fn drop(&mut self){
        unsafe {
            if let Some(messenger) = self.messenger {
                self.instance.destroy_debug_utils_messenger_ext(messenger, None);
            }

            // self.logical_device.destroy_pipeline();

            for image_view in &self.swap_chain_image_views{
                self.logical_device.destroy_image_view(image_view.clone(), None);
            }

            self.logical_device.destroy_swapchain_khr(self.swap_chain, None);

            self.instance.destroy_surface_khr(self.surface, None);
            self.logical_device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
        debug!("instance destroyed");
    }
}