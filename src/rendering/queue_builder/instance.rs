use std::collections::HashSet;
use core::ffi::c_char;

use winit::raw_window_handle::{
    HasDisplayHandle,
    HasWindowHandle
};

use vulkanalia::Entry;
use vulkanalia::window::{
    create_surface,
    get_required_instance_extensions
};

use vulkanalia::vk;
use vulkanalia::vk::{
    EntryV1_0,
    HasBuilder,
    InstanceCreateFlags,
    InstanceCreateInfo,
    ApplicationInfo,
    make_version,
    ExtDebugUtilsExtension
};
use crate::rendering::{RenderingError, RqResult};
use crate::rendering::RenderingError::{
    CreateInstanceError,
    SupportError
};

use super::{
    get_debug_info,
    PhysicalDeviceBuildStage,
    VALIDATION_LAYER};

pub struct InstanceBuildStage {
    pub entry: Box<Entry>,
}

impl InstanceBuildStage {
    pub fn create_instance<TWindow>(
        self,
        window: &TWindow,
        use_validation_layer: bool
    ) -> RqResult<PhysicalDeviceBuildStage>
    where TWindow: HasDisplayHandle + HasWindowHandle {

        let application_info = ApplicationInfo::builder()
            .application_name(b"Vulkan Learning\0")
            .application_version(make_version(1, 0, 0))
            .engine_name(b"No Engine\0")
            .engine_version(make_version(1, 0, 0))
            .api_version(make_version(1, 0, 0))
            .build();

        let extensions = get_extensions(window, use_validation_layer);
        let layers = unsafe {
            get_layers(&self.entry, use_validation_layer)?
        };

        let mut instance_info = InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers)
            .flags(InstanceCreateFlags::empty());


        let mut debug_info=  get_debug_info();

        if use_validation_layer {
            instance_info = instance_info.push_next(&mut debug_info);
        }


        let instance_info = instance_info.build();
        let instance = unsafe {
            self.entry.create_instance(&instance_info, None)
                .map_err(|err| CreateInstanceError(err))?
        };

        let messenger = if use_validation_layer {
            let messenger = unsafe {
                instance.create_debug_utils_messenger_ext(&debug_info, None)
                    .map_err(|err| CreateInstanceError(err))?
            };
            Some(messenger)
        } else {
            None
        };

        let window_surface = unsafe {
            create_surface(&instance, window, window)
                .map_err(|err| CreateInstanceError(err))?
        };

        Result::Ok(PhysicalDeviceBuildStage {
            entry: self.entry,
            instance: Box::new(instance),
            messenger: messenger,
            surface: window_surface,
        })
    }
}

fn get_extensions<TWindow>(
    window: &TWindow,
    use_validation_layer: bool
) -> Vec<*const c_char>
where TWindow: HasWindowHandle {

    let mut extensions = get_required_instance_extensions(window)
        .iter()
        .map(|extension|extension.as_ptr())
        .collect::<Vec<_>>();

    if use_validation_layer {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr())
    }

    return extensions;
}

unsafe fn get_layers(
    entry: &Entry,
    use_validation_layer: bool
) -> RqResult<Vec<*const c_char>> {
    let layers = entry
        .enumerate_instance_layer_properties()
        .map_err(|err| CreateInstanceError(err))?;

    let available_layers = layers.iter()
        .map(|layer| layer.layer_name)
        .collect::<HashSet<_>>();

    if use_validation_layer {
        Result::Ok(Vec::new())
    } else if available_layers.contains(&VALIDATION_LAYER) {
        Result::Ok(vec![VALIDATION_LAYER.as_ptr()])
    } else {
        Result::Err(SupportError("Required layers is not supported"))
    }
}

