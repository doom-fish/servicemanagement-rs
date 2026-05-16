use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr::NonNull,
};

use crate::{ffi, Result, ServiceManagementError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppServiceStatus {
    NotRegistered,
    Enabled,
    RequiresApproval,
    NotFound,
    Unknown(i32),
}

impl AppServiceStatus {
    fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::NotRegistered,
            1 => Self::Enabled,
            2 => Self::RequiresApproval,
            3 => Self::NotFound,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug)]
pub struct AppService(NonNull<c_void>);

impl AppService {
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

    pub fn status(&self) -> AppServiceStatus {
        let raw = unsafe { ffi::sm_app_service_status(self.0.as_ptr()) };
        AppServiceStatus::from_raw(raw)
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

    fn from_raw(raw: *mut c_void, error: *mut c_char, function: &'static str) -> Result<Self> {
        NonNull::new(raw)
            .map(Self)
            .ok_or_else(|| bridge_error(function, error))
    }
}

impl Drop for AppService {
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

fn c_string(value: &str, function: &'static str) -> Result<CString> {
    CString::new(value).map_err(|_| {
        ServiceManagementError::new(function, "strings cannot contain interior NUL bytes")
    })
}

fn bridge_error(function: &'static str, error: *mut c_char) -> ServiceManagementError {
    if error.is_null() {
        return ServiceManagementError::new(function, "operation failed without an error message");
    }

    let message = unsafe { CStr::from_ptr(error) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::sm_string_free(error) };
    ServiceManagementError::new(function, message)
}
