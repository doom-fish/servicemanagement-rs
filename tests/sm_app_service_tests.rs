use servicemanagement::{app_service_error_domain, SMAppService, SMAppServiceStatus};

fn assert_known_status(status: SMAppServiceStatus) {
    assert!(matches!(
        status,
        SMAppServiceStatus::NotRegistered
            | SMAppServiceStatus::Enabled
            | SMAppServiceStatus::RequiresApproval
            | SMAppServiceStatus::NotFound
            | SMAppServiceStatus::Unknown(_)
    ));
}

#[test]
fn main_app_status_is_known() {
    let service = SMAppService::main_app().expect("main app service should be constructible");
    assert_known_status(service.status());
}

#[test]
fn completion_based_unregister_reports_an_error_for_missing_agent() {
    let service = SMAppService::agent("com.example.servicemanagement.tests.agent.plist")
        .expect("agent service should be constructible");
    let error = service
        .unregister_with_completion_handler()
        .expect_err("unregister should fail for a missing agent plist");
    assert!(!error.message.is_empty());
}

#[test]
fn app_service_error_domain_is_available_or_version_gated() {
    match app_service_error_domain() {
        Ok(domain) => assert!(!domain.is_empty()),
        Err(error) => assert!(error.message.contains("macOS 15+")),
    }
}
