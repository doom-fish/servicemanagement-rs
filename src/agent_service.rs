use std::ops::Deref;

use crate::{bridge::c_string, ffi, Result, SMAppService};

#[derive(Debug)]
pub struct AgentService(SMAppService);

impl AgentService {
    pub fn new(plist_name: &str) -> Result<Self> {
        let plist_name = c_string(plist_name, "sm_agent_service")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sm_agent_service(plist_name.as_ptr(), &mut error) };
        SMAppService::from_raw(raw, error, "sm_agent_service").map(Self)
    }

    pub fn as_app_service(&self) -> &SMAppService {
        &self.0
    }

    pub fn into_app_service(self) -> SMAppService {
        self.0
    }
}

impl Deref for AgentService {
    type Target = SMAppService;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<AgentService> for SMAppService {
    fn from(value: AgentService) -> Self {
        value.0
    }
}
