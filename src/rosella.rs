use ash::Entry;
use crate::init::device::DeviceBuilder;
use crate::init::initialization_registry::InitializationRegistry;
use crate::init::instance_builder::InstanceBuilder;
use crate::window::{RosellaSurface, RosellaWindow};

pub struct Rosella {}

impl Rosella {
    pub fn new(mut registry: InitializationRegistry, window: &RosellaWindow, application_name: &str) -> Rosella {
        let now = std::time::Instant::now();
        let instance = InstanceBuilder::new(&registry).build(application_name, 0, window);

        let surface = RosellaSurface::new(&instance, &Entry::new(), window);

        let mut device_builder = DeviceBuilder {
            instance,
        };
        device_builder.build(&mut registry.required_features, &surface);

        let elapsed = now.elapsed();
        println!("Instance & Device Initialization took: {:.2?}", elapsed);


        /*        let vk = Entry::new();
                let app_name = CString::new(application_name);
                let surface_extensions = ash_window::enumerate_required_extensions(&window.handle).unwrap();
                let mut extension_names_raw = surface_extensions
                    .iter()
                    .map(|ext| ext.as_ptr())
                    .collect::<Vec<_>>();
                extension_names_raw.push(DebugUtils::name().as_ptr());

                let debug_utils_loader = DebugUtils::new(&vk, &instance);

                unsafe {
                    let debug_call_back = debug_utils_loader
                        .create_debug_utils_messenger(&debug_info, None)
                        .unwrap();
                }*/

        Rosella {}
    }

    pub fn window_update(&self) {

    }
}