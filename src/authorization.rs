use std::{
    ffi::{c_char, c_void, CString},
    ptr::NonNull,
};

use bitflags::bitflags;

use crate::{
    bridge::{bridge_error, c_string},
    ffi, Result, ServiceManagementError,
};

pub const EXTERNAL_FORM_LEN: usize = 32;
const EXTERNAL_FORM_ISIZE: isize = 32;

/// Authorization right string for ServiceManagement privileged helper installation.
pub const SM_RIGHT_BLESS_PRIVILEGED_HELPER: &str = "com.apple.ServiceManagement.blesshelper";
/// Authorization right string for ServiceManagement system daemon changes.
pub const SM_RIGHT_MODIFY_SYSTEM_DAEMONS: &str = "com.apple.ServiceManagement.daemons.modify";

bitflags! {
    /// Security authorization flags used by ServiceManagement helper operations.
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
    pub struct AuthorizationFlags: u32 {
        /// Mirrors the default Security authorization flags.
        const DEFAULTS = 0;
        /// Allows Security authorization UI during ServiceManagement requests.
        const INTERACTION_ALLOWED = 1 << 0;
        /// Requests Security to extend rights for ServiceManagement operations.
        const EXTEND_RIGHTS = 1 << 1;
        /// Accepts partial Security authorization rights when available.
        const PARTIAL_RIGHTS = 1 << 2;
        /// Destroys granted Security authorization rights on release.
        const DESTROY_RIGHTS = 1 << 3;
        /// Preauthorizes Security authorization rights when possible.
        const PREAUTHORIZE = 1 << 4;
        /// Skips internal authorization checks for legacy ServiceManagement flows.
        const SKIP_INTERNAL_AUTH = 1 << 9;
    }
}

/// Authorization rights accepted by ServiceManagement helper APIs.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum AuthorizationRight {
    /// Uses the ServiceManagement privileged helper right string.
    BlessPrivilegedHelper,
    /// Uses the ServiceManagement system daemon modification right string.
    ModifySystemDaemons,
    /// Uses a custom authorization right string.
    Custom(String),
}

impl AuthorizationRight {
    fn as_string(&self) -> String {
        match self {
            Self::BlessPrivilegedHelper => SM_RIGHT_BLESS_PRIVILEGED_HELPER.to_string(),
            Self::ModifySystemDaemons => SM_RIGHT_MODIFY_SYSTEM_DAEMONS.to_string(),
            Self::Custom(value) => value.clone(),
        }
    }
}

/// Owns a Security `AuthorizationRef` for ServiceManagement calls.
#[derive(Debug)]
pub struct Authorization(NonNull<c_void>);

impl Authorization {
    /// Creates a Security authorization reference for ServiceManagement use.
    pub fn new(flags: AuthorizationFlags) -> Result<Self> {
        let mut error = std::ptr::null_mut();
        // SAFETY: The FFI function returns a valid opaque pointer or null on error. The pointer
        // is consumed immediately by from_raw() which validates it or returns an error.
        let raw = unsafe { ffi::sm_authorization_create(flags.bits(), &raw mut error) };
        Self::from_raw(raw, error, "sm_authorization_create")
    }

    /// Creates a Security authorization reference with explicit ServiceManagement rights.
    pub fn with_rights(rights: &[AuthorizationRight], flags: AuthorizationFlags) -> Result<Self> {
        if rights.is_empty() {
            return Self::new(flags);
        }

        let rights = rights_payload(rights, "sm_authorization_create_with_rights")?;
        let mut error = std::ptr::null_mut();
        // SAFETY: rights.as_ptr() points to a valid nul-terminated C string from CString.
        // The FFI function returns a valid pointer or null on error, consumed by from_raw().
        let raw = unsafe {
            ffi::sm_authorization_create_with_rights(rights.as_ptr(), flags.bits(), &raw mut error)
        };
        Self::from_raw(raw, error, "sm_authorization_create_with_rights")
    }

