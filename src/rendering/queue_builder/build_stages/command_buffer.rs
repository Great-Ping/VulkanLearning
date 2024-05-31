use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::PipelineLayout;
use crate::rendering::RenderingError::CreateCommandBufferError;
use crate::rendering::RqResult;
use super::SyncObjectsBuildStage;

pub struct CommandBufferBuildStage {
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<vk::DebugUtilsMessengerEXT>,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: Box<Device>,
    pub queue_families: super::QueueFamilyIndices,
    pub surface: vk::SurfaceKHR,
    pub swap_chain: Box<super::SwapChainData>,
    pub render_pass: vk::RenderPass,
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: PipelineLayout,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub command_pool: vk::CommandPool
}

impl CommandBufferBuildStage{
    pub fn create_command_buffer(self) -> RqResult<SyncObjectsBuildStage>{
        let allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(self.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(self.framebuffers.len() as u32)
            .build();

        let command_buffers = unsafe {
            self.logical_device.allocate_command_buffers(&allocate_info)
                .map_err(|err| CreateCommandBufferError(err))?
        };


        for (i, command_buffer) in command_buffers.iter().enumerate() {
            let inheritance = vk::CommandBufferInheritanceInfo::builder();

            let info = vk::CommandBufferBeginInfo::builder()
                .flags(vk::CommandBufferUsageFlags::empty()) // Optional.
                .inheritance_info(&inheritance);             // Optional.
            unsafe {
                //Запуск записи командного буфера
                self.logical_device.begin_command_buffer(*command_buffer, &info)
                    .map_err(|err| CreateCommandBufferError(err))?;
            }


            let render_area = vk::Rect2D::builder()
                .offset(vk::Offset2D::default())
                .extent(self.swap_chain.extent);

            let color_clear_value = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            };

            let clear_values = &[color_clear_value];
            let info = vk::RenderPassBeginInfo::builder()
                .render_pass(self.render_pass)
                .framebuffer(self.framebuffers[i])
                .render_area(render_area)
                .clear_values(clear_values);

            unsafe {
                self.logical_device.cmd_begin_render_pass(*command_buffer, &info, vk::SubpassContents::INLINE);
                self.logical_device.cmd_bind_pipeline(*command_buffer, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
                self.logical_device.cmd_draw(*command_buffer, 3, 1, 0, 0);
                self.logical_device.cmd_end_render_pass(*command_buffer);

                self.logical_device.end_command_buffer(*command_buffer)
                    .map_err(|err|CreateCommandBufferError(err))?;
            };
        }

        Result::Ok(SyncObjectsBuildStage{
            entry: self.entry,
            instance: self.instance,
            messenger: self.messenger,
            physical_device: self.physical_device,
            logical_device: self.logical_device,
            queue_families: self.queue_families,
            surface: self.surface,
            swap_chain: self.swap_chain,
            render_pass: self.render_pass,
            pipeline: self.pipeline,
            pipeline_layout: self.pipeline_layout,
            framebuffers: self.framebuffers,
            command_pool: self.command_pool,
            command_buffers
        })
    }
}