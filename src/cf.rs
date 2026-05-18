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

    pub(crate) unsafe fn from_create_rule<T>(raw: *const T) -> Option<Self> {
        NonNull::new(raw.cast_mut().cast()).map(Self)
    }
}

impl Clone for OwnedCFType {
    fn clone(&self) -> Self {
        // SAFETY: self.0 is a valid NonNull CFType pointer. CFRetain returns a retained
        // reference to the same object (or null on allocation failure, which we panic on).
        let retained = unsafe { ffi::CFRetain(self.as_ptr()) };
        // SAFETY: CFRetain returns a valid pointer (we panic if null). from_create_rule is
        // safe because we know retained is non-null and valid.
        unsafe { Self::from_create_rule(retained) }.expect("CFRetain returned null")
    }
}

impl Drop for OwnedCFType {
    fn drop(&mut self) {
        // SAFETY: self.0 is a valid NonNull CFType pointer. CFRelease is the inverse of
        // CFRetain and is safe to call on valid pointers; it handles null internally but
        // we only call it on NonNull.
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
    // SAFETY: c_string.as_ptr() is a valid nul-terminated C string. CFStringCreateWithCString
    // returns a valid CFString pointer or null on allocation failure, which we check below.
    let string_ref = unsafe {
        ffi::CFStringCreateWithCString(std::ptr::null(), c_string.as_ptr(), CF_STRING_ENCODING_UTF8)
    };
    // SAFETY: We check for null via ok_or_else, so this call only happens when string_ref
    // is valid.
    unsafe { OwnedCFType::from_create_rule(string_ref) }.ok_or_else(|| {
        ServiceManagementError::new(
            "CFStringCreateWithCString",
            format!("could not create CFString for {value:?}"),
        )
    })
}

pub(crate) fn cfstring_to_string(string_ref: ffi::CFStringRef) -> Result<String> {
    // SAFETY: string_ref is assumed to be a valid CFString pointer (guaranteed by callers).
    // CFStringGetLength returns a CFIndex (valid) and cannot fail.
    let length = unsafe { ffi::CFStringGetLength(string_ref) };
    // SAFETY: string_ref is valid. CFStringGetMaximumSizeForEncoding returns buffer size needed.
    let capacity =
        unsafe { ffi::CFStringGetMaximumSizeForEncoding(length, CF_STRING_ENCODING_UTF8) } + 1;
    let mut buffer = vec![0_u8; usize::try_from(capacity).expect("negative CFString capacity")];
    // SAFETY: buffer.as_mut_ptr() is a valid mutable pointer to the allocated buffer.
    // string_ref is a valid CFString. CFStringGetCString extracts UTF-8 bytes.
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
    // SAFETY: value is assumed to be a valid CFType pointer (guaranteed by callers).
    // CFCopyDescription returns a CFString or null on allocation failure.
    let description_ref = unsafe { ffi::CFCopyDescription(value) };
    // SAFETY: We check for null via ok_or_else, so from_create_rule only called on valid ptr.
    let description =
        unsafe { OwnedCFType::from_create_rule(description_ref) }.ok_or_else(|| {
            ServiceManagementError::new("CFCopyDescription", "received null description")
        })?;
    cfstring_to_string(description.as_ptr().cast())
}

pub(crate) fn cfarray_descriptions(array_ref: ffi::CFArrayRef) -> Result<Vec<String>> {
    // SAFETY: We check for null via ok_or_else, so from_create_rule only called on valid ptr.
    let array = unsafe { OwnedCFType::from_create_rule(array_ref) }.ok_or_else(|| {
        ServiceManagementError::new("SMCopyAllJobDictionaries", "received null CFArray")
    })?;
    // SAFETY: array.as_ptr() is a valid CFArray pointer. CFArrayGetCount returns count.
    let count = unsafe { ffi::CFArrayGetCount(array.as_ptr().cast()) };
    (0..count)
        .map(|index| {
            // SAFETY: array.as_ptr() is a valid CFArray. CFArrayGetValueAtIndex returns
            // a pointer to a CFValue (dictionary) within the array.
            let value = unsafe { ffi::CFArrayGetValueAtIndex(array.as_ptr().cast(), index) };
            copy_description(value)
        })
        .collect()
}
