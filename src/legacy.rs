use std::ffi::c_void;

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
    let dictionary = unsafe { ffi::SMJobCopyDictionary(domain.as_cfstring(), job_label.as_ptr()) };
    if dictionary.is_null() {
        return Ok(None);
    }
    let dictionary = unsafe { OwnedCFType::from_create_rule(dictionary) }.ok_or_else(|| {
        ServiceManagementError::new("SMJobCopyDictionary", "received null CFDictionary")
    })?;
    copy_description(dictionary.as_ptr()).map(Some)
}

pub fn copy_all_job_dictionaries(domain: LaunchdDomain) -> Result<Vec<String>> {
    let dictionaries = unsafe { ffi::SMCopyAllJobDictionaries(domain.as_cfstring()) };
    cfarray_descriptions(dictionaries)
}

pub unsafe fn job_submit_raw(
    domain: LaunchdDomain,
    job: ffi::CFDictionaryRef,
    authorization: AuthorizationRef,
) -> Result<()> {
    let mut error = std::ptr::null();
    let ok = ffi::SMJobSubmit(domain.as_cfstring(), job, authorization, &mut error);
    if ok == 0 {
        Err(take_cf_error("SMJobSubmit", error))
    } else {
        if !error.is_null() {
            ffi::CFRelease(error);
        }
        Ok(())
    }
}

pub unsafe fn job_remove(
    domain: LaunchdDomain,
    job_label: &str,
    authorization: AuthorizationRef,
    wait: bool,
) -> Result<()> {
    let job_label = cfstring_from_str(job_label)?;
    let mut error = std::ptr::null();
    let ok = ffi::SMJobRemove(
        domain.as_cfstring(),
        job_label.as_ptr(),
        authorization,
        u8::from(wait),
        &mut error,
    );
    if ok == 0 {
        Err(take_cf_error("SMJobRemove", error))
    } else {
        if !error.is_null() {
            ffi::CFRelease(error);
        }
        Ok(())
    }
}

pub unsafe fn job_bless(
    domain: LaunchdDomain,
    executable_label: &str,
    authorization: AuthorizationRef,
) -> Result<()> {
    let executable_label = cfstring_from_str(executable_label)?;
    let mut error = std::ptr::null();
    let ok = ffi::SMJobBless(
        domain.as_cfstring(),
        executable_label.as_ptr(),
        authorization,
        &mut error,
    );
    if ok == 0 {
        Err(take_cf_error("SMJobBless", error))
    } else {
        if !error.is_null() {
            ffi::CFRelease(error);
        }
        Ok(())
    }
}

unsafe fn take_cf_error(function: &'static str, error: *const c_void) -> ServiceManagementError {
    if error.is_null() {
        return ServiceManagementError::new(function, "operation failed without a CFError");
    }

    let message = crate::cf::copy_description(error)
        .unwrap_or_else(|_| "operation failed without a readable CFError".to_string());
    ffi::CFRelease(error);
    ServiceManagementError::new(function, message)
}
