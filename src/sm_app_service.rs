use std::{
    ffi::{c_char, c_void},
    ptr::NonNull,
};

use crate::{
    bridge::{bridge_error, c_string, take_bridge_string},
    ffi, Result, SMAppServiceStatus,
};

/// Wraps a `ServiceManagement.SMAppService` instance.
#[derive(Debug)]
pub struct SMAppService(NonNull<c_void>);

impl SMAppService {
    /// Returns the ServiceManagement main app service.
    pub fn main_app() -> Result<Self> {
        let mut error = std::ptr::null_mut();
        // SAFETY: The FFI function returns a valid opaque pointer or null on error. The pointer
        // is consumed immediately by from_raw() which validates it or returns an error.
        let raw = unsafe { ffi::sm_app_service_main_app(&mut error) };
        Self::from_raw(raw, error, "sm_app_service_main_app")
    }

    /// Returns the ServiceManagement login item service for a bundle identifier.
    pub fn login_item(identifier: &str) -> Result<Self> {
        let identifier = c_string(identifier, "sm_app_service_login_item")?;
        let mut error = std::ptr::null_mut();
        // SAFETY: identifier.as_ptr() points to a valid nul-terminated C string from CString.
        // The FFI function returns a valid pointer or null on error, consumed by from_raw().
        let raw = unsafe { ffi::sm_app_service_login_item(identifier.as_ptr(), &mut error) };
        Self::from_raw(raw, error, "sm_app_service_login_item")
    }

    /// Returns the ServiceManagement agent service for a launch agent plist.
    pub fn agent(plist_name: &str) -> Result<Self> {
        let plist_name = c_string(plist_name, "sm_app_service_agent")?;
        let mut error = std::ptr::null_mut();
        // SAFETY: plist_name.as_ptr() points to a valid nul-terminated C string from CString.
        // The FFI function returns a valid pointer or null on error, consumed by from_raw().
        let raw = unsafe { ffi::sm_app_service_agent(plist_name.as_ptr(), &mut error) };
        Self::from_raw(raw, error, "sm_app_service_agent")
    }

    /// Returns the ServiceManagement daemon service for a launch daemon plist.
    pub fn daemon(plist_name: &str) -> Result<Self> {
        let plist_name = c_string(plist_name, "sm_app_service_daemon")?;
        let mut error = std::ptr::null_mut();
        // SAFETY: plist_name.as_ptr() points to a valid nul-terminated C string from CString.
        // The FFI function returns a valid pointer or null on error, consumed by from_raw().
        let raw = unsafe { ffi::sm_app_service_daemon(plist_name.as_ptr(), &mut error) };
        Self::from_raw(raw, error, "sm_app_service_daemon")
    }

    /// Returns the current `ServiceManagement.SMAppService.Status` for this service.
    pub fn status(&self) -> SMAppServiceStatus {
        // SAFETY: self.0 is a valid NonNull opaque pointer from a previous successful FFI call.
        // The FFI function returns an enum value (stateless, no lifetime issues).
        let raw = unsafe { ffi::sm_app_service_status(self.0.as_ptr()) };
        SMAppServiceStatus::from_raw(raw)
    }

    /// Registers this ServiceManagement service with the system.
    pub fn register(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        // SAFETY: self.0 is a valid NonNull opaque pointer from a previous successful FFI call.
        // The FFI function validates its argument and returns a bool status code.
        let ok = unsafe { ffi::sm_app_service_register(self.0.as_ptr(), &mut error) };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_app_service_register", error))
        }
    }

    /// Unregisters this ServiceManagement service from the system.
    pub fn unregister(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        // SAFETY: self.0 is a valid NonNull opaque pointer from a previous successful FFI call.
        // The FFI function validates its argument and returns a bool status code.
        let ok = unsafe { ffi::sm_app_service_unregister(self.0.as_ptr(), &mut error) };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_app_service_unregister", error))
        }
    }

    /// Unregisters this ServiceManagement service using the completion-handler variant.
    pub fn unregister_with_completion_handler(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        // SAFETY: self.0 is a valid NonNull opaque pointer from a previous successful FFI call.
        // The FFI function validates its argument and returns a bool status code. The
        // completion handler is managed internally by the bridged Swift code.
        let ok =
            unsafe { ffi::sm_app_service_unregister_with_completion(self.0.as_ptr(), &mut error) };
        if ok {
            Ok(())
        } else {
            Err(bridge_error(
                "sm_app_service_unregister_with_completion",
                error,
            ))
        }
    }

    pub(crate) fn from_raw(
        raw: *mut c_void,
        error: *mut c_char,
        function: &'static str,
    ) -> Result<Self> {
        NonNull::new(raw)
            .map(Self)
            .ok_or_else(|| bridge_error(function, error))
    }
}

impl Drop for SMAppService {
    fn drop(&mut self) {
        // SAFETY: self.0 is a valid NonNull opaque pointer from a previous successful FFI call.
        // sm_app_service_release is the inverse of service creation and is safe to call;
        // the bridged Swift code handles null internally.
        unsafe { ffi::sm_app_service_release(self.0.as_ptr()) };
    }
}

/// Opens the Login Items settings pane used by ServiceManagement services.
pub fn open_system_settings_login_items() -> Result<()> {
    let mut error = std::ptr::null_mut();
    // SAFETY: The FFI function takes no pointer arguments other than an error pointer.
    // It returns a bool status code.
    let ok = unsafe { ffi::sm_open_system_settings_login_items(&mut error) };
    if ok {
        Ok(())
    } else {
        Err(bridge_error("sm_open_system_settings_login_items", error))
    }
}

/// Returns the `SMAppServiceErrorDomain` string from ServiceManagement.
pub fn app_service_error_domain() -> Result<String> {
    let mut error = std::ptr::null_mut();
    // SAFETY: The FFI function returns a pointer to a C string that must be freed via
    // take_bridge_string, which handles the cleanup.
    let raw = unsafe { ffi::sm_app_service_error_domain(&mut error) };
    if !error.is_null() {
        return Err(bridge_error("sm_app_service_error_domain", error));
    }
    take_bridge_string(raw, "sm_app_service_error_domain")
}
