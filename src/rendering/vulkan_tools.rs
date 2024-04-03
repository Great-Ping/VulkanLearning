use std::collections::HashSet;
use core::ffi::c_char;
use vulkanalia::Entry;
use vulkanalia::window::get_required_instance_extensions;

//Что-о-о-о?
use vulkanalia::{
    Instance, // Обертка инстанса вулкана
    Device,
};

use vulkanalia::vk;
use vulkanalia::vk::{EntryV1_0, HasBuilder};
use vulkanalia::vk::make_version;
use vulkanalia::vk::{
    // Шаг первый, выбор экземпляра
    // Instance, -> из вулканалии
    InstanceCreateFlags,
    PhysicalDevice,
    // Шаг второй, выбор логических устройств и очередей
    // Device, -> из вулканалии
    Queue,
    // Шаг третий, поверхность окна и цепочка подкачки
    SurfaceKHR,
    SwapchainKHR,
    // Шаг четвертый, просмотр изображений и фреймбуферы
    ImageView,
    Framebuffer,// Шаг пятный, проходы ренедеринга
    // Шаг шестой, графический конвеер (создается для каждого изменения с нуля)
    Pipeline,
    ShaderModule,
    // Шаг седьмой, пулы команд и командные буферы
    CommandBuffer,
    CommandPool,
    // Шаг восьмой, основной цикл
    AcquireNextImageInfoKHR,
    // получение изображения из цепочки подкачки
    // Выбираем буффер комманд и выполняем queuesubmit

    // Возвращаем изображение в цепчочку подкачки, для представления с помощь
    // vk::QueuePresentKHR
    InstanceCreateInfo,
    ApplicationInfo
};


use winit::raw_window_handle::HasWindowHandle;
use super::{VALIDATION_ENABLED, VALIDATION_LAYER};
use super::CreateInstanceError;
use super::CreateInstanceError::{
    VulkanError,
    LayersError
};

/**
Чтобы нарисовать простой треугольник
Начинаем прохождение рендеринга
Привязка графического конвейера
Нарисуйте 3 вершины
Завершите прохождение рендеринга **/

pub unsafe fn create_instance(
    window: &dyn HasWindowHandle,
    entry: &Entry
) -> Result<Instance, CreateInstanceError> {

    let application_info = ApplicationInfo::builder()
        .application_name(b"Vulkan Learning\0")
        .application_version(make_version(1, 0, 0))
        .engine_name(b"Hello World Engine\0")
        .engine_version(make_version(1, 0, 0))
        .api_version(make_version(1, 0, 0))
        .build();

    let extensinos = get_extensions(window)?;
    let layers = get_layers(entry)?;
    //Не для мака ягодка делана, нет флагов для поддержки мака

    let create_info = InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&extensinos)
        .enabled_layer_names(&layers)
        .flags(InstanceCreateFlags::empty())
        .build();

    Result::Ok(
        entry.create_instance(&create_info, None)?
    )
}

unsafe fn get_extensions(
    window: &dyn HasWindowHandle
) -> Result<Vec<*const c_char>, CreateInstanceError> {
    let extensions = get_required_instance_extensions(window)
        .iter()
        .map(|extension|extension.as_ptr())
        .collect::<Vec<_>>();

    Result::Ok(extensions)
}

unsafe fn get_layers(
    entry: &Entry
) -> Result<Vec<*const c_char>, CreateInstanceError>{
    let available_layers = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .map(|layer| layer.layer_name)
        .collect::<HashSet<_>>();

    if !VALIDATION_ENABLED {
        Result::Ok(Vec::new())
    } else if available_layers.contains(&VALIDATION_LAYER) {
        Result::Ok(vec![VALIDATION_LAYER.as_ptr()])
    } else {
        Result::Err(LayersError())
    }
}