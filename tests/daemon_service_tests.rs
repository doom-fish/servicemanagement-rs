use servicemanagement::{DaemonService, SMAppServiceStatus};

#[test]
fn daemon_service_wrapper_exposes_status() {
    let service = DaemonService::new("com.example.daemon.plist")
        .expect("daemon service wrapper should be constructible");
    assert!(matches!(
        service.status().expect("status should be readable"),
        SMAppServiceStatus::NotRegistered
            | SMAppServiceStatus::Enabled
            | SMAppServiceStatus::RequiresApproval
            | SMAppServiceStatus::NotFound
            | SMAppServiceStatus::Unknown(_)
    ));
}
