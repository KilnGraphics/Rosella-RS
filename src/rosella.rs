use crate::init::device::{create_device, DeviceCreateError};
use crate::init::initialization_registry::InitializationRegistry;
use crate::init::instance::{create_instance, InstanceCreateError};
use crate::window::{RosellaSurface, RosellaWindow};

use crate::init::rosella_features::WindowSurface;
use crate::objects::ObjectManager;

pub use crate::instance::VulkanVersion;
pub use crate::instance::InstanceContext;
pub use crate::device::DeviceContext;

pub struct Rosella {
    pub instance: InstanceContext,
    pub surface: RosellaSurface,
    pub device: DeviceContext,
    pub object_manager: ObjectManager,
}

#[derive(Debug)]
pub enum RosellaCreateError {
    InstanceCreateError(InstanceCreateError),
    DeviceCreateError(DeviceCreateError),
}

impl From<InstanceCreateError> for RosellaCreateError {
    fn from(err: InstanceCreateError) -> Self {
        RosellaCreateError::InstanceCreateError(err)
    }
}

impl From<DeviceCreateError> for RosellaCreateError {
    fn from(err: DeviceCreateError) -> Self {
        RosellaCreateError::DeviceCreateError(err)
    }
}

impl Rosella {
    pub fn new(mut registry: InitializationRegistry, window: &RosellaWindow, application_name: &str) -> Result<Rosella, RosellaCreateError> {
        log::info!("Starting Rosella");

        WindowSurface::register_into(&mut registry, &window.handle, true);

        let now = std::time::Instant::now();

        let instance = create_instance(&mut registry, application_name, 0)?;

        let surface = RosellaSurface::new(instance.vk(), &instance.get_entry(), window);

        let device = create_device(&mut registry, instance.clone())?;

        let elapsed = now.elapsed();
        println!("Instance & Device Initialization took: {:.2?}", elapsed);

        let object_manager = ObjectManager::new(device.clone());

        Ok(Rosella {
            instance,
            surface,
            device,
            object_manager,
        })
    }

    pub fn window_update(&self) {}

    pub fn recreate_swapchain(&self, width: u32, height: u32) {
        println!("resize to {}x{}", width, height);
    }
}