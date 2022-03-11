//! Contains manipulations about VRSystem interface.

use crate::{
    call_interface,
    error::{Error, Result, TrackedPropertyError},
    openvr::{Context, Interface},
};

use std::{ffi::CString, sync::Arc};

use log::debug;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use moonlight_openvr_sys::binding::{HmdMatrix34_t, TrackedDevicePose_t, VR_IVRSystem_FnTable};

/// Tracking universe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum Universe {
    Seated = 0,
    Standing = 1,
    Raw = 2,
}

/// Tracking device class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
#[repr(i32)]
pub enum DeviceClass {
    Invalid = 0,
    HeadMountDisplay = 1,
    Controller = 2,
    GenericTracker = 3,
    TrackingReference = 4,
    DisplayRedirect = 5,
}

// Tracked device property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
#[repr(i32)]
pub enum TrackedDeviceProperty {
    Invalid = 0,
    TrackingSystemNameString = 1000,
    ModelNumberString = 1001,
    SerialNumberString = 1002,
    RenderModelNameString = 1003,
    WillDriftInYawBool = 1004,
    ManufacturerNameString = 1005,
    TrackingFirmwareVersionString = 1006,
    HardwareRevisionString = 1007,
    AllWirelessDongleDescriptionsString = 1008,
    ConnectedWirelessDongleString = 1009,
    DeviceIsWirelessBool = 1010,
    DeviceIsChargingBool = 1011,
    DeviceBatteryPercentageFloat = 1012,
    StatusDisplayTransformMatrix34 = 1013,
    FirmwareUpdateAvailableBool = 1014,
    FirmwareManualUpdateBool = 1015,
    FirmwareManualUpdateURLString = 1016,
    HardwareRevisionUint64 = 1017,
    FirmwareVersionUint64 = 1018,
    FPGAVersionUint64 = 1019,
    VRCVersionUint64 = 1020,
    RadioVersionUint64 = 1021,
    DongleVersionUint64 = 1022,
    BlockServerShutdownBool = 1023,
    CanUnifyCoordinateSystemWithHmdBool = 1024,
    ContainsProximitySensorBool = 1025,
    DeviceProvidesBatteryStatusBool = 1026,
    DeviceCanPowerOffBool = 1027,
    FirmwareProgrammingTargetString = 1028,
    DeviceClassInt32 = 1029,
    HasCameraBool = 1030,
    DriverVersionString = 1031,
    FirmwareForceUpdateRequiredBool = 1032,
    ViveSystemButtonFixRequiredBool = 1033,
    ParentDriverUint64 = 1034,
    ResourceRootString = 1035,
    RegisteredDeviceTypeString = 1036,
    InputProfilePathString = 1037,
    NeverTrackedBool = 1038,
    NumCamerasInt32 = 1039,
    CameraFrameLayoutInt32 = 1040,
    CameraStreamFormatInt32 = 1041,
    AdditionalDeviceSettingsPathString = 1042,
    IdentifiableBool = 1043,
    BootloaderVersionUint64 = 1044,
    AdditionalSystemReportDataString = 1045,
    CompositeFirmwareVersionString = 1046,
    FirmwareRemindUpdateBool = 1047,
    PeripheralApplicationVersionUint64 = 1048,
    ManufacturerSerialNumberString = 1049,
    ComputedSerialNumberString = 1050,
    EstimatedDeviceFirstUseTimeInt32 = 1051,
    ReportsTimeSinceVSyncBool = 2000,
    SecondsFromVsyncToPhotonsFloat = 2001,
    DisplayFrequencyFloat = 2002,
    UserIpdMetersFloat = 2003,
    CurrentUniverseIdUint64 = 2004,
    PreviousUniverseIdUint64 = 2005,
    DisplayFirmwareVersionUint64 = 2006,
    IsOnDesktopBool = 2007,
    DisplayMCTypeInt32 = 2008,
    DisplayMCOffsetFloat = 2009,
    DisplayMCScaleFloat = 2010,
    EdidVendorIDInt32 = 2011,
    DisplayMCImageLeftString = 2012,
    DisplayMCImageRightString = 2013,
    DisplayGCBlackClampFloat = 2014,
    EdidProductIDInt32 = 2015,
    CameraToHeadTransformMatrix34 = 2016,
    DisplayGCTypeInt32 = 2017,
    DisplayGCOffsetFloat = 2018,
    DisplayGCScaleFloat = 2019,
    DisplayGCPrescaleFloat = 2020,
    DisplayGCImageString = 2021,
    LensCenterLeftUFloat = 2022,
    LensCenterLeftVFloat = 2023,
    LensCenterRightUFloat = 2024,
    LensCenterRightVFloat = 2025,
    UserHeadToEyeDepthMetersFloat = 2026,
    CameraFirmwareVersionUint64 = 2027,
    CameraFirmwareDescriptionString = 2028,
    DisplayFPGAVersionUint64 = 2029,
    DisplayBootloaderVersionUint64 = 2030,
    DisplayHardwareVersionUint64 = 2031,
    AudioFirmwareVersionUint64 = 2032,
    CameraCompatibilityModeInt32 = 2033,
    ScreenshotHorizontalFieldOfViewDegreesFloat = 2034,
    ScreenshotVerticalFieldOfViewDegreesFloat = 2035,
    DisplaySuppressedBool = 2036,
    DisplayAllowNightModeBool = 2037,
    DisplayMCImageWidthInt32 = 2038,
    DisplayMCImageHeightInt32 = 2039,
    DisplayMCImageNumChannelsInt32 = 2040,
    DisplayMCImageDataBinary = 2041,
    SecondsFromPhotonsToVblankFloat = 2042,
    DriverDirectModeSendsVsyncEventsBool = 2043,
    DisplayDebugModeBool = 2044,
    GraphicsAdapterLuidUint64 = 2045,
    DriverProvidedChaperonePathString = 2048,
    ExpectedTrackingReferenceCountInt32 = 2049,
    ExpectedControllerCountInt32 = 2050,
    NamedIconPathControllerLeftDeviceOffString = 2051,
    NamedIconPathControllerRightDeviceOffString = 2052,
    NamedIconPathTrackingReferenceDeviceOffString = 2053,
    DoNotApplyPredictionBool = 2054,
    CameraToHeadTransformsMatrix34Array = 2055,
    DistortionMeshResolutionInt32 = 2056,
    DriverIsDrawingControllersBool = 2057,
    DriverRequestsApplicationPauseBool = 2058,
    DriverRequestsReducedRenderingBool = 2059,
    MinimumIpdStepMetersFloat = 2060,
    AudioBridgeFirmwareVersionUint64 = 2061,
    ImageBridgeFirmwareVersionUint64 = 2062,
    ImuToHeadTransformMatrix34 = 2063,
    ImuFactoryGyroBiasVector3 = 2064,
    ImuFactoryGyroScaleVector3 = 2065,
    ImuFactoryAccelerometerBiasVector3 = 2066,
    ImuFactoryAccelerometerScaleVector3 = 2067,
    ConfigurationIncludesLighthouse20FeaturesBool = 2069,
    AdditionalRadioFeaturesUint64 = 2070,
    CameraWhiteBalanceVector4Array = 2071,
    CameraDistortionFunctionInt32Array = 2072,
    CameraDistortionCoefficientsFloatArray = 2073,
    ExpectedControllerTypeString = 2074,
    HmdTrackingStyleInt32 = 2075,
    DriverProvidedChaperoneVisibilityBool = 2076,
    HmdColumnCorrectionSettingPrefixString = 2077,
    CameraSupportsCompatibilityModesBool = 2078,
    SupportsRoomViewDepthProjectionBool = 2079,
    DisplayAvailableFrameRatesFloatArray = 2080,
    DisplaySupportsMultipleFrameratesBool = 2081,
    DisplayColorMultLeftVector3 = 2082,
    DisplayColorMultRightVector3 = 2083,
    DisplaySupportsRuntimeFramerateChangeBool = 2084,
    DisplaySupportsAnalogGainBool = 2085,
    DisplayMinAnalogGainFloat = 2086,
    DisplayMaxAnalogGainFloat = 2087,
    CameraExposureTimeFloat = 2088,
    CameraGlobalGainFloat = 2089,
    DashboardScaleFloat = 2091,
    IpdUIRangeMinMetersFloat = 2100,
    IpdUIRangeMaxMetersFloat = 2101,
    HmdSupportsHDCP14LegacyCompatBool = 2102,
    HmdSupportsMicMonitoringBool = 2103,
    DriverRequestedMuraCorrectionModeInt32 = 2200,
    DriverRequestedMuraFeatherInnerLeftInt32 = 2201,
    DriverRequestedMuraFeatherInnerRightInt32 = 2202,
    DriverRequestedMuraFeatherInnerTopInt32 = 2203,
    DriverRequestedMuraFeatherInnerBottomInt32 = 2204,
    DriverRequestedMuraFeatherOuterLeftInt32 = 2205,
    DriverRequestedMuraFeatherOuterRightInt32 = 2206,
    DriverRequestedMuraFeatherOuterTopInt32 = 2207,
    DriverRequestedMuraFeatherOuterBottomInt32 = 2208,
    AudioDefaultPlaybackDeviceIdString = 2300,
    AudioDefaultRecordingDeviceIdString = 2301,
    AudioDefaultPlaybackDeviceVolumeFloat = 2302,
    AudioSupportsDualSpeakerAndJackOutputBool = 2303,
    AttachedDeviceIdString = 3000,
    SupportedButtonsUint64 = 3001,
    Axis0TypeInt32 = 3002,
    Axis1TypeInt32 = 3003,
    Axis2TypeInt32 = 3004,
    Axis3TypeInt32 = 3005,
    Axis4TypeInt32 = 3006,
    ControllerRoleHintInt32 = 3007,
    FieldOfViewLeftDegreesFloat = 4000,
    FieldOfViewRightDegreesFloat = 4001,
    FieldOfViewTopDegreesFloat = 4002,
    FieldOfViewBottomDegreesFloat = 4003,
    TrackingRangeMinimumMetersFloat = 4004,
    TrackingRangeMaximumMetersFloat = 4005,
    ModeLabelString = 4006,
    CanWirelessIdentifyBool = 4007,
    NonceInt32 = 4008,
    IconPathNameString = 5000,
    NamedIconPathDeviceOffString = 5001,
    NamedIconPathDeviceSearchingString = 5002,
    NamedIconPathDeviceSearchingAlertString = 5003,
    NamedIconPathDeviceReadyString = 5004,
    NamedIconPathDeviceReadyAlertString = 5005,
    NamedIconPathDeviceNotReadyString = 5006,
    NamedIconPathDeviceStandbyString = 5007,
    NamedIconPathDeviceAlertLowString = 5008,
    NamedIconPathDeviceStandbyAlertString = 5009,
    DisplayHiddenAreaBinaryStart = 5100,
    DisplayHiddenAreaBinaryEnd = 5150,
    ParentContainer = 5151,
    OverrideContainerUint64 = 5152,
    UserConfigPathString = 6000,
    InstallPathString = 6001,
    HasDisplayComponentBool = 6002,
    HasControllerComponentBool = 6003,
    HasCameraComponentBool = 6004,
    HasDriverDirectModeComponentBool = 6005,
    HasVirtualDisplayComponentBool = 6006,
    HasSpatialAnchorsSupportBool = 6007,
    ControllerTypeString = 7000,
    ControllerHandSelectionPriorityInt32 = 7002,
    VendorSpecificReservedStart = 10000,
    VendorSpecificReservedEnd = 10999,
    TrackedDevicePropertyMax = 1000000,
}

