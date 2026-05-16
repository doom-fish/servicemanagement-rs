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

mod app_service;
mod cf;
mod error;
mod ffi;
pub mod legacy;

pub use app_service::{open_system_settings_login_items, AppService, AppServiceStatus};
pub use error::{Result, ServiceManagementError};
