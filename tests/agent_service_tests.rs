use servicemanagement::{AgentService, SMAppServiceStatus};

#[test]
fn agent_service_wrapper_exposes_status() {
    let service = AgentService::new("com.example.agent.plist")
        .expect("agent service wrapper should be constructible");
    assert!(matches!(
        service.status(),
        SMAppServiceStatus::NotRegistered
            | SMAppServiceStatus::Enabled
            | SMAppServiceStatus::RequiresApproval
            | SMAppServiceStatus::NotFound
            | SMAppServiceStatus::Unknown(_)
    ));
}