// Tracked controller hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
#[repr(i32)]
pub enum TrackedControllerRole {
    Invalid = 0,
    LeftHand = 1,
    RightHand = 2,
    OptOut = 3,
    Treadmill = 4,
    Stylus = 5,
}

/// Wraps `IVRSystem` interface.
pub struct System {
    _context: Arc<Context>,
    interface: &'static VR_IVRSystem_FnTable,
}

impl Interface for System {
    type FunctionTable = VR_IVRSystem_FnTable;

    unsafe fn with_interface_pointer(
        context: Arc<Context>,
        interface: *const VR_IVRSystem_FnTable,
    ) -> Self {
        let interface = interface.as_ref().expect("Interface should be non-null");
        debug!("IVRSystem: {:?}", interface);
        System {
            _context: context,
            interface,
        }
    }
}

impl System {
    /// Calls `IVRSystem::GetRecommendedRenderTargetSize`.
    pub fn recommended_render_target_size(&self) -> Result<(u32, u32)> {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        call_interface!(
            self.interface.GetRecommendedRenderTargetSize,
            &mut width as *mut u32,
            &mut height as *mut u32
        );
        Ok((width, height))
    }

    /// Calls `IVRSystem::GetDeviceToAbsoluteTrackingPose`.
    pub fn absolute_tracking_pose(
        &self,
        universe: Universe,
        prediction: f32,
        buffer: &mut [TrackedDevicePose_t],
    ) -> Result<()> {
        call_interface!(
            self.interface.GetDeviceToAbsoluteTrackingPose,
            universe as i32,
            prediction,
            buffer.as_mut_ptr(),
            buffer.len() as u32
        );
        Ok(())
    }

