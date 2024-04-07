use std::collections::HashSet;
use log::{debug, error, info};
use vulkanalia::{
    Device,
    Instance,
    vk
};
use vulkanalia::prelude::v1_0::InstanceV1_0;
use vulkanalia::vk::{
    ExtensionProperties,
    KhrSurfaceExtension,
    PhysicalDevice,
    PhysicalDeviceFeatures,
    PhysicalDeviceProperties,
    PhysicalDeviceType,
    QueueFamilyProperties,
    QueueFlags,
    SurfaceKHR,
    SwapchainKHR
};

use super::{
    PickPhysicalDeviceError,
    SwapСhainSupport,
    REQUIRED_EXTENSIONS,
};
use super::PickPhysicalDeviceError::{
    SuitableDeviceNotFound,
    SuitabilityError
};

#[derive(Debug)]
pub struct PhysicalDeviceInfo<'a> {
    pub device: PhysicalDevice,
    instance: &'a Instance,
    properties: PhysicalDeviceProperties,
    features: PhysicalDeviceFeatures,
    queue_family_properties: Vec<QueueFamilyProperties>,
}


impl<'a> PhysicalDeviceInfo<'a> {
    pub unsafe fn create(
        instance: &'a Instance,
        device: PhysicalDevice
    ) -> Self {
        //Имя, тип, поддерживаемая версия вулкан
        let device_properties = instance
            .get_physical_device_properties(device);
        //Поддержка сжатия текстур,  64- битные переоды,
        //Ренедринг с несколькими видовыми экранами
        let device_features = instance
            .get_physical_device_features(device);

        let queue_properties = instance
            .get_physical_device_queue_family_properties(device);

        return Self {
            device,
            instance,
            properties: device_properties,
            features: device_features,
            queue_family_properties: queue_properties
        };
    }

    pub fn get_queue_index(
        &self, flags: QueueFlags
    ) -> Option<u32> {
        self.queue_family_properties
            .iter()
            .position(|propery|
                propery.queue_flags
                    .contains(
                        flags
                    )
            ).map(|index| index as u32)
    }

    pub unsafe fn get_present_queue_index(
        &self,
        surface: &SurfaceKHR
    ) -> Option<u32> {
        let properties_enum = self.queue_family_properties
            .iter()
            .enumerate();
        for (index, properties) in properties_enum {
            let surface_support = self.instance.get_physical_device_surface_support_khr(
                self.device,
                index as u32,
                surface.clone()
            );

            if surface_support.is_err(){
                break
            }

            let surface_support = surface_support.unwrap();
            if surface_support {
                return Some(index as u32);
            }
        }

        return None;
    }

    unsafe fn check(&self, surface: &SurfaceKHR) ->  Result<(), PickPhysicalDeviceError>{
        self.check_physical_device()?;
        self.check_extensions_support()?;
        self.check_swapchain_support(surface)?;

        Result::Ok(())
    }

    unsafe fn check_swapchain_support(
        &self,
        surface: &SurfaceKHR
    )->  Result<(), PickPhysicalDeviceError>{
        let support = SwapСhainSupport::create(
            self.instance,
            surface,
            &self.device
        ).map_err(|err|SuitabilityError("swapchain is not supported"))?;

        if support.formats.is_empty() || support.present_modes.is_empty(){
            return Result::Err(SuitabilityError("swapchain is not supported"))
        }

        Result::Ok(())
    }

    fn check_physical_device(&self)->  Result<(), PickPhysicalDeviceError>{
        if self.properties.device_type != PhysicalDeviceType::DISCRETE_GPU {
            return Result::Err(SuitabilityError("device is not GPU."));
        }
        if self.features.geometry_shader != vk::TRUE{
            return Result::Err(SuitabilityError("missing geometry shaders support."));
        }

        let graphics_queue_index = self.get_queue_index(QueueFlags::GRAPHICS);
        if let None = graphics_queue_index {
            return Result::Err(SuitabilityError("missing graphics queue"));
        }
        Result::Ok(())
    }

    unsafe fn check_extensions_support(&self) -> Result<(), PickPhysicalDeviceError>{

        let extensions = self.instance
            .enumerate_device_extension_properties(self.device.clone(), None)
            .map_err(|error|SuitabilityError("сouldn't get extensions"))?;

        let extensions = extensions
            .iter()
            .map(|extension| extension.extension_name)
            .collect::<HashSet<_>>();

        if REQUIRED_EXTENSIONS.iter().all(|name|extensions.contains(name)) {
            Result::Ok(())
        }
        else {
            Result::Err(SuitabilityError("missing required device extensions"))
        }
    }

}


pub unsafe fn pick_physical_device<'a>(
    instance: &'a Instance,
    surface: &SurfaceKHR
)-> Result<PhysicalDeviceInfo<'a>, PickPhysicalDeviceError> {
    let devices =  instance
        .enumerate_physical_devices()
        .map_err(|err| SuitableDeviceNotFound)?;


    for device in devices{
        let device_info = PhysicalDeviceInfo::create(&instance, device);
        if device_info.check(surface).is_ok() {
            info!("Picked physical device {}", device_info.properties.device_name);
            return Result::Ok(device_info);
        }
    }

    Result::Err(SuitableDeviceNotFound)
}
