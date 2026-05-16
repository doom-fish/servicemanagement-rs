use std::ffi::{c_char, CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use crate::{ffi, Result, ServiceManagementError};

pub(crate) fn c_string(value: &str, function: &'static str) -> Result<CString> {
    CString::new(value).map_err(|_| {
        ServiceManagementError::new(function, "strings cannot contain interior NUL bytes")
    })
}

pub(crate) fn path_c_string(path: &Path, function: &'static str) -> Result<CString> {
    CString::new(path.as_os_str().as_bytes()).map_err(|_| {
        ServiceManagementError::new(function, "paths cannot contain interior NUL bytes")
    })
}

pub(crate) fn bridge_error(function: &'static str, error: *mut c_char) -> ServiceManagementError {
    if error.is_null() {
        return ServiceManagementError::new(function, "operation failed without an error message");
    }

    let message = unsafe { CStr::from_ptr(error) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::sm_string_free(error) };
    ServiceManagementError::new(function, message)
}

pub(crate) fn take_bridge_string(raw: *mut c_char, function: &'static str) -> Result<String> {
    if raw.is_null() {
        return Err(ServiceManagementError::new(
            function,
            "bridge returned a null string",
        ));
    }

    let value = unsafe { CStr::from_ptr(raw) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::sm_string_free(raw) };
    Ok(value)
}
