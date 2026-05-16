use servicemanagement::{status_for_legacy_plist, SMAppServiceStatus};

#[test]
fn status_values_match_sdk_raw_values() {
    assert_eq!(SMAppServiceStatus::NotRegistered.raw_value(), 0);
    assert_eq!(SMAppServiceStatus::Enabled.raw_value(), 1);
    assert_eq!(SMAppServiceStatus::RequiresApproval.raw_value(), 2);
    assert_eq!(SMAppServiceStatus::NotFound.raw_value(), 3);
    assert_eq!(
        SMAppServiceStatus::RequiresApproval.as_str(),
        "requiresApproval"
    );
}

#[test]
fn legacy_plist_lookup_returns_a_known_status() {
    let status = status_for_legacy_plist(
        "/Library/LaunchDaemons/com.example.servicemanagement.tests.missing.plist",
    )
    .expect("status lookup should return a status for a missing plist path");
    assert!(matches!(
        status,
        SMAppServiceStatus::NotRegistered
            | SMAppServiceStatus::Enabled
            | SMAppServiceStatus::RequiresApproval
            | SMAppServiceStatus::NotFound
            | SMAppServiceStatus::Unknown(_)
    ));
}
