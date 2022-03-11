//! Contains root functionality of OpenVR API.

use crate::{
    error::{Error, InitError, Result},
    system::System,
};

use std::{
    ffi::CStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use log::{error, info};
use moonlight_openvr_sys::{
    binding::IVRSystem_Version, VR_GetGenericInterface, VR_InitInternal, VR_ShutdownInternal,
};

/// Global flag whether OpenVR API is already initialized.
static OPENVR_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Tells OpenVR your application type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ApplicationType {
    Other = 0,
    Scene,
    Overlay,
    Background,
    Utility,
}

/// Assures the validity of OpenVR API.
/// This struct should be unique if exists.
#[derive(Debug, PartialEq, Eq)]
pub struct Context(u32);

impl Context {
    /// Initializes OpenVR API.
    pub fn new(app_type: ApplicationType) -> Result<Arc<Context>> {
        if OPENVR_INITIALIZED.load(Ordering::SeqCst) {
            error!("OpenVR is already initialized");
            return Err(Error::AlreadyInitialized);
        }

        let mut err: i32 = 0;
        let token = unsafe { VR_InitInternal(&mut err as *mut i32, app_type as i32) };
        match err {
            0 => {
                info!("OpenVR initialized (token: {})", token);
                OPENVR_INITIALIZED.store(true, Ordering::SeqCst);
                Ok(Arc::new(Context(token)))
            }
            _ => Err(Error::Init(InitError::from_raw(err as u32))),
        }
    }

    /// Returns token number returned from VR_InitInternal().
    pub fn token(self: Arc<Self>) -> u32 {
        self.0
    }

    /// Get IVRSystem Interface.
    pub fn system(self: Arc<Self>) -> Result<System> {
        let name = CStr::from_bytes_with_nul(IVRSystem_Version)
            .expect("Invalid name")
            .to_str()
            .expect("Invalid name");

        self.get_interface(name)
    }

    /// Fetches Interface table struct pointer by interface name.
    /// `interface` parameter must end with NUL byte.
    pub fn get_interface<I: Interface>(self: Arc<Self>, interface: &str) -> Result<I> {
        // **Undocumented behavior**
        // We have to suffix the interface name by "FnTable:" when C API is needed.
        let mut qualified_interface = String::from("FnTable:");
        qualified_interface.push_str(interface);

        info!("Fetching interface \"{}\"", qualified_interface);
        let mut err: i32 = 0;
        let interface = qualified_interface.as_ptr();
        let interface_pointer = unsafe { VR_GetGenericInterface(interface, &mut err as *mut i32) };
        match err {
            0 => {
                info!("Interface returned: {:?}", interface_pointer);
                let interface = unsafe {
                    I::with_interface_pointer(self, interface_pointer as *const I::FunctionTable)
                };
                Ok(interface)
            }
            _ => Err(Error::Init(InitError::from_raw(err as u32))),
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        OPENVR_INITIALIZED.store(false, Ordering::SeqCst);
        unsafe {
            VR_ShutdownInternal();
        }
    }
}

/// OpenVR interface functions.
pub trait Interface {
    /// OpenVR raw function table type.
    type FunctionTable;

    /// Constructs safe interface wrapper with raw pointer.
    ///
    /// # Safety
    /// Given `interface` pointer will not be null, but function pointers within it are maybe null.
    unsafe fn with_interface_pointer(
        context: Arc<Context>,
        interface: *const Self::FunctionTable,
    ) -> Self;
}
