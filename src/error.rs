use std::{error::Error, fmt};

use crate::{cf::cfstring_to_string, ffi};

/// Result type returned by ServiceManagement framework wrappers.
pub type Result<T> = std::result::Result<T, ServiceManagementError>;

/// Error returned by a ServiceManagement framework wrapper call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceManagementError {
    /// Name of the failing ServiceManagement bridge function.
    pub function: &'static str,
    /// Human-readable message returned by the bridge or framework.
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

/// Known legacy `ServiceManagement` error codes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum SMErrorCode {
    /// Corresponds to `kSMErrorInternalFailure`.
    InternalFailure = 2,
    /// Corresponds to `kSMErrorInvalidSignature`.
    InvalidSignature = 3,
    /// Corresponds to `kSMErrorAuthorizationFailure`.
    AuthorizationFailure = 4,
    /// Corresponds to `kSMErrorToolNotValid`.
    ToolNotValid = 5,
    /// Corresponds to `kSMErrorJobNotFound`.
    JobNotFound = 6,
    /// Corresponds to `kSMErrorServiceUnavailable`.
    ServiceUnavailable = 7,
    /// Corresponds to `kSMErrorJobPlistNotFound`.
    JobPlistNotFound = 8,
    /// Corresponds to `kSMErrorJobMustBeEnabled`.
    JobMustBeEnabled = 9,
    /// Corresponds to `kSMErrorInvalidPlist`.
    InvalidPlist = 10,
    /// Corresponds to `kSMErrorLaunchDeniedByUser`.
    LaunchDeniedByUser = 11,
    /// Corresponds to `kSMErrorAlreadyRegistered`.
    AlreadyRegistered = 12,
}

impl SMErrorCode {
    /// Converts a raw ServiceManagement legacy error code into a typed value.
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

    /// Returns the raw ServiceManagement legacy error code.
    pub const fn raw_value(self) -> i32 {
        self as i32
    }

    /// Returns the canonical ServiceManagement legacy error code name.
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

/// Returns the legacy `kSMErrorDomainIPC` string from ServiceManagement.
pub fn legacy_error_domain_ipc() -> Result<String> {
    // SAFETY: kSMErrorDomainIPC is a static CFStringRef constant defined in the FFI module.
    // It is always valid and never null.
    legacy_error_domain(unsafe { ffi::kSMErrorDomainIPC }, "kSMErrorDomainIPC")
}

/// Returns the legacy `kSMErrorDomainFramework` string from ServiceManagement.
pub fn legacy_error_domain_framework() -> Result<String> {
    // SAFETY: kSMErrorDomainFramework is a static CFStringRef constant defined in the FFI
    // module. It is always valid and never null.
    legacy_error_domain(
        unsafe { ffi::kSMErrorDomainFramework },
        "kSMErrorDomainFramework",
    )
}

/// Returns the legacy `kSMErrorDomainLaunchd` string from ServiceManagement.
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
