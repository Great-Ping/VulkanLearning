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

use crate::rendering::RenderingPipelineConfig;
use crate::rendering::RenderingQueueBuildError::{
    ErrorCode,
    ErrorMessage
};

use super::{
    get_debug_info,
    PhysicalDeviceBuilder,
    RenderingQueueBuildError,
    VALIDATION_LAYER};


pub struct InstanceBuilder<'config, TWindow>
    where TWindow: HasDisplayHandle + HasWindowHandle {
    pub config: &'config RenderingPipelineConfig<TWindow>,
    pub entry: Entry,
}

impl<'config, TWindow> InstanceBuilder<'config, TWindow>
    where TWindow: HasDisplayHandle + HasWindowHandle {
    pub fn create_instance(self) -> Result<PhysicalDeviceBuilder<'config, TWindow>, RenderingQueueBuildError>{
        let application_info = ApplicationInfo::builder()
            .application_name(b"Vulkan Learning\0")
            .application_version(make_version(1, 0, 0))
            .engine_name(b"Hello World Engine\0")
            .engine_version(make_version(1, 0, 0))
            .api_version(make_version(1, 0, 0))
            .build();

        let extensions = get_extensions(self.config);
        let layers = unsafe { get_layers(&self.entry, self.config)?};

        let mut instance_info = InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers)
            .flags(InstanceCreateFlags::empty());


        let mut debug_info=  get_debug_info();

        if self.config.use_validation_layer {
            instance_info = instance_info.push_next(&mut debug_info);
        }


        let instance_info = instance_info.build();
        let instance = unsafe {
            self.entry.create_instance(&instance_info, None)
                .map_err(|err| ErrorCode(err))?
        };

        let messenger = if self.config.use_validation_layer {
            unsafe {
                Some(instance.create_debug_utils_messenger_ext(&debug_info, None)
                    .map_err(|err| ErrorCode(err))?)
            }
        } else {
            None
        };

        let window_surface = unsafe {
            create_surface(&instance, &self.config.window, &self.config.window)
                .map_err(|err| ErrorCode(err))?
        };

        Result::Ok(PhysicalDeviceBuilder {
            config: self.config,
            entry: self.entry,
            instance,
            messenger,
            surface: window_surface,
        })
    }
}

fn get_extensions<TWindow>(
    config: &RenderingPipelineConfig<TWindow>
) -> Vec<*const c_char>
    where TWindow: HasDisplayHandle + HasWindowHandle{
    let mut extensions = get_required_instance_extensions(&config.window)
        .iter()
        .map(|extension|extension.as_ptr())
        .collect::<Vec<_>>();

    if config.use_validation_layer {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr())
    }

    return extensions;
}

unsafe fn get_layers<TWindow>(
    entry: &Entry,
    config: &RenderingPipelineConfig<TWindow>
) -> Result<Vec<*const c_char>, RenderingQueueBuildError>
    where TWindow: HasDisplayHandle + HasWindowHandle{
    let layers = entry
        .enumerate_instance_layer_properties()
        .map_err(|err| ErrorCode(err))?;

    let available_layers = layers
        .iter()
        .map(|layer| layer.layer_name)
        .collect::<HashSet<_>>();

    if !config.use_validation_layer {
        Result::Ok(Vec::new())
    } else if available_layers.contains(&VALIDATION_LAYER) {
        Result::Ok(vec![VALIDATION_LAYER.as_ptr()])
    } else {
        Result::Err(ErrorMessage("Required layers is not supported"))
    }
}

