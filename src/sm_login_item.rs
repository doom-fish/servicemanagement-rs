use crate::{
    bridge::{bridge_error, c_string},
    ffi, Result,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SMLoginItem;

impl SMLoginItem {
    pub fn set_enabled(identifier: &str, enabled: bool) -> Result<()> {
        let identifier = c_string(identifier, "sm_legacy_login_item_set_enabled")?;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::sm_legacy_login_item_set_enabled(identifier.as_ptr(), enabled, &mut error)
        };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_legacy_login_item_set_enabled", error))
        }
    }
}

pub fn set_enabled(identifier: &str, enabled: bool) -> Result<()> {
    SMLoginItem::set_enabled(identifier, enabled)
}
