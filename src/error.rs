use std::{error::Error, fmt};

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
