//! Tracking manipulations.

use std::{collections::HashMap, mem::zeroed};

use moonlight_openvr::{
    system::{System, TrackedControllerRole},
    DeviceClass, Result as OvrResult, TrackedDeviceProperty, Universe, MAX_TRACKED_DEVICES,
};
use vek::{num_traits::FromPrimitive, Mat4, Vec3};

#[derive(Debug, Default)]
pub struct TrackingState {
    devices: HashMap<String, Device>,
}

impl TrackingState {
    /// Creates new `TrackingState`.
    pub fn new() -> TrackingState {
        Default::default()
    }

    pub fn tracked_devices(&self) -> impl Iterator<Item = &Device> {
        self.devices.values()
    }

    /// Updates tracked device statuses.
    pub fn update(&mut self, ovr_system: &System) -> OvrResult<()> {
        let mut raw_poses = vec![unsafe { zeroed() }; MAX_TRACKED_DEVICES];
        ovr_system.absolute_tracking_pose(Universe::Standing, 0.0, &mut raw_poses)?;

        for (index, raw_pose) in raw_poses.into_iter().enumerate() {
            let serial: String = match ovr_system
                .tracked_device_property(index, TrackedDeviceProperty::SerialNumberString)
            {
                Ok(s) => s,
                Err(_) => continue,
            };

            if let Some(found_device) = self.devices.get_mut(&serial) {
                found_device.update_pose(raw_pose.mDeviceToAbsoluteTracking.m);
            } else {
                let device_description = match ovr_system.tracked_device_class(index)? {
                    // Maybe should return error.
                    DeviceClass::Invalid => continue,

                    DeviceClass::HeadMountDisplay => DeviceDescription::HeadmountDisplay,
                    DeviceClass::Controller => {
                        let hand = ovr_system.tracked_device_property(
                            index,
                            TrackedDeviceProperty::ControllerRoleHintInt32,
                        )?;
                        let role =
                            FromPrimitive::from_i32(hand).expect("Invalid controller role hint");
                        DeviceDescription::Controller(role)
                    }
                    DeviceClass::GenericTracker => DeviceDescription::Tracker,
                    DeviceClass::TrackingReference => DeviceDescription::Reference,
                    DeviceClass::DisplayRedirect => continue,
                };
                let mut device = Device::new(serial.clone(), device_description);
                device.update_pose(raw_pose.mDeviceToAbsoluteTracking.m);
                self.devices.insert(serial, device);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Device {
    serial_number: String,
    pose: Mat4<f32>,
    description: DeviceDescription,
}

#[allow(dead_code)]
impl Device {
    /// Initializes `Device` with serial number and description.
    pub fn new(serial_number: impl Into<String>, description: DeviceDescription) -> Device {
        Device {
            pose: Mat4::default(),
            serial_number: serial_number.into(),
            description,
        }
    }

    /// Returns the serial number.
    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    /// Returns device description.
    pub fn description(&self) -> &DeviceDescription {
        &self.description
    }

    /// Updates the pose of device with Mat34.
    pub fn update_pose(&mut self, raw_pose: [[f32; 4]; 3]) {
        let mat = [raw_pose[0], raw_pose[1], raw_pose[2], [0.0, 0.0, 0.0, 1.0]];
        let mat = Mat4::from_row_arrays(mat);
        self.pose = mat;
    }

    /// Returns the device position.
    pub fn position(&self) -> Vec3<f32> {
        self.pose.cols[3].xyz()
    }
}

/// Describes further device information.
#[derive(Debug, Clone)]
pub enum DeviceDescription {
    /// HMD.
    HeadmountDisplay,

    /// Handheld controller with index.
    Controller(TrackedControllerRole),

    /// Generic tracker.
    Tracker,

    /// Tracking reference.
    Reference,
}
