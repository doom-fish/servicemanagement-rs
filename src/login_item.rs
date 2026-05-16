use std::ops::Deref;

use crate::{bridge::c_string, ffi, Result, SMAppService};

#[derive(Debug)]
pub struct LoginItem(SMAppService);

impl LoginItem {
    pub fn new(identifier: &str) -> Result<Self> {
        let identifier = c_string(identifier, "sm_login_item_service")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_login_item_service(identifier.as_ptr(), &mut error) };
        SMAppService::from_raw(raw, error, "sm_login_item_service").map(Self)
    }

    pub fn as_app_service(&self) -> &SMAppService {
        &self.0
    }

    pub fn into_app_service(self) -> SMAppService {
        self.0
    }
}

impl Deref for LoginItem {
    type Target = SMAppService;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<LoginItem> for SMAppService {
    fn from(value: LoginItem) -> Self {
        value.0
    }
}
