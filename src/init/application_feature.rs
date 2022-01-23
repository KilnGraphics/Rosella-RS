use std::any::Any;
use crate::init::{device, instance};
use crate::rosella::InstanceContext;
use crate::util::extensions::ExtensionFunctionSet;
use crate::UUID;


/// Common functions requires by all features
pub trait FeatureBase {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Represents the result of a init operation of a feature
pub enum InitResult {
    /// Indicates that the feature is supported and can be enabled
    Ok,

    /// Indicates that the feature is not supported and must be disabled
    Disable,
}

/// A feature that controls instance creation
///
/// See [`crate::init::instance`] for more information.
pub trait ApplicationInstanceFeature : FeatureBase {

    /// Tests if the feature is supported
    fn init(&mut self, features: &mut dyn FeatureAccess, info: &instance::InstanceInfo) -> InitResult;

    /// Configures the instance
    fn enable(&mut self, features: &mut dyn FeatureAccess, info: &instance::InstanceInfo, config: &mut instance::InstanceConfigurator);

    /// Performs any necessary post creation steps and generates the data that is sent back to the application
    fn finish(&mut self, _: &ash::Instance, _: &ExtensionFunctionSet) -> Option<Box<dyn Any>> {
        None
    }
}

/// A object that can generate [`ApplicationDeviceFeature`] instances used during the device
/// creation process.
///
/// See [`crate::init::device`] for more information.
pub trait ApplicationDeviceFeatureGenerator {

    /// Creates a new instance
    fn make_instance(&self) -> Box<dyn ApplicationDeviceFeature>;
}

/// A feature that controls device creation
///
/// See [`crate::init::device`] for more information.
pub trait ApplicationDeviceFeature: Send + FeatureBase {

    /// Tests if the feature is supported
    fn init(&mut self, features: &mut dyn FeatureAccess, info: &device::DeviceInfo) -> InitResult;

    /// Configures the device
    fn enable(&mut self, features: &mut dyn FeatureAccess, info: &device::DeviceInfo, config: &mut device::DeviceConfigurator);

    fn finish(&mut self, _: &InstanceContext, _: &ash::Device, _: &ExtensionFunctionSet) -> Option<Box<dyn Any>> {
        None
    }
}

/// Trait used by features to access their dependencies
pub trait FeatureAccess {
    fn get(&self, feature: &UUID) -> Option<&dyn Any>;

    fn get_mut(&mut self, feature: &UUID) -> Option<&mut dyn Any>;

    fn is_supported(&self, feature: &UUID) -> bool {
        self.get(feature).is_some()
    }
}