    /// Calls `IVRSystem::GetTrackedDeviceClass`.
    pub fn tracked_device_class(&self, index: usize) -> Result<DeviceClass> {
        let raw_class = call_interface!(self.interface.GetTrackedDeviceClass, index as u32);
        match FromPrimitive::from_i32(raw_class) {
            Some(class) => Ok(class),
            None => Err(Error::Internal("Unknown tracked device class".into())),
        }
    }

    /// Calls `IVRSystem::GetTrackedDeviceProperty`.
    pub fn tracked_device_property<T: PropertyType>(
        &self,
        index: usize,
        property: TrackedDeviceProperty,
    ) -> Result<T> {
        T::get_property(self.interface, index, property)
    }
}

/// Tracked device property types should implement this trait.
pub trait PropertyType
where
    Self: Sized,
{
    fn get_property(
        interface: &VR_IVRSystem_FnTable,
        index: usize,
        property: TrackedDeviceProperty,
    ) -> Result<Self>;
}

impl PropertyType for bool {
    fn get_property(
        interface: &VR_IVRSystem_FnTable,
        index: usize,
        property: TrackedDeviceProperty,
    ) -> Result<Self> {
        let mut prop_error = 0;
        let value = call_interface!(
            interface.GetBoolTrackedDeviceProperty,
            index as u32,
            property as i32,
            &mut prop_error as *mut i32
        );
        match prop_error {
            0 => Ok(value),
            _ => Err(Error::TrackedProperty(TrackedPropertyError::from_raw(
                prop_error as u32,
            ))),
        }
    }
}

