use std::{error::Error, fmt};

use crate::{cf::cfstring_to_string, ffi};

pub type Result<T> = std::result::Result<T, ServiceManagementError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceManagementError {
    pub function: &'static str,
    pub message: String,
}

impl ServiceManagementError {
    pub(crate) fn new(function: &'static str, message: impl Into<String>) -> Self {
        Self {
            function,
            message: message.into(),
        }
    }
}

impl fmt::Display for ServiceManagementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} failed: {}", self.function, self.message)
    }
}

impl Error for ServiceManagementError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum SMErrorCode {
    InternalFailure = 2,
    InvalidSignature = 3,
    AuthorizationFailure = 4,
    ToolNotValid = 5,
    JobNotFound = 6,
    ServiceUnavailable = 7,
    JobPlistNotFound = 8,
    JobMustBeEnabled = 9,
    InvalidPlist = 10,
    LaunchDeniedByUser = 11,
    AlreadyRegistered = 12,
}

impl SMErrorCode {
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            2 => Some(Self::InternalFailure),
            3 => Some(Self::InvalidSignature),
            4 => Some(Self::AuthorizationFailure),
            5 => Some(Self::ToolNotValid),
            6 => Some(Self::JobNotFound),
            7 => Some(Self::ServiceUnavailable),
            8 => Some(Self::JobPlistNotFound),
            9 => Some(Self::JobMustBeEnabled),
            10 => Some(Self::InvalidPlist),
            11 => Some(Self::LaunchDeniedByUser),
            12 => Some(Self::AlreadyRegistered),
            _ => None,
        }
    }

    pub const fn raw_value(self) -> i32 {
        self as i32
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InternalFailure => "internalFailure",
            Self::InvalidSignature => "invalidSignature",
            Self::AuthorizationFailure => "authorizationFailure",
            Self::ToolNotValid => "toolNotValid",
            Self::JobNotFound => "jobNotFound",
            Self::ServiceUnavailable => "serviceUnavailable",
            Self::JobPlistNotFound => "jobPlistNotFound",
            Self::JobMustBeEnabled => "jobMustBeEnabled",
            Self::InvalidPlist => "invalidPlist",
            Self::LaunchDeniedByUser => "launchDeniedByUser",
            Self::AlreadyRegistered => "alreadyRegistered",
        }
    }
}

pub fn legacy_error_domain_ipc() -> Result<String> {
    // SAFETY: kSMErrorDomainIPC is a static CFStringRef constant defined in the FFI module.
    // It is always valid and never null.
    legacy_error_domain(unsafe { ffi::kSMErrorDomainIPC }, "kSMErrorDomainIPC")
}

pub fn legacy_error_domain_framework() -> Result<String> {
    // SAFETY: kSMErrorDomainFramework is a static CFStringRef constant defined in the FFI
    // module. It is always valid and never null.
    legacy_error_domain(
        unsafe { ffi::kSMErrorDomainFramework },
        "kSMErrorDomainFramework",
    )
}

pub fn legacy_error_domain_launchd() -> Result<String> {
    // SAFETY: kSMErrorDomainLaunchd is a static CFStringRef constant defined in the FFI
    // module. It is always valid and never null.
    legacy_error_domain(
        unsafe { ffi::kSMErrorDomainLaunchd },
        "kSMErrorDomainLaunchd",
    )
}

fn legacy_error_domain(domain: ffi::CFStringRef, function: &'static str) -> Result<String> {
    if domain.is_null() {
        return Err(ServiceManagementError::new(
            function,
            "received a null error domain",
        ));
    }

    cfstring_to_string(domain)
}
