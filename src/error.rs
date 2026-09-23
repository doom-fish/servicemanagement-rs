use std::{error::Error, fmt};

use serde::Deserialize;

use apple_cf::cf::CFString;

use crate::ffi;

const SM_APP_SERVICE_ERROR_DOMAIN: &str = "SMAppServiceErrorDomain";
const SM_ERROR_DOMAIN_FRAMEWORK: &str = "kSMErrorDomainFramework";
const TIMED_OUT: i64 = 60;

/// Result type returned by ServiceManagement framework wrappers.
pub type Result<T> = std::result::Result<T, ServiceManagementError>;

/// Error returned by a ServiceManagement framework wrapper call.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct ServiceManagementError {
    /// Name of the failing ServiceManagement bridge function.
    pub function: &'static str,
    /// Human-readable message returned by the bridge or framework.
    pub message: String,
    pub domain: Option<String>,
    pub code: Option<i64>,
}

#[derive(Deserialize)]
struct BridgeErrorPayload {
    message: String,
    domain: String,
    code: i64,
}

impl ServiceManagementError {
    pub(crate) fn new(function: &'static str, message: impl Into<String>) -> Self {
        Self {
            function,
            message: message.into(),
            domain: None,
            code: None,
        }
    }

    pub(crate) fn with_domain_and_code(
        function: &'static str,
        message: impl Into<String>,
        domain: impl Into<String>,
        code: i64,
    ) -> Self {
        Self {
            function,
            message: message.into(),
            domain: Some(domain.into()),
            code: Some(code),
        }
    }

    pub(crate) fn from_bridge_message(function: &'static str, message: String) -> Self {
        if message.starts_with('{') {
            if let Ok(payload) = serde_json::from_str::<BridgeErrorPayload>(&message) {
                return Self::with_domain_and_code(
                    function,
                    payload.message,
                    payload.domain,
                    payload.code,
                );
            }
        }
        Self::new(function, message)
    }

    pub fn is_timeout(&self) -> bool {
        self.domain.as_deref() == Some("NSPOSIXErrorDomain") && self.code == Some(TIMED_OUT)
    }

    pub fn sm_error_code(&self) -> Option<SMErrorCode> {
        match self.domain.as_deref() {
            Some(SM_APP_SERVICE_ERROR_DOMAIN | SM_ERROR_DOMAIN_FRAMEWORK) => self
                .code
                .and_then(|code| i32::try_from(code).ok())
                .and_then(SMErrorCode::from_raw),
            _ => None,
        }
    }
}

impl fmt::Display for ServiceManagementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} failed: {}", self.function, self.message)?;
        if let (Some(domain), Some(code)) = (&self.domain, self.code) {
            write!(f, " ({domain} {code})")?;
        }
        Ok(())
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
    // SAFETY: domain is one of the framework's static CFStringRef constants (or null, which
    // from_raw_borrowed maps to None); retaining it for the duration of the read is valid.
    unsafe { CFString::from_raw_borrowed(domain.cast_mut().cast()) }
        .map(|domain| domain.to_string_lossy())
        .ok_or_else(|| ServiceManagementError::new(function, "received a null error domain"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_payloads_keep_the_domain_and_code() {
        let error = ServiceManagementError::from_bridge_message(
            "sm_app_service_register",
            r#"{"code":11,"domain":"SMAppServiceErrorDomain","message":"denied"}"#.to_owned(),
        );
        assert_eq!(error.message, "denied");
        assert_eq!(error.domain.as_deref(), Some("SMAppServiceErrorDomain"));
        assert_eq!(error.code, Some(11));
        assert_eq!(error.sm_error_code(), Some(SMErrorCode::LaunchDeniedByUser));
        assert_eq!(
            error.to_string(),
            "sm_app_service_register failed: denied (SMAppServiceErrorDomain 11)"
        );
    }

    #[test]
    fn service_management_codes_map_only_in_service_management_domains() {
        for (domain, code, expected) in [
            (
                "SMAppServiceErrorDomain",
                12,
                Some(SMErrorCode::AlreadyRegistered),
            ),
            (
                "kSMErrorDomainFramework",
                3,
                Some(SMErrorCode::InvalidSignature),
            ),
            ("SMAppServiceErrorDomain", 22, None),
            ("NSPOSIXErrorDomain", 11, None),
            ("kSMErrorDomainLaunchd", 3, None),
        ] {
            let error = ServiceManagementError::with_domain_and_code("op", "message", domain, code);
            assert_eq!(error.sm_error_code(), expected, "{domain} {code}");
        }
    }

    #[test]
    fn plain_bridge_messages_have_no_code() {
        let error = ServiceManagementError::from_bridge_message("op", "plain failure".to_owned());
        assert_eq!(error.message, "plain failure");
        assert_eq!(error.domain, None);
        assert_eq!(error.code, None);
        assert_eq!(error.sm_error_code(), None);
        assert_eq!(error.to_string(), "op failed: plain failure");
    }
}
