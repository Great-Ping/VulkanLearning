use vulkanalia::Entry;
use vulkanalia::loader::LibloadingLoader;
use vulkanalia::loader::LIBRARY;
use vulkanalia::window::get_required_instance_extensions;

use vulkanalia::vk;
use vulkanalia::vk::HasBuilder;
use vulkanalia::vk::make_version;
use vulkanalia::vk::{
    // Шаг первый, выбор экземпляра
    Instance,
    PhysicalDevice,
    // Шаг второй, выбор логических устройств и очередей
    Device,
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

/**
Чтобы нарисовать простой треугольник
Начинаем прохождение рендеринга
Привязка графического конвейера
Нарисуйте 3 вершины
Завершите прохождение рендеринга **/

pub unsafe fn create_instance(
    window: &dyn HasWindowHandle,
    entry: &Entry
) -> Result<vulkanalia::Instance, vk::ErrorCode> {

    let application_info = ApplicationInfo::builder()
        .application_name(b"Vulkan Learning\0")
        .application_version(make_version(1, 0, 0))
        .engine_name(b"Hello World Engine\0")
        .engine_version(make_version(1, 0, 0))
        .api_version(make_version(1, 0, 0))
        .build();

    let extensinos = get_required_instance_extensions(window)
        .iter()
        .map(|extension|extension.as_ptr())
        .collect::<Vec<_>>();

    let create_info = InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&extensinos)
        //.enabled_layer_count(0)
        .build();

    return Result::Ok(entry.create_instance(&create_info, None)?);
}