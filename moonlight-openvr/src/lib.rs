pub mod error;
pub mod openvr;
pub mod system;

pub use error::{Error, InitError, Result, TrackedPropertyError};
pub use openvr::{ApplicationType, Context};
pub use system::{DeviceClass, TrackedDeviceProperty, Universe};

/// Maximum tracked devices.
pub const MAX_TRACKED_DEVICES: usize = 64;

#[macro_export]
macro_rules! call_interface {
    ($method:expr, $($args: expr),*) => {
        match $method {
            Some(f) => unsafe { f($($args),*) },
            None => return Err(crate::error::Error::InvalidInterfaceMethod),
        }
    };
}
