use servicemanagement::{LoginItem, SMAppServiceStatus};

#[test]
fn login_item_wrapper_exposes_status() {
    let service = LoginItem::new("com.example.login-item")
        .expect("login item wrapper should be constructible");
    assert!(matches!(
        service.status(),
        SMAppServiceStatus::NotRegistered
            | SMAppServiceStatus::Enabled
            | SMAppServiceStatus::RequiresApproval
            | SMAppServiceStatus::NotFound
            | SMAppServiceStatus::Unknown(_)
    ));
}
