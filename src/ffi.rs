#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_void};

pub type AuthorizationRef = *mut c_void;
#[cfg(feature = "async")]
pub type SMAppServiceUnitCallback = unsafe extern "C" fn(context: *mut c_void, error: *mut c_char);
#[cfg(feature = "async")]
pub type SMAppServiceStatusCallback =
    unsafe extern "C" fn(context: *mut c_void, status: i32, error: *mut c_char);
pub use apple_cf::raw::{Boolean, CFArrayRef, CFDictionaryRef, CFErrorRef, CFStringRef};

unsafe extern "C" {
    pub static kSMDomainSystemLaunchd: CFStringRef;
    pub static kSMDomainUserLaunchd: CFStringRef;
    pub static kSMErrorDomainIPC: CFStringRef;
    pub static kSMErrorDomainFramework: CFStringRef;
    pub static kSMErrorDomainLaunchd: CFStringRef;

    pub fn CFArrayGetCount(the_array: CFArrayRef) -> isize;
    pub fn CFArrayGetValueAtIndex(the_array: CFArrayRef, idx: isize) -> *const c_void;
    pub fn CFCopyDescription(cf: *const c_void) -> CFStringRef;
    pub fn CFRetain(cf: *const c_void) -> *const c_void;
    pub fn CFRelease(cf: *const c_void);
    pub fn CFStringCreateWithCString(
        allocator: *const c_void,
        c_str: *const c_char,
        encoding: u32,
    ) -> CFStringRef;
    pub fn CFStringGetLength(the_string: CFStringRef) -> isize;
    pub fn CFStringGetMaximumSizeForEncoding(length: isize, encoding: u32) -> isize;
    pub fn CFStringGetCString(
        the_string: CFStringRef,
        buffer: *mut c_char,
        buffer_size: isize,
        encoding: u32,
    ) -> Boolean;

    pub fn SMJobCopyDictionary(domain: CFStringRef, job_label: CFStringRef) -> CFDictionaryRef;
    pub fn SMCopyAllJobDictionaries(domain: CFStringRef) -> CFArrayRef;
    pub fn SMJobBless(
        domain: CFStringRef,
        executable_label: CFStringRef,
        auth: AuthorizationRef,
        out_error: *mut CFErrorRef,
    ) -> Boolean;
    pub fn SMJobSubmit(
        domain: CFStringRef,
        job: CFDictionaryRef,
        auth: AuthorizationRef,
        out_error: *mut CFErrorRef,
    ) -> Boolean;
    pub fn SMJobRemove(
        domain: CFStringRef,
        job_label: CFStringRef,
        auth: AuthorizationRef,
        wait: Boolean,
        out_error: *mut CFErrorRef,
    ) -> Boolean;

    pub fn sm_string_free(pointer: *mut c_char);
    pub fn sm_app_service_main_app(error_out: *mut *mut c_char) -> *mut c_void;
    pub fn sm_app_service_login_item(
        identifier: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sm_app_service_agent(
        plist_name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sm_app_service_daemon(
        plist_name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sm_app_service_status(service: *mut c_void) -> i32;
    pub fn sm_app_service_register(service: *mut c_void, error_out: *mut *mut c_char) -> bool;
    #[cfg(feature = "async")]
    pub fn sm_app_service_register_async(
        service: *mut c_void,
        context: *mut c_void,
        callback: SMAppServiceUnitCallback,
    );
    pub fn sm_app_service_unregister(service: *mut c_void, error_out: *mut *mut c_char) -> bool;
    #[cfg(feature = "async")]
    pub fn sm_app_service_unregister_async(
        service: *mut c_void,
        context: *mut c_void,
        callback: SMAppServiceUnitCallback,
    );
    pub fn sm_app_service_unregister_with_completion(
        service: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sm_app_service_error_domain(error_out: *mut *mut c_char) -> *mut c_char;
    pub fn sm_app_service_status_for_legacy_plist(
        path: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    #[cfg(feature = "async")]
    pub fn sm_app_service_status_for_legacy_plist_async(
        path: *const c_char,
        context: *mut c_void,
        callback: SMAppServiceStatusCallback,
    );
    pub fn sm_main_app_service(error_out: *mut *mut c_char) -> *mut c_void;
    pub fn sm_agent_service(plist_name: *const c_char, error_out: *mut *mut c_char) -> *mut c_void;
    pub fn sm_daemon_service(plist_name: *const c_char, error_out: *mut *mut c_char)
        -> *mut c_void;
    pub fn sm_login_item_service(
        identifier: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sm_open_system_settings_login_items(error_out: *mut *mut c_char) -> bool;
    pub fn sm_app_service_release(service: *mut c_void);

    pub fn sm_legacy_login_item_set_enabled(
        identifier: *const c_char,
        enabled: bool,
        error_out: *mut *mut c_char,
    ) -> bool;

    pub fn sm_authorization_create(flags: u32, error_out: *mut *mut c_char) -> *mut c_void;
    pub fn sm_authorization_create_with_rights(
        rights_json: *const c_char,
        flags: u32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sm_authorization_copy_rights(
        authorization: *mut c_void,
        rights_json: *const c_char,
        flags: u32,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sm_authorization_external_form(
        authorization: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn sm_authorization_from_external_form(
        external_form: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sm_authorization_destroy_rights(
        authorization: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sm_authorization_release(authorization: *mut c_void);

    pub fn sm_legacy_copy_job_dictionary(
        domain: i32,
        job_label: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn sm_legacy_copy_all_job_dictionaries(
        domain: i32,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn sm_legacy_job_submit_plist(
        domain: i32,
        plist_xml: *const c_char,
        authorization: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sm_legacy_job_remove(
        domain: i32,
        job_label: *const c_char,
        authorization: *mut c_void,
        wait: bool,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sm_legacy_job_bless(
        domain: i32,
        executable_label: *const c_char,
        authorization: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> bool;
}
