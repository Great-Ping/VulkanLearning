use std::ffi::OsStr;
use std::path::PathBuf;

use vulkanalia::{Device, vk};
use vulkanalia::bytecode::Bytecode;
use vulkanalia::vk::{DeviceV1_0, GraphicsPipelineCreateInfo, HasBuilder, ShaderModule};

use crate::rendering::{RenderingQueueBuildError, SwapChainData};
use crate::rendering::RenderingError::CreatePipeLineError;
use crate::rendering::RenderingQueueBuildError::ErrorMessage;
use crate::rendering::shaders::Shader;

struct PipelineBuilder<'b>{
    logical_device: &'b Device,
    swap_chain: &'b SwapChainData,

    pub vertex_input_state: vk::PipelineVertexInputStateCreateInfo,
    pub input_assembly_state: vk::PipelineInputAssemblyStateCreateInfo,
    pub viewport: vk::Viewport,
    pub rasterization_state: vk::PipelineRasterizationStateCreateInfo,
    pub multisample_state: vk::PipelineMultisampleStateCreateInfo,
    pub color_blend_state: vk::PipelineColorBlendStateCreateInfo,
    pub dynamic_state: vk::PipelineDynamicStateCreateInfo,
    pub pipeline_layout: vk::PipelineLayout,
    pub fragment_shader_stage: Option<vk::PipelineShaderStageCreateInfo>,
    pub vertex_shader_stage: Option<vk::PipelineShaderStageCreateInfo>
}

impl<'b> PipelineBuilder<'b> {

    fn default(logical_device: &'b Device, swap_chain: &'b SwapChainData) -> Result<Self, RenderingQueueBuildError> {
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

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default();

        let pipeline_layout = unsafe {
            logical_device.create_pipeline_layout(
                &pipeline_layout_info,
                None
            ).map_err(|err| ErrorMessage("Create layout error"))?
        };

        Result::Ok(Self {
            logical_device,
            swap_chain,

            vertex_input_state,
            input_assembly_state,
            viewport,
            rasterization_state,
            multisample_state,
            color_blend_state,
            dynamic_state,
            pipeline_layout,

            fragment_shader_stage: Option::None,
            vertex_shader_stage: Option::None
        })

    }

    fn set_fragment_shader(mut self, path_to_shader: &PathBuf, buffer: &mut Vec<u8>) -> Result<Self, RenderingQueueBuildError> {
        let default_name = OsStr::new("frag_shader");

        let shader = load_shader_module(self.logical_device, path_to_shader, buffer)?;

        let shader_name = get_file_name_or_default(path_to_shader, default_name);
        let shader_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(shader)
            .name(shader_name)
            .build();

        self.fragment_shader_stage = Option::Some(shader_stage);

        Result::Ok(self)
    }

    fn set_vertex_shader(mut self,path_to_shader: &PathBuf, buffer: &mut Vec<u8>) -> Result<Self, RenderingQueueBuildError> {
        let default_name = OsStr::new("vertex_shader");

        let shader = load_shader_module(self.logical_device, path_to_shader, buffer)?;

        let shader_name = get_file_name_or_default(path_to_shader, default_name);
        let shader_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(shader)
            .name(shader_name)
            .build();

        self.vertex_shader_stage = Option::Some(shader_stage);

        Result::Ok(self)
    }

    fn build() -> Result<GraphicsPipelineCreateInfo, RenderingQueueBuildError> {
        todo!()
    }
}
fn get_file_name_or_default<'b>(
    path_to_shader: &'b PathBuf,
    default: &'b OsStr
) -> &'b [u8]
{
    let shader_file_name = path_to_shader.file_name()
        .map_or(default, | value | value);

    return shader_file_name.as_encoded_bytes()
}

fn load_shader_module(
    logical_device: &Device,
    path_to_shader: &PathBuf,
    buffer: &mut Vec<u8>
) -> Result<ShaderModule, RenderingQueueBuildError> {
    let file_size = Shader::read_file(&path_to_shader, buffer)
        .map_err(|err| ErrorMessage("cannot read file"))?;
    let shader_module = create_shader_module(logical_device, &buffer[..file_size])?;

    Result::Ok(shader_module)
}

fn create_shader_module(
    logical_device: &Device, bytecode: &[u8]
) -> Result<vk::ShaderModule, RenderingQueueBuildError> {
    let bytecode = Bytecode::new(bytecode)
        .map_err(|err|
            ErrorMessage("Bytecode error"))?;

    let shader_module_info = vk::ShaderModuleCreateInfo::builder()
        .code_size(bytecode.code_size())
        .code(bytecode.code())
        .build();

    let shader_module = unsafe {
        logical_device.create_shader_module(&shader_module_info, None)
            .map_err(|err| ErrorMessage("create shaders module error"))?
    };

    Result::Ok(shader_module)
}