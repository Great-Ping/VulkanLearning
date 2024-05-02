use vulkanalia::{Device, vk};
use vulkanalia::vk::{AttachmentLoadOp, AttachmentStoreOp, DeviceV1_0, Handle, HasBuilder, RenderPassCreateInfo};
use crate::rendering::RenderingError::{CreatePipelineLayoutError, CreateRenderPassError};

use super::{RqResult, SwapChainData};
use super::shaders::Shader;

struct PipelineBuilder{
    pub vertex_input_state: vk::PipelineVertexInputStateCreateInfo,
    pub input_assembly_state: vk::PipelineInputAssemblyStateCreateInfo,
    pub viewport_state: vk::PipelineViewportStateCreateInfo,
    pub rasterization_state: vk::PipelineRasterizationStateCreateInfo,
    pub multisample_state: vk::PipelineMultisampleStateCreateInfo,
    pub color_blend_state: vk::PipelineColorBlendStateCreateInfo,
    pub dynamic_state: vk::PipelineDynamicStateCreateInfo,
    pub pipeline_layout: vk::PipelineLayoutCreateInfo,
    pub render_pass: RenderPassCreateInfo,
    pub fragment_shader_stage: Option<vk::PipelineShaderStageCreateInfo>,
    pub vertex_shader_stage: Option<vk::PipelineShaderStageCreateInfo>
}

impl PipelineBuilder {
    fn default(swap_chain: &SwapChainData) -> Self {
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default();

        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
            .build();

        let viewport = vk::Viewport::builder()
            .x(0.0)
            .y(0.0)
            .width(swap_chain.extent.width as f32)
            .height(swap_chain.extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .build();

        let scissor = vk::Rect2D::builder()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(swap_chain.extent);

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

        let pipeline_layout = vk::PipelineLayoutCreateInfo::default();


        //Далее идет создание проходов рендеринга
        let render_pass = default_render_pass_create_info(swap_chain);

        return Self {
            vertex_input_state,
            input_assembly_state,
            viewport_state,
            rasterization_state,
            multisample_state,
            color_blend_state,
            dynamic_state,
            pipeline_layout,
            render_pass,

            fragment_shader_stage: Option::None,
            vertex_shader_stage: Option::None
        };
    }

    pub fn set_fragment_shader(
        mut self,
        shader: &Shader
    ) -> Self {
        let shader_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(shader.module.clone())
            .name(&shader.name)
            .build();

        self.fragment_shader_stage = Option::Some(shader_stage);

        return self;
    }

    pub fn set_vertex_shader(
        mut self,
        shader: &Shader
    ) -> Self {
        let shader_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(shader.module.clone())
            .name(&shader.name)
            .build();

        self.vertex_shader_stage = Option::Some(shader_stage);

        return self;
    }


    fn get_stages(&self) -> Vec<vk::PipelineShaderStageCreateInfo>{
        let mut stages = Vec::with_capacity(2);

        if let Some(stage) = self.vertex_shader_stage {
            stages.push(stage);
        }

        if let Some(stage) = self.fragment_shader_stage{
            stages.push(stage);
        }
        return stages;
    }


    fn build(
        self,
        logical_device: &Device
    ) -> RqResult<vk::GraphicsPipelineCreateInfo> {
        let pipeline_layout = unsafe {
            logical_device.create_pipeline_layout(&self.pipeline_layout, None)
                .map_err(|err| CreatePipelineLayoutError(err))?
        };

        let render_pass = unsafe {
            logical_device.create_render_pass(&self.render_pass, None)
                .map_err(|err|CreateRenderPassError(err))?
        };

        let pipline_stages = self.get_stages();
        let grahics_pipeline = vk::GraphicsPipelineCreateInfo::builder()
            .stages(pipline_stages.as_ref())
            .vertex_input_state(&self.vertex_input_state)
            .input_assembly_state(&self.input_assembly_state)
            .viewport_state(&self.viewport_state)
            .rasterization_state(&self.rasterization_state)
            .multisample_state(&self.multisample_state)
            .color_blend_state(&self.color_blend_state)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0)
            .base_pipeline_handle(vk::Pipeline::null())
            .base_pipeline_index(-1)
            .build();

        Result::Ok(grahics_pipeline)
    }
}

fn default_render_pass_create_info(
    swap_chain: &SwapChainData
) -> vk::RenderPassCreateInfo {
    let color_attachment = vk::AttachmentDescription::builder()
        .format(swap_chain.format)
        .samples(vk::SampleCountFlags::_1)
        //Определяем что делать с данными до рендеринга и после
        //Применяется к данным о цвете и глубине
        .load_op(vk::AttachmentLoadOp::CLEAR) //Отчистка фрейм буфера
        .store_op(vk::AttachmentStoreOp::STORE) // Сохраняем в памяти
        //Применяются к данным трафарета
        .stencil_load_op(AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(AttachmentStoreOp::DONT_CARE)
        //Макет изоражения до начала этапа рендеринга
        .initial_layout(vk::ImageLayout::UNDEFINED)
        //После
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR) // Изображения для SwapChain
        .build();

    let color_attachments = &[color_attachment];

    let color_attachment_ref = vk::AttachmentReference::builder()
        .attachment(0) //Индекс в массиве attachments (у нас 1 элемент тот что выше)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .build();

    let color_attachment_refs = &[color_attachment_ref];

    //Подпас графического типа
    let subpass = vk::SubpassDescription::builder()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(color_attachment_refs);

    let subpasses = &[subpass];

    let render_pass = vk::RenderPassCreateInfo::builder()
        .attachments(color_attachments)
        .subpasses(subpasses)
        .build();

    return render_pass;
}