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

pub mod agent_service;
mod app_service;
pub mod authorization;
mod bridge;
mod cf;
pub mod daemon_service;
mod error;
mod ffi;
pub mod legacy;
pub mod login_item;
pub mod main_app;
pub mod sm_app_service;
pub mod sm_app_service_status;
pub mod sm_job_bless;
pub mod sm_login_item;

pub use agent_service::AgentService;
pub use app_service::{AppService, AppServiceStatus};
pub use authorization::{
    Authorization, AuthorizationFlags, AuthorizationRight, SM_RIGHT_BLESS_PRIVILEGED_HELPER,
    SM_RIGHT_MODIFY_SYSTEM_DAEMONS,
};
pub use daemon_service::DaemonService;
pub use error::{
    legacy_error_domain_framework, legacy_error_domain_ipc, legacy_error_domain_launchd, Result,
    SMErrorCode, ServiceManagementError,
};
pub use login_item::LoginItem;
pub use main_app::MainApp;
pub use sm_app_service::{
    app_service_error_domain, open_system_settings_login_items, SMAppService,
};
pub use sm_app_service_status::{status_for_legacy_plist, SMAppServiceStatus};
pub use sm_job_bless::{
    bless, copy_all_job_dictionaries, copy_job_dictionary, job_remove, job_submit_plist,
    LaunchdDomain, LegacyJobDictionary, SMJobBless,
};
pub use sm_login_item::SMLoginItem;
