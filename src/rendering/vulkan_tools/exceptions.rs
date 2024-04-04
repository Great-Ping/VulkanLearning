use vulkanalia::vk;

#[derive(Debug)]
pub enum CreateInstanceError{
    LayersError,
    EntryError,
    CreateDebuggerError
}

#[derive(Debug)]
pub enum PickPhysicalDeviceError{
    SuitableDeviceNotFound,
    SuitabilityError (&'static str)
}