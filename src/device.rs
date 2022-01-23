use std::sync::Arc;

use ash::vk;

use crate::init::EnabledFeatures;
use crate::instance::InstanceContext;
use crate::util::extensions::{AsRefOption, ExtensionFunctionSet, VkExtensionInfo, VkExtensionFunctions};
use crate::UUID;

pub struct DeviceContextImpl {
    instance: InstanceContext,
    device: ash::Device,
    physical_device: vk::PhysicalDevice,
    extensions: ExtensionFunctionSet,
    features: EnabledFeatures,
}

impl Drop for DeviceContextImpl {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}

#[derive(Clone)]
pub struct DeviceContext(Arc<DeviceContextImpl>);

impl DeviceContext {
    pub fn new(instance: InstanceContext, device: ash::Device, physical_device: vk::PhysicalDevice, extensions: ExtensionFunctionSet, features: EnabledFeatures) -> Self {
        Self(Arc::new(DeviceContextImpl{
            instance,
            device,
            physical_device,
            extensions,
            features,
        }))
    }

    pub fn get_entry(&self) -> &ash::Entry {
        self.0.instance.get_entry()
    }

    pub fn get_instance(&self) -> &InstanceContext {
        &self.0.instance
    }

    pub fn vk(&self) -> &ash::Device {
        &self.0.device
    }

    pub fn get_physical_device(&self) -> &vk::PhysicalDevice {
        &self.0.physical_device
    }

    pub fn get_extension<T: VkExtensionInfo>(&self) -> Option<&T> where VkExtensionFunctions: AsRefOption<T> {
        self.0.extensions.get()
    }

    pub fn is_extension_enabled(&self, uuid: UUID) -> bool {
        self.0.extensions.contains(uuid)
    }

    pub fn get_enabled_features(&self) -> &EnabledFeatures {
        &self.0.features
    }
}
