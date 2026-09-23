#![allow(deprecated)]

use servicemanagement::SMLoginItem;

#[test]
#[ignore = "calls SMLoginItemSetEnabled, which changes the user's login items"]
fn invalid_legacy_login_item_reports_an_error() {
    let error = SMLoginItem::set_enabled("com.example.legacy-login-item", false)
        .expect_err("missing login item should fail cleanly");
    assert!(error.message.contains("SMLoginItemSetEnabled"));
}
