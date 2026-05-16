use servicemanagement::{MainApp, SMAppServiceStatus};

#[test]
fn main_app_wrapper_exposes_status() {
    let service = MainApp::new().expect("main app wrapper should be constructible");
    assert!(matches!(
        service.status(),
        SMAppServiceStatus::NotRegistered
            | SMAppServiceStatus::Enabled
            | SMAppServiceStatus::RequiresApproval
            | SMAppServiceStatus::NotFound
            | SMAppServiceStatus::Unknown(_)
    ));
}
