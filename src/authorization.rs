use std::{
    ffi::{c_char, c_void, CString},
    ptr::NonNull,
};

use bitflags::bitflags;

use crate::{
    bridge::{bridge_error, c_string, take_bridge_string},
    ffi, Result, ServiceManagementError,
};

pub const SM_RIGHT_BLESS_PRIVILEGED_HELPER: &str = "com.apple.ServiceManagement.blesshelper";
pub const SM_RIGHT_MODIFY_SYSTEM_DAEMONS: &str = "com.apple.ServiceManagement.daemons.modify";

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
    pub struct AuthorizationFlags: u32 {
        const DEFAULTS = 0;
        const INTERACTION_ALLOWED = 1 << 0;
        const EXTEND_RIGHTS = 1 << 1;
        const PARTIAL_RIGHTS = 1 << 2;
        const DESTROY_RIGHTS = 1 << 3;
        const PREAUTHORIZE = 1 << 4;
        const SKIP_INTERNAL_AUTH = 1 << 9;
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum AuthorizationRight {
    BlessPrivilegedHelper,
    ModifySystemDaemons,
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

#[derive(Debug)]
pub struct Authorization(NonNull<c_void>);

impl Authorization {
    pub fn new(flags: AuthorizationFlags) -> Result<Self> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_authorization_create(flags.bits(), &mut error) };
        Self::from_raw(raw, error, "sm_authorization_create")
    }

    pub fn with_rights(rights: &[AuthorizationRight], flags: AuthorizationFlags) -> Result<Self> {
        if rights.is_empty() {
            return Self::new(flags);
        }

        let rights = rights_payload(rights, "sm_authorization_create_with_rights")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::sm_authorization_create_with_rights(rights.as_ptr(), flags.bits(), &mut error)
        };
        Self::from_raw(raw, error, "sm_authorization_create_with_rights")
    }

    pub fn copy_rights(
        &self,
        rights: &[AuthorizationRight],
        flags: AuthorizationFlags,
    ) -> Result<()> {
        let rights = rights_payload(rights, "sm_authorization_copy_rights")?;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::sm_authorization_copy_rights(
                self.0.as_ptr(),
                rights.as_ptr(),
                flags.bits(),
                &mut error,
            )
        };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_authorization_copy_rights", error))
        }
    }

    pub fn external_form(&self) -> Result<String> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_authorization_external_form(self.0.as_ptr(), &mut error) };
        if !error.is_null() {
            return Err(bridge_error("sm_authorization_external_form", error));
        }
        take_bridge_string(raw, "sm_authorization_external_form")
    }

    pub fn from_external_form(form: &str) -> Result<Self> {
        let form = c_string(form, "sm_authorization_from_external_form")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_authorization_from_external_form(form.as_ptr(), &mut error) };
        Self::from_raw(raw, error, "sm_authorization_from_external_form")
    }

    pub fn destroy_rights(self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::sm_authorization_destroy_rights(self.0.as_ptr(), &mut error) };
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
        unsafe { ffi::sm_authorization_release(self.0.as_ptr()) };
    }
}

fn rights_payload(rights: &[AuthorizationRight], function: &'static str) -> Result<CString> {
    let rights: Vec<String> = rights.iter().map(AuthorizationRight::as_string).collect();
    let payload = serde_json::to_string(&rights)
        .map_err(|error| ServiceManagementError::new(function, error.to_string()))?;
    c_string(&payload, function)
}
