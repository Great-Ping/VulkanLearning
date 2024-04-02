use vulkanalia::vk::ExtensionName;

pub const VALIDATION_ENABLED:bool =
    cfg!(debug_assertions);
pub const VALIDATION_LAYER: ExtensionName =
    ExtensionName::from_bytes(
        b"VK_LAYER_KHRONOS_validation"
    );