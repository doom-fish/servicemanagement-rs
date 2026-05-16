use std::{
    ffi::{c_char, c_void},
    ptr::NonNull,
};

use crate::{
    bridge::{bridge_error, c_string, take_bridge_string},
    ffi, Result, SMAppServiceStatus,
};

#[derive(Debug)]
pub struct SMAppService(NonNull<c_void>);

impl SMAppService {
    pub fn main_app() -> Result<Self> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_app_service_main_app(&mut error) };
        Self::from_raw(raw, error, "sm_app_service_main_app")
    }

    pub fn login_item(identifier: &str) -> Result<Self> {
        let identifier = c_string(identifier, "sm_app_service_login_item")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_app_service_login_item(identifier.as_ptr(), &mut error) };
        Self::from_raw(raw, error, "sm_app_service_login_item")
    }

    pub fn agent(plist_name: &str) -> Result<Self> {
        let plist_name = c_string(plist_name, "sm_app_service_agent")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_app_service_agent(plist_name.as_ptr(), &mut error) };
        Self::from_raw(raw, error, "sm_app_service_agent")
    }

    pub fn daemon(plist_name: &str) -> Result<Self> {
        let plist_name = c_string(plist_name, "sm_app_service_daemon")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_app_service_daemon(plist_name.as_ptr(), &mut error) };
        Self::from_raw(raw, error, "sm_app_service_daemon")
    }

    pub fn status(&self) -> SMAppServiceStatus {
        let raw = unsafe { ffi::sm_app_service_status(self.0.as_ptr()) };
        SMAppServiceStatus::from_raw(raw)
    }

    pub fn register(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::sm_app_service_register(self.0.as_ptr(), &mut error) };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_app_service_register", error))
        }
    }

    pub fn unregister(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::sm_app_service_unregister(self.0.as_ptr(), &mut error) };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_app_service_unregister", error))
        }
    }

    pub fn unregister_with_completion_handler(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
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
        unsafe { ffi::sm_app_service_release(self.0.as_ptr()) };
    }
}

pub fn open_system_settings_login_items() -> Result<()> {
    let mut error = std::ptr::null_mut();
    let ok = unsafe { ffi::sm_open_system_settings_login_items(&mut error) };
    if ok {
        Ok(())
    } else {
        Err(bridge_error("sm_open_system_settings_login_items", error))
    }
}

pub fn app_service_error_domain() -> Result<String> {
    let mut error = std::ptr::null_mut();
    let raw = unsafe { ffi::sm_app_service_error_domain(&mut error) };
    if !error.is_null() {
        return Err(bridge_error("sm_app_service_error_domain", error));
    }
    take_bridge_string(raw, "sm_app_service_error_domain")
}
