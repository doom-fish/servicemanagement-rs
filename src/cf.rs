use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr::NonNull,
};

use crate::{ffi, Result, ServiceManagementError};

const CF_STRING_ENCODING_UTF8: u32 = 0x0800_0100;

#[derive(Debug)]
pub(crate) struct OwnedCFType(NonNull<c_void>);

impl OwnedCFType {
    pub(crate) fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr().cast_const()
    }

    pub(crate) unsafe fn from_create_rule(raw: *const c_void) -> Option<Self> {
        NonNull::new(raw.cast_mut()).map(Self)
    }
}

impl Clone for OwnedCFType {
    fn clone(&self) -> Self {
        let retained = unsafe { ffi::CFRetain(self.as_ptr()) };
        unsafe { Self::from_create_rule(retained) }.expect("CFRetain returned null")
    }
}

impl Drop for OwnedCFType {
    fn drop(&mut self) {
        unsafe { ffi::CFRelease(self.as_ptr()) };
    }
}

pub(crate) fn cfstring_from_str(value: &str) -> Result<OwnedCFType> {
    let c_string = CString::new(value).map_err(|_| {
        ServiceManagementError::new(
            "CFStringCreateWithCString",
            "strings passed to CoreFoundation cannot contain interior NUL bytes",
        )
    })?;
    let string_ref = unsafe {
        ffi::CFStringCreateWithCString(std::ptr::null(), c_string.as_ptr(), CF_STRING_ENCODING_UTF8)
    };
    unsafe { OwnedCFType::from_create_rule(string_ref) }.ok_or_else(|| {
        ServiceManagementError::new(
            "CFStringCreateWithCString",
            format!("could not create CFString for {value:?}"),
        )
    })
}

pub(crate) fn cfstring_to_string(string_ref: *const c_void) -> Result<String> {
    let length = unsafe { ffi::CFStringGetLength(string_ref) };
    let capacity =
        unsafe { ffi::CFStringGetMaximumSizeForEncoding(length, CF_STRING_ENCODING_UTF8) } + 1;
    let mut buffer = vec![0_u8; usize::try_from(capacity).expect("negative CFString capacity")];
    let ok = unsafe {
        ffi::CFStringGetCString(
            string_ref,
            buffer.as_mut_ptr().cast::<c_char>(),
            capacity,
            CF_STRING_ENCODING_UTF8,
        )
    };
    if ok == 0 {
        return Err(ServiceManagementError::new(
            "CFStringGetCString",
            "CoreFoundation rejected UTF-8 conversion",
        ));
    }
    let c_string = CStr::from_bytes_until_nul(&buffer).map_err(|_| {
        ServiceManagementError::new(
            "CFStringGetCString",
            "CoreFoundation returned a non-NUL-terminated string",
        )
    })?;
    Ok(c_string.to_string_lossy().into_owned())
}

pub(crate) fn copy_description(value: *const c_void) -> Result<String> {
    let description_ref = unsafe { ffi::CFCopyDescription(value) };
    let description =
        unsafe { OwnedCFType::from_create_rule(description_ref) }.ok_or_else(|| {
            ServiceManagementError::new("CFCopyDescription", "received null description")
        })?;
    cfstring_to_string(description.as_ptr())
}

pub(crate) fn cfarray_descriptions(array_ref: *const c_void) -> Result<Vec<String>> {
    let array = unsafe { OwnedCFType::from_create_rule(array_ref) }.ok_or_else(|| {
        ServiceManagementError::new("SMCopyAllJobDictionaries", "received null CFArray")
    })?;
    let count = unsafe { ffi::CFArrayGetCount(array.as_ptr()) };
    (0..count)
        .map(|index| {
            let value = unsafe { ffi::CFArrayGetValueAtIndex(array.as_ptr(), index) };
            copy_description(value)
        })
        .collect()
}
