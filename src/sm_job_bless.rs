use std::ffi::c_char;

use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    bridge::{bridge_error, c_string, take_bridge_string},
    ffi, Authorization, Result, ServiceManagementError,
};

/// Selects a launchd domain for legacy ServiceManagement operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum LaunchdDomain {
    /// Corresponds to `kSMDomainSystemLaunchd`.
    System,
    /// Corresponds to `kSMDomainUserLaunchd`.
    User,
}

impl LaunchdDomain {
    pub(crate) const fn raw_value(self) -> i32 {
        match self {
            Self::System => 0,
            Self::User => 1,
        }
    }

    pub(crate) fn as_cfstring(self) -> ffi::CFStringRef {
        // SAFETY: kSMDomainSystemLaunchd and kSMDomainUserLaunchd are static CFStringRef
        // constants defined in the FFI module. They are always valid and never null.
        unsafe {
            match self {
                Self::System => ffi::kSMDomainSystemLaunchd,
                Self::User => ffi::kSMDomainUserLaunchd,
            }
        }
    }
}

/// Structured result returned by legacy `SMJobCopyDictionary` helpers.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct LegacyJobDictionary {
    /// Launchd job label from the ServiceManagement dictionary, if present.
    pub label: Option<String>,
    /// XML plist payload returned by ServiceManagement.
    pub plist_xml: String,
    /// Human-readable Core Foundation description of the dictionary.
    pub description: String,
}

/// Namespace for legacy `SMJobBless`-style ServiceManagement helpers.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMJobBless;

impl SMJobBless {
    /// Returns the legacy `SMJobCopyDictionary` result for a label.
    pub fn copy_job_dictionary(
        domain: LaunchdDomain,
        job_label: &str,
    ) -> Result<Option<LegacyJobDictionary>> {
        let job_label = c_string(job_label, "sm_legacy_copy_job_dictionary")?;
        let mut error = std::ptr::null_mut();
        // SAFETY: domain.raw_value() returns a valid i32. job_label.as_ptr() points to a valid
        // nul-terminated C string. The FFI function returns a C string or null (checked below).
        let raw = unsafe {
            ffi::sm_legacy_copy_job_dictionary(domain.raw_value(), job_label.as_ptr(), &mut error)
        };
        if !error.is_null() {
            return Err(bridge_error("sm_legacy_copy_job_dictionary", error));
        }
        if raw.is_null() {
            return Ok(None);
        }
        parse_json(raw, "sm_legacy_copy_job_dictionary").map(Some)
    }

    /// Returns all legacy `SMCopyAllJobDictionaries` results for a domain.
    pub fn copy_all_job_dictionaries(domain: LaunchdDomain) -> Result<Vec<LegacyJobDictionary>> {
        let mut error = std::ptr::null_mut();
        // SAFETY: domain.raw_value() returns a valid i32. The FFI function returns a C string
        // or null (checked below), or an error pointer.
        let raw =
            unsafe { ffi::sm_legacy_copy_all_job_dictionaries(domain.raw_value(), &mut error) };
        if !error.is_null() {
            return Err(bridge_error("sm_legacy_copy_all_job_dictionaries", error));
        }
        parse_json(raw, "sm_legacy_copy_all_job_dictionaries")
    }

