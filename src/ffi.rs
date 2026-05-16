#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_void};

pub type AuthorizationRef = *mut c_void;
pub type Boolean = u8;
pub type CFArrayRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFErrorRef = *const c_void;
pub type CFStringRef = *const c_void;

unsafe extern "C" {
    pub static kSMDomainSystemLaunchd: CFStringRef;
    pub static kSMDomainUserLaunchd: CFStringRef;

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
    pub fn sm_app_service_unregister(service: *mut c_void, error_out: *mut *mut c_char) -> bool;
    pub fn sm_open_system_settings_login_items(error_out: *mut *mut c_char) -> bool;
    pub fn sm_app_service_release(service: *mut c_void);
}
