#![cfg(feature = "async")]

use servicemanagement::async_api::{status_for_legacy_plist_async, SMAppServiceAsyncExt};
use servicemanagement::SMAppService;
use servicemanagement::SMAppServiceStatus;

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
fn async_unregister_reports_an_error_for_missing_agent() {
    pollster::block_on(async {
        let service = SMAppService::agent("com.example.servicemanagement.tests.agent.plist")
            .expect("agent service should be constructible");
        let error = service
            .unregister_async()
            .await
            .expect_err("unregister should fail for a missing agent plist");
        assert!(!error.message.is_empty());
    });
}

#[test]
fn async_legacy_plist_lookup_returns_a_known_status() {
    pollster::block_on(async {
        let status = status_for_legacy_plist_async(
            "/Library/LaunchDaemons/com.example.servicemanagement.tests.missing.plist",
        )
        .await
        .expect("status lookup should return a status for a missing plist path");
        assert_known_status(status);
    });
}
