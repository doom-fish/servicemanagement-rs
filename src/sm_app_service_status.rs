use std::path::Path;

use crate::{
    bridge::{bridge_error, path_c_string},
    ffi, Result, ServiceManagementError,
};

/// Mirrors `ServiceManagement.SMAppService.Status`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SMAppServiceStatus {
    /// Corresponds to `SMAppService.Status.notRegistered`.
    NotRegistered,
    /// Corresponds to `SMAppService.Status.enabled`.
    Enabled,
    /// Corresponds to `SMAppService.Status.requiresApproval`.
    RequiresApproval,
    /// Corresponds to `SMAppService.Status.notFound`.
    NotFound,
    /// Preserves an unknown raw status value from ServiceManagement.
    Unknown(i32),
}

impl SMAppServiceStatus {
    pub(crate) const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::NotRegistered,
            1 => Self::Enabled,
            2 => Self::RequiresApproval,
            3 => Self::NotFound,
            other => Self::Unknown(other),
        }
    }

    /// Returns the raw ServiceManagement status value.
    pub const fn raw_value(self) -> i32 {
        match self {
            Self::NotRegistered => 0,
            Self::Enabled => 1,
            Self::RequiresApproval => 2,
            Self::NotFound => 3,
            Self::Unknown(raw) => raw,
        }
    }

    /// Returns the canonical ServiceManagement status string.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRegistered => "notRegistered",
            Self::Enabled => "enabled",
            Self::RequiresApproval => "requiresApproval",
            Self::NotFound => "notFound",
            Self::Unknown(_) => "unknown",
        }
    }
}

/// Returns the ServiceManagement status for a legacy launchd plist path.
pub fn status_for_legacy_plist(path: impl AsRef<Path>) -> Result<SMAppServiceStatus> {
    let path = path_c_string(path.as_ref(), "sm_app_service_status_for_legacy_plist")?;
    let mut error = std::ptr::null_mut();
    let raw = unsafe { ffi::sm_app_service_status_for_legacy_plist(path.as_ptr(), &mut error) };
    if !error.is_null() {
        return Err(bridge_error(
            "sm_app_service_status_for_legacy_plist",
            error,
        ));
    }
    if raw < 0 {
        return Err(ServiceManagementError::new(
            "sm_app_service_status_for_legacy_plist",
            format!("bridge returned an invalid status value {raw}"),
        ));
    }
    Ok(SMAppServiceStatus::from_raw(raw))
}
