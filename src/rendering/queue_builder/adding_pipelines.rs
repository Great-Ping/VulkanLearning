use std::collections::LinkedList;
use vulkanalia::{Device, Entry, Instance, vk};
use vulkanalia::vk::{DeviceV1_0, Handle, HasBuilder};
use crate::rendering::{FramebuffersBuildStage, PipelineBuilder, QueueFamilyIndices, RqResult, SwapChainData};
use crate::rendering::RenderingError::{BuildPipelinesError, CreatePipelineLayoutError};
use crate::rendering::shaders::Shader;

pub struct PipelineAddingStage{
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
    pub physical_device: Box<vk::PhysicalDevice>,
    pub logical_device: Box<Device>,
    pub queue_families:QueueFamilyIndices,
    pub surface: Box<vk::SurfaceKHR>,
    pub swap_chain: Box<SwapChainData>,
    pub render_pass: Box<vk::RenderPass>,
    pub pipelines: LinkedList<vk::Pipeline>
}


impl PipelineAddingStage{
    pub fn add_pipeline (
        mut self,
        vertex_shader: &Shader,
        fragment_shader: &Shader,
    ) -> RqResult<Self>
    {

        let vertex_shader_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vertex_shader.module.clone())
            .name(&vertex_shader.name.as_slice())
            .build();

        let fragment_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(fragment_shader.module.clone())
            .name(fragment_shader.name.as_slice())
            .build();

        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder().build()  ;

        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
            .build();

        let viewport = vk::Viewport::builder()
            .x(0.0)
            .y(0.0)
            .width(self.swap_chain.extent.width as f32)
            .height(self.swap_chain.extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .build();

        let scissor = vk::Rect2D::builder()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(self.swap_chain.extent);

        let viewports = &[viewport];
        let scissors = &[scissor];
        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(viewports)
            .scissors(scissors)
            .build();

        let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
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
            .rasterization_samples(vk::SampleCountFlags::_1)
            .build();


        // Настроить буфер глубины
        // PipelineDepthStencilStateCreateInfo

        //Смешивание цветов без учета альфа канала
        let attachment = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(vk::ColorComponentFlags::all())
            .blend_enable(false)
            // .src_color_blend_factor(vk::BlendFactor::ONE)
            // .dst_color_blend_factor(vk::BlendFactor::ZERO)
            // .color_blend_op(vk::BlendOp::ADD)
            // .src_alpha_blend_factor(vk::BlendFactor::ONE)
            // .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            // .alpha_blend_op(vk::BlendOp::ADD)
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

        let layout_info = vk::PipelineLayoutCreateInfo::builder().build();

        let pipeline_layout = unsafe {
            self.logical_device.create_pipeline_layout(&layout_info, None)
                .map_err(|err| CreatePipelineLayoutError(err))?
        };

        let pipeline_stages = &[vertex_shader_stage, fragment_stage];
        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(pipeline_stages.as_ref())
            .vertex_input_state(&vertex_input_state)
            .input_assembly_state(&input_assembly_state)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization_state)
            .multisample_state(&multisample_state)
            .color_blend_state(&color_blend_state)
            .layout(pipeline_layout)
            .render_pass(*self.render_pass)
            .subpass(0)
            .base_pipeline_handle(vk::Pipeline::null())
            .base_pipeline_index(-1)
            .build();


        let pipelines = unsafe {
            self.logical_device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[pipeline_info],
                None
            ).map_err(|err| BuildPipelinesError(err))?
        };

        let pipeline = pipelines.0[0];
        self.pipelines.push_back(pipeline);

        Result::Ok(self)
    }

    pub fn build_pipelines(self) -> RqResult<FramebuffersBuildStage> {
        let pipelines = self.pipelines.into_iter()
            .collect::<Vec<_>>();

        Result::Ok(FramebuffersBuildStage{
            entry: self.entry,
            instance: self.instance,
            messenger: self.messenger,
            physical_device: self.physical_device,
            logical_device: self.logical_device,
            queue_families: self.queue_families,
            surface: self.surface,
            swap_chain: self.swap_chain,
            render_pass: self.render_pass,
            pipelines: pipelines,
        })
    }
}