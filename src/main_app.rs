use std::ops::Deref;

use crate::{ffi, Result, SMAppService};

#[derive(Debug)]
pub struct MainApp(SMAppService);

impl MainApp {
    pub fn new() -> Result<Self> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_main_app_service(&mut error) };
        SMAppService::from_raw(raw, error, "sm_main_app_service").map(Self)
    }

    pub fn as_app_service(&self) -> &SMAppService {
        &self.0
    }

    pub fn into_app_service(self) -> SMAppService {
        self.0
    }
}

impl Deref for MainApp {
    type Target = SMAppService;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<MainApp> for SMAppService {
    fn from(value: MainApp) -> Self {
        value.0
    }
}