    /// Submits a launchd plist with legacy `SMJobSubmit`.
    pub fn job_submit_plist(
        domain: LaunchdDomain,
        plist_xml: &str,
        authorization: Option<&Authorization>,
    ) -> Result<()> {
        let plist_xml = c_string(plist_xml, "sm_legacy_job_submit_plist")?;
        let mut error = std::ptr::null_mut();
        // SAFETY: domain.raw_value() returns valid i32. plist_xml.as_ptr() points to a valid
        // nul-terminated C string. authorization.map_or() returns either a valid ptr from
        // Authorization::as_ptr or null. The FFI function returns a bool status code.
        let ok = unsafe {
            ffi::sm_legacy_job_submit_plist(
                domain.raw_value(),
                plist_xml.as_ptr(),
                authorization.map_or(std::ptr::null_mut(), Authorization::as_ptr),
                &mut error,
            )
        };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_legacy_job_submit_plist", error))
        }
    }

    /// Removes a launchd job with legacy `SMJobRemove`.
    pub fn job_remove(
        domain: LaunchdDomain,
        job_label: &str,
        authorization: Option<&Authorization>,
        wait: bool,
    ) -> Result<()> {
        let job_label = c_string(job_label, "sm_legacy_job_remove")?;
        let mut error = std::ptr::null_mut();
        // SAFETY: domain.raw_value() returns valid i32. job_label.as_ptr() points to a valid
        // nul-terminated C string. authorization.map_or() returns either a valid ptr or null.
        // wait is a bool. The FFI function returns a bool status code.
        let ok = unsafe {
            ffi::sm_legacy_job_remove(
                domain.raw_value(),
                job_label.as_ptr(),
                authorization.map_or(std::ptr::null_mut(), Authorization::as_ptr),
                wait,
                &mut error,
            )
        };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_legacy_job_remove", error))
        }
    }

    /// Installs a privileged helper with legacy `SMJobBless`.
    pub fn bless(
        domain: LaunchdDomain,
        executable_label: &str,
        authorization: Option<&Authorization>,
    ) -> Result<()> {
        let executable_label = c_string(executable_label, "sm_legacy_job_bless")?;
        let mut error = std::ptr::null_mut();
        // SAFETY: domain.raw_value() returns valid i32. executable_label.as_ptr() points to a
        // valid nul-terminated C string. authorization.map_or() returns either a valid ptr or
        // null. The FFI function returns a bool status code.
        let ok = unsafe {
            ffi::sm_legacy_job_bless(
                domain.raw_value(),
                executable_label.as_ptr(),
                authorization.map_or(std::ptr::null_mut(), Authorization::as_ptr),
                &mut error,
            )
        };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_legacy_job_bless", error))
        }
    }
}

/// Convenience wrapper around `SMJobBless::copy_job_dictionary`.
pub fn copy_job_dictionary(
    domain: LaunchdDomain,
    job_label: &str,
) -> Result<Option<LegacyJobDictionary>> {
    SMJobBless::copy_job_dictionary(domain, job_label)
}

/// Convenience wrapper around `SMJobBless::copy_all_job_dictionaries`.
pub fn copy_all_job_dictionaries(domain: LaunchdDomain) -> Result<Vec<LegacyJobDictionary>> {
    SMJobBless::copy_all_job_dictionaries(domain)
}

/// Convenience wrapper around `SMJobBless::job_submit_plist`.
pub fn job_submit_plist(
    domain: LaunchdDomain,
    plist_xml: &str,
    authorization: Option<&Authorization>,
) -> Result<()> {
    SMJobBless::job_submit_plist(domain, plist_xml, authorization)
}

/// Convenience wrapper around `SMJobBless::job_remove`.
pub fn job_remove(
    domain: LaunchdDomain,
    job_label: &str,
    authorization: Option<&Authorization>,
    wait: bool,
) -> Result<()> {
    SMJobBless::job_remove(domain, job_label, authorization, wait)
}

/// Convenience wrapper around `SMJobBless::bless`.
pub fn bless(
    domain: LaunchdDomain,
    executable_label: &str,
    authorization: Option<&Authorization>,
) -> Result<()> {
    SMJobBless::bless(domain, executable_label, authorization)
}

fn parse_json<T: DeserializeOwned>(raw: *mut c_char, function: &'static str) -> Result<T> {
    let payload = take_bridge_string(raw, function)?;
    serde_json::from_str(&payload)
        .map_err(|error| ServiceManagementError::new(function, error.to_string()))
}
