use std::collections::HashSet;
use core::ffi::c_char;

use vulkanalia::Entry;
use vulkanalia::window::get_required_instance_extensions;
use winit::raw_window_handle::HasWindowHandle;

use vulkanalia::Instance;
use vulkanalia::vk;
use vulkanalia::vk::{
    Cast,
    EntryV1_0,
    HasBuilder,
    InstanceCreateFlags,
    InstanceCreateInfo,
    ApplicationInfo,
    ExtendsInstanceCreateInfo,
    make_version,
};

use super::{
    VALIDATION_ENABLED,
    VALIDATION_LAYER
};
use super::CreateInstanceError;
use super::CreateInstanceError::{
    EntryError,
    LayersError
};

pub unsafe fn create_instance<'b, T>(
    window: &dyn HasWindowHandle,
    entry: &Entry,
    next: &'b mut Option<impl Cast<Target = T>>
) -> Result<Instance, CreateInstanceError>
    where T : ExtendsInstanceCreateInfo {

    let application_info = ApplicationInfo::builder()
        .application_name(b"Vulkan Learning\0")
        .application_version(make_version(1, 0, 0))
        .engine_name(b"Hello World Engine\0")
        .engine_version(make_version(1, 0, 0))
        .api_version(make_version(1, 0, 0))
        .build();

    let extensions = get_extensions(window)?;
    let layers = get_layers(entry)?;
    //Не для мака ягодка делана, нет флагов для поддержки мака

    let mut instance_info = InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&extensions)
        .enabled_layer_names(&layers)
        .flags(InstanceCreateFlags::empty());

    if let Some(next) = next{
        instance_info = instance_info.push_next(next);
    }

    let instance_info = instance_info.build();
    let instance = entry.create_instance(&instance_info, None)
        .map_err(|err| EntryError)?;

    Result::Ok(instance)
}


unsafe fn get_extensions(
    window: &dyn HasWindowHandle
) -> Result<Vec<*const c_char>, CreateInstanceError> {
    let mut extensions = get_required_instance_extensions(window)
        .iter()
        .map(|extension|extension.as_ptr())
        .collect::<Vec<_>>();

    if VALIDATION_ENABLED {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr())
    }

    Result::Ok(extensions)
}

unsafe fn get_layers(
    entry: &Entry
) -> Result<Vec<*const c_char>, CreateInstanceError>{
    let layers = entry
        .enumerate_instance_layer_properties()
        .map_err(|err| LayersError)?;

    let available_layers = layers
        .iter()
        .map(|layer| layer.layer_name)
        .collect::<HashSet<_>>();

    if !VALIDATION_ENABLED {
        Result::Ok(Vec::new())
    } else if available_layers.contains(&VALIDATION_LAYER) {
        Result::Ok(vec![VALIDATION_LAYER.as_ptr()])
    } else {
        Result::Err(LayersError)
    }
}