impl PropertyType for i32 {
    fn get_property(
        interface: &VR_IVRSystem_FnTable,
        index: usize,
        property: TrackedDeviceProperty,
    ) -> Result<Self> {
        let mut prop_error = 0;
        let value = call_interface!(
            interface.GetInt32TrackedDeviceProperty,
            index as u32,
            property as i32,
            &mut prop_error as *mut i32
        );
        match prop_error {
            0 => Ok(value),
            _ => Err(Error::TrackedProperty(TrackedPropertyError::from_raw(
                prop_error as u32,
            ))),
        }
    }
}

impl PropertyType for u64 {
    fn get_property(
        interface: &VR_IVRSystem_FnTable,
        index: usize,
        property: TrackedDeviceProperty,
    ) -> Result<Self> {
        let mut prop_error = 0;
        let value = call_interface!(
            interface.GetUint64TrackedDeviceProperty,
            index as u32,
            property as i32,
            &mut prop_error as *mut i32
        );
        match prop_error {
            0 => Ok(value),
            _ => Err(Error::TrackedProperty(TrackedPropertyError::from_raw(
                prop_error as u32,
            ))),
        }
    }
}

impl PropertyType for f32 {
    fn get_property(
        interface: &VR_IVRSystem_FnTable,
        index: usize,
        property: TrackedDeviceProperty,
    ) -> Result<Self> {
        let mut prop_error = 0;
        let value = call_interface!(
            interface.GetFloatTrackedDeviceProperty,
            index as u32,
            property as i32,
            &mut prop_error as *mut i32
        );
        match prop_error {
            0 => Ok(value),
            _ => Err(Error::TrackedProperty(TrackedPropertyError::from_raw(
                prop_error as u32,
            ))),
        }
    }
}

impl PropertyType for String {
    fn get_property(
        interface: &VR_IVRSystem_FnTable,
        index: usize,
        property: TrackedDeviceProperty,
    ) -> Result<Self> {
        let mut prop_error = 0;
        let buffer_size = call_interface!(
            interface.GetStringTrackedDeviceProperty,
            index as u32,
            property as i32,
            std::ptr::null_mut(),
            0,
            &mut prop_error as *mut i32
        );

        let mut buffer = vec![0u8; buffer_size as usize];
        call_interface!(
            interface.GetStringTrackedDeviceProperty,
            index as u32,
            property as i32,
            buffer.as_mut_ptr() as *mut i8,
            buffer_size,
            &mut prop_error as *mut i32
        ) as usize;

        match prop_error {
            0 => {
                let result =
                    CString::from_vec_with_nul(buffer).map_err(|e| Error::Internal(e.into()))?;
                Ok(result
                    .to_str()
                    .map_err(|e| Error::Internal(e.into()))?
                    .to_string())
            }
            _ => Err(Error::TrackedProperty(TrackedPropertyError::from_raw(
                prop_error as u32,
            ))),
        }
    }
}

impl PropertyType for HmdMatrix34_t {
    fn get_property(
        interface: &VR_IVRSystem_FnTable,
        index: usize,
        property: TrackedDeviceProperty,
    ) -> Result<Self> {
        let mut prop_error = 0;
        let value = call_interface!(
            interface.GetMatrix34TrackedDeviceProperty,
            index as u32,
            property as i32,
            &mut prop_error as *mut i32
        );
        match prop_error {
            0 => Ok(value),
            _ => Err(Error::TrackedProperty(TrackedPropertyError::from_raw(
                prop_error as u32,
            ))),
        }
    }
}
