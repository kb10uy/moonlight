pub mod binding;

use std::os::raw::c_void;

extern "C" {
    pub fn VR_InitInternal(
        err: *mut binding::EVRInitError,
        app_type: binding::EVRApplicationType,
    ) -> u32;

    pub fn VR_ShutdownInternal();

    pub fn VR_GetGenericInterface(
        interface: *const u8,
        err: *mut binding::EVRInitError,
    ) -> *mut c_void;

    pub fn VR_IsInterfaceVersionValid(interface: *const u8) -> bool;
}
