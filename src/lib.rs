#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::redundant_pub_crate,
    clippy::similar_names,
    clippy::use_self
)]
#![doc = include_str!("../README.md")]

#[cfg(not(target_os = "macos"))]
compile_error!("servicemanagement only supports macOS");

/// ServiceManagement framework agent service wrappers.
pub mod agent_service;
mod app_service;
/// ServiceManagement framework authorization helpers.
pub mod authorization;
mod bridge;
mod cf;
/// ServiceManagement framework daemon service wrappers.
pub mod daemon_service;
mod error;
mod ffi;
/// Legacy ServiceManagement Core Foundation helpers.
pub mod legacy;
/// ServiceManagement framework login item service wrappers.
pub mod login_item;
/// ServiceManagement framework main app service wrappers.
pub mod main_app;
/// Modern `ServiceManagement.SMAppService` bindings.
pub mod sm_app_service;
/// `ServiceManagement.SMAppService.Status` bindings.
pub mod sm_app_service_status;
/// Legacy `ServiceManagement.SMJobBless` helpers.
pub mod sm_job_bless;
/// Legacy `ServiceManagement.SMLoginItemSetEnabled` helpers.
pub mod sm_login_item;

/// Re-exports the ServiceManagement agent service wrapper.
pub use agent_service::AgentService;
/// Re-exports the modern ServiceManagement service aliases.
pub use app_service::{AppService, AppServiceStatus};
/// Re-exports authorization helpers used with ServiceManagement.
pub use authorization::{
    Authorization, AuthorizationFlags, AuthorizationRight, SM_RIGHT_BLESS_PRIVILEGED_HELPER,
    SM_RIGHT_MODIFY_SYSTEM_DAEMONS,
};
/// Re-exports the ServiceManagement daemon service wrapper.
pub use daemon_service::DaemonService;
/// Re-exports ServiceManagement error and domain helpers.
pub use error::{
    legacy_error_domain_framework, legacy_error_domain_ipc, legacy_error_domain_launchd, Result,
    SMErrorCode, ServiceManagementError,
};
/// Re-exports the ServiceManagement login item service wrapper.
pub use login_item::LoginItem;
/// Re-exports the ServiceManagement main app service wrapper.
pub use main_app::MainApp;
/// Re-exports the modern `ServiceManagement.SMAppService` helpers.
pub use sm_app_service::{
    app_service_error_domain, open_system_settings_login_items, SMAppService,
};
/// Re-exports `SMAppService.Status` helpers from ServiceManagement.
pub use sm_app_service_status::{status_for_legacy_plist, SMAppServiceStatus};
/// Re-exports legacy `SMJobBless` helpers from ServiceManagement.
pub use sm_job_bless::{
    bless, copy_all_job_dictionaries, copy_job_dictionary, job_remove, job_submit_plist,
    LaunchdDomain, LegacyJobDictionary, SMJobBless,
};
/// Re-exports the legacy `SMLoginItemSetEnabled` wrapper.
pub use sm_login_item::SMLoginItem;
