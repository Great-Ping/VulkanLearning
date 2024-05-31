use vulkanalia::prelude::v1_0::*;
use super::PipelineAddingStage;
use crate::rendering::{QueueFamilyIndices, RqResult, SwapChainData};
use crate::rendering::RenderingError::CreateRenderPassError;

pub struct RenderPassBuildStage {
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<vk::DebugUtilsMessengerEXT>,
    pub physical_device: vk::PhysicalDevice,
    pub logical_device: Box<Device>,
    pub queue_families: QueueFamilyIndices,
    pub surface: vk::SurfaceKHR,
    pub swap_chain: Box<SwapChainData>
}

impl RenderPassBuildStage{
    pub fn create_render_pass(self) -> RqResult<PipelineAddingStage> {
        //Далее идет создание проходов рендеринга
        let color_attachment = vk::AttachmentDescription::builder()
            .format(self.swap_chain.format)
            .samples(vk::SampleCountFlags::_1)
            //Определяем что делать с данными до рендеринга и после
            //Применяется к данным о цвете и глубине
            .load_op(vk::AttachmentLoadOp::CLEAR) //Отчистка фрейм буфера
            .store_op(vk::AttachmentStoreOp::STORE) // Сохраняем в памяти
            //Применяются к данным трафарета
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
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

        let dependency = vk::SubpassDependency::builder()
            .src_subpass(vk::SUBPASS_EXTERNAL) //  |неявный подпас
            .dst_subpass(0) //индекс подпаса       |до ренедринга
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .build();

        let dependencies = &[dependency];

        //Подпас графического типа
        let subpass = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(color_attachment_refs)
            .build();

        let subpasses = &[subpass];

        let render_pass = vk::RenderPassCreateInfo::builder()
            .attachments(color_attachments)
            .subpasses(subpasses)
            .dependencies(dependencies)
            .build();

        let render_pass = unsafe {
            self.logical_device.create_render_pass(&render_pass, None)
                .map_err(|err|CreateRenderPassError(err))?
        };

        Result::Ok(PipelineAddingStage{
            entry: self.entry,
            instance: self.instance,
            messenger: self.messenger,
            physical_device: self.physical_device,
            logical_device: self.logical_device,
            queue_families: self.queue_families,
            surface: self.surface,
            swap_chain: self.swap_chain,
            render_pass: render_pass
        })
    }
}