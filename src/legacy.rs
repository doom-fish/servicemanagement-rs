use apple_cf::cf::{CFError, CFString, CFType};

use crate::{ffi, Result, ServiceManagementError};

/// Re-exports legacy authorization helpers used with ServiceManagement.
pub use crate::authorization::{
    Authorization, AuthorizationFlags, AuthorizationRight, SM_RIGHT_BLESS_PRIVILEGED_HELPER,
    SM_RIGHT_MODIFY_SYSTEM_DAEMONS,
};
/// Re-exports the raw legacy ServiceManagement `AuthorizationRef` type.
pub use crate::ffi::AuthorizationRef;
/// Re-exports legacy `SMJobBless` helpers from ServiceManagement.
#[allow(deprecated)]
pub use crate::sm_job_bless::{
    bless as bless_plist, copy_all_job_dictionaries, copy_job_dictionary,
    job_remove as job_remove_plist, job_submit_plist, LaunchdDomain, LegacyJobDictionary,
    SMJobBless,
};
/// Re-exports the legacy `SMLoginItemSetEnabled` wrapper.
#[allow(deprecated)]
pub use crate::sm_login_item::{set_enabled as login_item_set_enabled, SMLoginItem};

fn cfstring(value: &str, function: &'static str) -> Result<CFString> {
    if value.contains('\0') {
        return Err(ServiceManagementError::new(
            function,
            "strings passed to CoreFoundation cannot contain interior NUL bytes",
        ));
    }
    Ok(CFString::new(value))
}

#[deprecated(
    since = "0.5.0",
    note = "SMJobCopyDictionary is deprecated since macOS 10.10; use SMAppService::status instead"
)]
/// Returns the `SMJobCopyDictionary` description for a matching launchd job.
pub fn job_copy_dictionary(domain: LaunchdDomain, job_label: &str) -> Result<Option<String>> {
    let job_label = cfstring(job_label, "SMJobCopyDictionary")?;
    // SAFETY: domain.as_cfstring() returns a valid CFStringRef and job_label is a live CFString.
    // SMJobCopyDictionary follows the create rule, so the returned dictionary (or null) is
    // adopted by CFType::from_raw and released on drop.
    let dictionary = unsafe {
        let raw = ffi::SMJobCopyDictionary(domain.as_cfstring(), job_label.as_ptr().cast());
        CFType::from_raw(raw.cast_mut().cast())
    };
    Ok(dictionary.map(|dictionary| dictionary.description()))
}

#[deprecated(
    since = "0.5.0",
    note = "SMJobSubmit is deprecated since macOS 10.10; use SMAppService instead"
)]
/// Raw CoreFoundation interface for SMJobSubmit.
///
/// # Safety
///
/// Caller must provide:
/// - `domain`: a valid CFStringRef from domain.as_cfstring()
/// - `job`: a valid CFDictionaryRef (non-null)
/// - `authorization`: a valid AuthorizationRef or null
pub unsafe fn job_submit_raw(
    domain: LaunchdDomain,
    job: ffi::CFDictionaryRef,
    authorization: AuthorizationRef,
) -> Result<()> {
    let mut error = std::ptr::null_mut();
    // SAFETY: Caller guarantees domain, job, and authorization are valid as documented above.
    let ok = unsafe { ffi::SMJobSubmit(domain.as_cfstring(), job, authorization, &raw mut error) };
    // SAFETY: SMJobSubmit hands back a +1 CFErrorRef (or null), adopted here.
    cf_result("SMJobSubmit", ok, unsafe {
        CFError::from_raw(error.cast())
    })
}

#[deprecated(
    since = "0.5.0",
    note = "SMJobRemove is deprecated since macOS 10.10; use SMAppService::unregister instead"
)]
/// Raw CoreFoundation interface for SMJobRemove.
///
/// # Safety
///
/// Caller must provide:
/// - `domain`: a valid CFStringRef from domain.as_cfstring()
/// - `job_label`: a valid Rust str reference
/// - `authorization`: a valid AuthorizationRef or null
pub unsafe fn job_remove(
    domain: LaunchdDomain,
    job_label: &str,
    authorization: AuthorizationRef,
    wait: bool,
) -> Result<()> {
    let job_label = cfstring(job_label, "SMJobRemove")?;
    let mut error = std::ptr::null_mut();
    // SAFETY: Caller guarantees domain and authorization are valid as documented above.
    // job_label is a live CFString.
    let ok = unsafe {
        ffi::SMJobRemove(
            domain.as_cfstring(),
            job_label.as_ptr().cast(),
            authorization,
            u8::from(wait),
            &raw mut error,
        )
    };
    // SAFETY: SMJobRemove hands back a +1 CFErrorRef (or null), adopted here.
    cf_result("SMJobRemove", ok, unsafe {
        CFError::from_raw(error.cast())
    })
}

#[deprecated(
    since = "0.5.0",
    note = "SMJobBless is deprecated since macOS 13; register the helper with SMAppService::daemon instead. A blessed helper runs as root and must validate every XPC client, for example by looking the client up with security-rs Code::guest_with_audit_token and checking it with Code::check_validity against a code requirement"
)]
/// Raw CoreFoundation interface for SMJobBless.
///
/// # Safety
///
/// Caller must provide:
/// - `domain`: a valid CFStringRef from domain.as_cfstring()
/// - `executable_label`: a valid Rust str reference
/// - `authorization`: a valid AuthorizationRef or null
pub unsafe fn job_bless(
    domain: LaunchdDomain,
    executable_label: &str,
    authorization: AuthorizationRef,
) -> Result<()> {
    let executable_label = cfstring(executable_label, "SMJobBless")?;
    let mut error = std::ptr::null_mut();
    // SAFETY: Caller guarantees domain and authorization are valid as documented above.
    // executable_label is a live CFString.
    let ok = unsafe {
        ffi::SMJobBless(
            domain.as_cfstring(),
            executable_label.as_ptr().cast(),
            authorization,
            &raw mut error,
        )
    };
    // SAFETY: SMJobBless hands back a +1 CFErrorRef (or null), adopted here.
    cf_result("SMJobBless", ok, unsafe { CFError::from_raw(error.cast()) })
}

fn cf_result(function: &'static str, ok: ffi::Boolean, error: Option<CFError>) -> Result<()> {
    if ok != 0 {
        return Ok(());
    }
    let Some(error) = error else {
        return Err(ServiceManagementError::new(
            function,
            "operation failed without a CFError",
        ));
    };
    let message = error.description_string().map_or_else(
        || "operation failed without a readable CFError".to_string(),
        |description| description.to_string_lossy(),
    );
    Err(ServiceManagementError::with_domain_and_code(
        function,
        message,
        error.domain().to_string_lossy(),
        error.code(),
    ))
}
