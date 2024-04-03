use std::ffi::{c_void, CStr};
use log::{debug, error, info, trace, warn};
use vulkanalia::{Instance, vk};
use vulkanalia::vk::{Bool32, DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessengerEXT, ExtDebugUtilsExtension, ExtensionName};
use vulkanalia::vk::{
    DebugUtilsMessageSeverityFlagsEXT as SeverityFlagsEXT,
    DebugUtilsMessageTypeFlagsEXT as TypeFlagsEXT,
    DebugUtilsMessengerCallbackDataEXT as CallbackDataEXT,
    DebugUtilsMessengerCreateInfoEXT as CreateInfoEXT,
    DebugUtilsMessengerEXT as MessengerEXT,
    HasBuilder
};

pub const VALIDATION_ENABLED:bool =
    cfg!(debug_assertions);
pub const VALIDATION_LAYER: ExtensionName =
    ExtensionName::from_bytes(
        b"VK_LAYER_KHRONOS_validation"
    );


// vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
// – Произошло какое-то событие, не связанное со спецификацией или производительностью
// vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
// – Произошло что-то, что нарушает спецификацию или указывает на возможную ошибку
// vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
// – Потенциальное неоптимальное использование Vulkan
pub extern "system" fn debug_callback(
    severity: SeverityFlagsEXT,
    type_flags: TypeFlagsEXT,
    data: *const CallbackDataEXT,
    _: *mut c_void // Указатель для передачи каких то данных
) -> Bool32 {
    let data = unsafe {
        *data
    };
    let message = unsafe {
        CStr::from_ptr(data.message)
    }.to_string_lossy();

    if severity >= SeverityFlagsEXT::ERROR {
        error!("({:?}) {}", type_flags, message);
    } else if severity >= SeverityFlagsEXT::WARNING {
        warn!("({:?}) {}", type_flags, message);
    } else if severity >= SeverityFlagsEXT::INFO {
        debug!("({:?}) {}", type_flags, message)
    } else /*if severity >= SeverityFlagsEXT::VERBOSE*/{
        trace!("({:?}) {}", type_flags, message)
    }

    return vk::FALSE;
}


pub fn get_debug_info() -> Option<CreateInfoEXT>{
    if !VALIDATION_ENABLED{
        return None
    }

    let info = CreateInfoEXT::builder()
        .message_severity(SeverityFlagsEXT::all())
        .message_type(TypeFlagsEXT::all())
        .user_callback(Some(debug_callback))
        .build();

    return Some(info);
}

pub fn create_messenger(
    instance: &Instance,
    debug_info: &Option<CreateInfoEXT>
) -> Option<MessengerEXT> {
    if let Some(debug_info) = debug_info{
        unsafe {
            instance.create_debug_utils_messenger_ext(debug_info, None).ok()
        }
    } else {
        None
    }
}