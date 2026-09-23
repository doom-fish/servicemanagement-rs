use crate::{
    bridge::{bridge_error, c_string},
    ffi, Result,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
/// Namespace for legacy `ServiceManagement.SMLoginItemSetEnabled` helpers.
pub struct SMLoginItem;

impl SMLoginItem {
    #[deprecated(
        since = "0.5.0",
        note = "SMLoginItemSetEnabled is deprecated since macOS 13; use SMAppService::login_item with register and unregister instead"
    )]
    /// Calls legacy `SMLoginItemSetEnabled` for a login item bundle identifier.
    pub fn set_enabled(identifier: &str, enabled: bool) -> Result<()> {
        let identifier = c_string(identifier, "sm_legacy_login_item_set_enabled")?;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::sm_legacy_login_item_set_enabled(identifier.as_ptr(), enabled, &raw mut error)
        };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sm_legacy_login_item_set_enabled", error))
        }
    }
}

#[deprecated(
    since = "0.5.0",
    note = "SMLoginItemSetEnabled is deprecated since macOS 13; use SMAppService::login_item with register and unregister instead"
)]
#[allow(deprecated)]
/// Convenience wrapper around `SMLoginItem::set_enabled`.
pub fn set_enabled(identifier: &str, enabled: bool) -> Result<()> {
    SMLoginItem::set_enabled(identifier, enabled)
}
