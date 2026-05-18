use std::ops::Deref;

use crate::{bridge::c_string, ffi, Result, SMAppService};

#[derive(Debug)]
/// Wraps `ServiceManagement.SMAppService.loginItem(identifier:)`.
pub struct LoginItem(SMAppService);

impl LoginItem {
    /// Creates a login item service wrapper for a bundle identifier.
    pub fn new(identifier: &str) -> Result<Self> {
        let identifier = c_string(identifier, "sm_login_item_service")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_login_item_service(identifier.as_ptr(), &mut error) };
        SMAppService::from_raw(raw, error, "sm_login_item_service").map(Self)
    }

    /// Returns the underlying `ServiceManagement.SMAppService` reference.
    pub fn as_app_service(&self) -> &SMAppService {
        &self.0
    }

    /// Consumes this wrapper and returns the underlying `ServiceManagement.SMAppService`.
    pub fn into_app_service(self) -> SMAppService {
        self.0
    }
}

impl Deref for LoginItem {
    type Target = SMAppService;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<LoginItem> for SMAppService {
    fn from(value: LoginItem) -> Self {
        value.0
    }
}
