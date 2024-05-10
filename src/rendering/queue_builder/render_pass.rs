use std::collections::LinkedList;
use vulkanalia::{Device, Entry, Instance, vk};
use vulkanalia::vk::{AttachmentLoadOp, AttachmentStoreOp, DeviceV1_0, HasBuilder};
use super::PipelineAddingStage;
use crate::rendering::{QueueFamilyIndices, RqResult, SwapChainData};
use crate::rendering::RenderingError::CreateRenderPassError;

pub struct RenderPassBuildStage {
    pub entry: Box<Entry>,
    pub instance: Box<Instance>,
    pub messenger: Option<Box<vk::DebugUtilsMessengerEXT>>,
    pub physical_device: Box<vk::PhysicalDevice>,
    pub logical_device: Box<Device>,
    pub queue_families:QueueFamilyIndices,
    pub surface: Box<vk::SurfaceKHR>,
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
            render_pass: Box::new(render_pass),
            pipelines_info: LinkedList::default(),
        })
    }
}