use crate::{
    cf::{cfarray_descriptions, cfstring_from_str, copy_description, OwnedCFType},
    ffi, Result, ServiceManagementError,
};

pub use crate::authorization::{
    Authorization, AuthorizationFlags, AuthorizationRight, SM_RIGHT_BLESS_PRIVILEGED_HELPER,
    SM_RIGHT_MODIFY_SYSTEM_DAEMONS,
};
pub use crate::ffi::AuthorizationRef;
pub use crate::sm_job_bless::{
    bless as bless_plist, copy_all_job_dictionaries as copy_all_job_dictionaries_structured,
    copy_job_dictionary, job_remove as job_remove_plist, job_submit_plist, LaunchdDomain,
    LegacyJobDictionary, SMJobBless,
};
pub use crate::sm_login_item::{set_enabled as login_item_set_enabled, SMLoginItem};

pub fn job_copy_dictionary(domain: LaunchdDomain, job_label: &str) -> Result<Option<String>> {
    let job_label = cfstring_from_str(job_label)?;
    // SAFETY: domain.as_cfstring() returns a valid CFStringRef. job_label.as_ptr() is a
    // valid NonNull CFStringRef pointer. SMJobCopyDictionary returns a CFDictionaryRef or null.
    let dictionary =
        unsafe { ffi::SMJobCopyDictionary(domain.as_cfstring(), job_label.as_ptr().cast()) };
    if dictionary.is_null() {
        return Ok(None);
    }
    // SAFETY: We checked dictionary is not null above. from_create_rule wraps it in a
    // NonNull which manages the lifetime and ensures CFRelease is called on drop.
    let dictionary = unsafe { OwnedCFType::from_create_rule(dictionary) }.ok_or_else(|| {
        ServiceManagementError::new("SMJobCopyDictionary", "received null CFDictionary")
    })?;
    copy_description(dictionary.as_ptr()).map(Some)
}

pub fn copy_all_job_dictionaries(domain: LaunchdDomain) -> Result<Vec<String>> {
    // SAFETY: domain.as_cfstring() returns a valid CFStringRef. SMCopyAllJobDictionaries
    // returns a CFArrayRef (possibly empty but never null). cfarray_descriptions handles
    // the pointer safely.
    let dictionaries = unsafe { ffi::SMCopyAllJobDictionaries(domain.as_cfstring()) };
    cfarray_descriptions(dictionaries)
}

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
    let ok = ffi::SMJobSubmit(domain.as_cfstring(), job, authorization, &mut error);
    if ok == 0 {
        Err(take_cf_error("SMJobSubmit", error))
    } else {
        if !error.is_null() {
            ffi::CFRelease(error.cast());
        }
        Ok(())
    }
}

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
    let job_label = cfstring_from_str(job_label)?;
    let mut error = std::ptr::null_mut();
    // SAFETY: Caller guarantees domain and authorization are valid as documented above.
    // job_label is a valid CFStringRef from cfstring_from_str.
    let ok = ffi::SMJobRemove(
        domain.as_cfstring(),
        job_label.as_ptr().cast(),
        authorization,
        u8::from(wait),
        &mut error,
    );
    if ok == 0 {
        Err(take_cf_error("SMJobRemove", error))
    } else {
        if !error.is_null() {
            ffi::CFRelease(error.cast());
        }
        Ok(())
    }
}

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
    let executable_label = cfstring_from_str(executable_label)?;
    let mut error = std::ptr::null_mut();
    // SAFETY: Caller guarantees domain and authorization are valid as documented above.
    // executable_label is a valid CFStringRef from cfstring_from_str.
    let ok = ffi::SMJobBless(
        domain.as_cfstring(),
        executable_label.as_ptr().cast(),
        authorization,
        &mut error,
    );
    if ok == 0 {
        Err(take_cf_error("SMJobBless", error))
    } else {
        if !error.is_null() {
            ffi::CFRelease(error.cast());
        }
        Ok(())
    }
}

// SAFETY: Only called from error paths within unsafe functions, with error pointers
// guaranteed to be non-null by the caller. CFRelease is safe to call on valid CFError pointers.
unsafe fn take_cf_error(function: &'static str, error: ffi::CFErrorRef) -> ServiceManagementError {
    if error.is_null() {
        return ServiceManagementError::new(function, "operation failed without a CFError");
    }

    let message = crate::cf::copy_description(error.cast())
        .unwrap_or_else(|_| "operation failed without a readable CFError".to_string());
    ffi::CFRelease(error.cast());
    ServiceManagementError::new(function, message)
}