    /// Copies additional Security authorization rights onto this reference.
    pub fn copy_rights(
        &self,
        rights: &[AuthorizationRight],
        flags: AuthorizationFlags,
    ) -> Result<()> {
        let rights = rights_payload(rights, "sm_authorization_copy_rights")?;
        let mut error = std::ptr::null_mut();
        // SAFETY: self.0 is a valid NonNull opaque pointer from a previous successful FFI call.
        // rights.as_ptr() points to valid nul-terminated C string. The FFI function validates
        // its arguments and returns a bool status code.
        let ok = unsafe {
            ffi::sm_authorization_copy_rights(
                self.0.as_ptr(),
                rights.as_ptr(),
                flags.bits(),
                &raw mut error,
            )
        };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_authorization_copy_rights", error))
        }
    }

    /// Returns the `AuthorizationExternalForm` bytes for this Security authorization reference.
    pub fn external_form(&self) -> Result<Vec<u8>> {
        let mut bytes = vec![0_u8; EXTERNAL_FORM_LEN];
        let mut error = std::ptr::null_mut();
        // SAFETY: self.0 is a valid NonNull opaque pointer from a previous successful FFI call,
        // and bytes has room for the EXTERNAL_FORM_LEN bytes the bridge writes.
        let ok = unsafe {
            ffi::sm_authorization_external_form(
                self.0.as_ptr(),
                bytes.as_mut_ptr().cast(),
                EXTERNAL_FORM_ISIZE,
                &raw mut error,
            )
        };
        if ok {
            Ok(bytes)
        } else {
            Err(bridge_error("sm_authorization_external_form", error))
        }
    }

    /// Restores a Security authorization reference from `AuthorizationExternalForm` bytes.
    pub fn from_external_form(form: &[u8]) -> Result<Self> {
        if form.len() != EXTERNAL_FORM_LEN {
            return Err(ServiceManagementError::new(
                "sm_authorization_from_external_form",
                format!("an Authorization external form is {EXTERNAL_FORM_LEN} bytes"),
            ));
        }
        let mut error = std::ptr::null_mut();
        // SAFETY: form points to EXTERNAL_FORM_LEN readable bytes. The FFI function returns a
        // valid pointer or null on error, consumed by from_raw().
        let raw = unsafe {
            ffi::sm_authorization_from_external_form(
                form.as_ptr().cast(),
                EXTERNAL_FORM_ISIZE,
                &raw mut error,
            )
        };
        Self::from_raw(raw, error, "sm_authorization_from_external_form")
    }

    /// Destroys the Security authorization rights held by this reference.
    pub fn destroy_rights(self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        // SAFETY: self.0 is a valid NonNull opaque pointer from a previous successful FFI call.
        // The FFI function validates its argument and returns a bool status code. self is
        // consumed, preventing further use after destroy_rights() is called.
        let ok = unsafe { ffi::sm_authorization_destroy_rights(self.0.as_ptr(), &raw mut error) };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_authorization_destroy_rights", error))
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    fn from_raw(raw: *mut c_void, error: *mut c_char, function: &'static str) -> Result<Self> {
        NonNull::new(raw)
            .map(Self)
            .ok_or_else(|| bridge_error(function, error))
    }
}

impl Drop for Authorization {
    fn drop(&mut self) {
        // SAFETY: self.0 is a valid NonNull opaque pointer from a previous successful FFI call.
        // Calling sm_authorization_release on a valid pointer is safe; it handles null internally.
        unsafe { ffi::sm_authorization_release(self.0.as_ptr()) };
    }
}

fn rights_payload(rights: &[AuthorizationRight], function: &'static str) -> Result<CString> {
    let rights: Vec<String> = rights.iter().map(AuthorizationRight::as_string).collect();
    let payload = serde_json::to_string(&rights)
        .map_err(|error| ServiceManagementError::new(function, error.to_string()))?;
    c_string(&payload, function)
}
