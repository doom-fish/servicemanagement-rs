use std::ops::Deref;

use crate::{bridge::c_string, ffi, Result, SMAppService};

#[derive(Debug)]
/// Wraps `ServiceManagement.SMAppService.daemon(plistName:)`.
pub struct DaemonService(SMAppService);

impl DaemonService {
    /// Creates a daemon service wrapper for a launch daemon plist.
    pub fn new(plist_name: &str) -> Result<Self> {
        let plist_name = c_string(plist_name, "sm_daemon_service")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_daemon_service(plist_name.as_ptr(), &mut error) };
        SMAppService::from_raw(raw, error, "sm_daemon_service").map(Self)
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

impl Deref for DaemonService {
    type Target = SMAppService;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DaemonService> for SMAppService {
    fn from(value: DaemonService) -> Self {
        value.0
    }
}
