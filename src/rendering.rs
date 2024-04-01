
use vulkanalia::window as vk_window;
use vulkanalia::vk::HasBuilder;
use vulkanalia::vk::make_version;
use vulkanalia::prelude::v1_0::{
    // Шаг первый, выбор экземпляра
    vk::Instance,
    vk::PhysicalDevice,
    // Шаг второй, выбор логических устройств и очередей
    vk::Device,
    vk::Queue,
    // Шаг третий, поверхность окна и цепочка подкачки
    vk::SurfaceKHR,
    vk::SwapchainKHR,
    // Шаг четвертый, просмотр изображений и фреймбуферы
    vk::ImageView,
    vk::Framebuffer,// Шаг пятный, проходы ренедеринга
    // Шаг шестой, графический конвеер (создается для каждого изменения с нуля)
    vk::Pipeline,
    vk::ShaderModule,
    // Шаг седьмой, пулы команд и командные буферы
    vk::CommandBuffer,
    vk::CommandPool,
    // Шаг восьмой, основной цикл
    vk::AcquireNextImageInfoKHR,
    // получение изображения из цепочки подкачки
    // Выбираем буффер комманд и выполняем queuesubmit

    // Возвращаем изображение в цепчочку подкачки, для представления с помощь
    // vk::QueuePresentKHR

    vk::ApplicationInfo
};

/**
    Чтобы нарисовать простой треугольник
    Начинаем прохождение рендеринга
    Привязка графического конвейера
    Нарисуйте 3 вершины
    Завершите прохождение рендеринга **/

struct RenderingQueue{
}

impl RenderingQueue {
  fn new() {
      let application_info = ApplicationInfo::builder()
          .application_name(b"Vulkan Learning\0")
          .application_version(make_version(1, 0, 0))
          .engine_name(b"Hello World Engine\0")
          .engine_version(make_version(1, 0, 0))
          .api_version(make_version(1, 0, 0));
  }
}