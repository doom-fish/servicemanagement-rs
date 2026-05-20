//! Executor-agnostic async wrappers for `ServiceManagement` APIs.
//!
//! Enabled with the `async` Cargo feature. These wrappers use
//! `doom_fish_utils::completion::AsyncCompletion` so they work with any async
//! runtime.
//!
//! Wrapped operations:
//! - [`SMAppServiceAsyncExt::register_async`] for `SMAppService.register()`
//! - [`SMAppServiceAsyncExt::unregister_async`] for `SMAppService.unregisterWithCompletionHandler(_:)`
//! - [`status_for_legacy_plist_async`] for `SMAppService.statusForLegacyPlist(at:)`
//!
//! # Example
//!
//! ```rust,no_run
//! use servicemanagement::SMAppService;
//! use servicemanagement::async_api::{status_for_legacy_plist_async, SMAppServiceAsyncExt};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! pollster::block_on(async {
//!     let service = SMAppService::agent("com.example.servicemanagement.example.agent.plist")?;
//!     let _ = service.unregister_async().await;
//!
//!     let status = status_for_legacy_plist_async(
//!         "/Library/LaunchDaemons/com.example.servicemanagement.example.plist",
//!     )
//!     .await?;
//!     println!("{}", status.as_str());
//!     Ok::<(), Box<dyn std::error::Error>>(())
//! })
//! # }
//! ```

#![cfg(feature = "async")]

use std::{
    ffi::{c_char, c_void},
    future::Future,
    ops::Deref,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

use doom_fish_utils::{
    completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture},
    panic_safe::catch_user_panic,
};

use crate::{
    bridge::path_c_string, ffi, Result, SMAppService, SMAppServiceStatus, ServiceManagementError,
};

const REGISTER_FUNCTION: &str = "sm_app_service_register_async";
const UNREGISTER_FUNCTION: &str = "sm_app_service_unregister_async";
const STATUS_FOR_LEGACY_PLIST_FUNCTION: &str = "sm_app_service_status_for_legacy_plist_async";

fn ready_error_future<T>(error: ServiceManagementError) -> AsyncCompletionFuture<T> {
    let (future, context) = AsyncCompletion::create();
    // SAFETY: `context` comes from `AsyncCompletion::create()` and is consumed exactly once here.
    unsafe { AsyncCompletion::<T>::complete_err(context, error.message) };
    future
}

fn bridge_error_message(error: *mut c_char) -> String {
    let message = unsafe { error_from_cstr(error.cast_const()) };
    unsafe { ffi::sm_string_free(error) };
    message
}

fn poll_unit_future(
    inner: &mut AsyncCompletionFuture<()>,
    cx: &mut Context<'_>,
    function: &'static str,
) -> Poll<Result<()>> {
    Pin::new(inner)
        .poll(cx)
        .map(|result| result.map_err(|message| ServiceManagementError::new(function, message)))
}

fn poll_status_future(
    inner: &mut AsyncCompletionFuture<i32>,
    cx: &mut Context<'_>,
) -> Poll<Result<SMAppServiceStatus>> {
    Pin::new(inner).poll(cx).map(|result| match result {
        Ok(raw) if raw >= 0 => Ok(SMAppServiceStatus::from_raw(raw)),
        Ok(raw) => Err(ServiceManagementError::new(
            STATUS_FOR_LEGACY_PLIST_FUNCTION,
            format!("bridge returned an invalid status value {raw}"),
        )),
        Err(message) => Err(ServiceManagementError::new(
            STATUS_FOR_LEGACY_PLIST_FUNCTION,
            message,
        )),
    })
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
unsafe extern "C" fn unit_callback(context: *mut c_void, error: *mut c_char) {
    catch_user_panic("servicemanagement::async_api::unit_callback", || {
        if error.is_null() {
            // SAFETY: `context` comes from `AsyncCompletion::create()` and is consumed once here.
            unsafe { AsyncCompletion::<()>::complete_ok(context, ()) };
        } else {
            let message = bridge_error_message(error);
            // SAFETY: `context` comes from `AsyncCompletion::create()` and is consumed once here.
            unsafe { AsyncCompletion::<()>::complete_err(context, message) };
        }
    });
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
unsafe extern "C" fn status_callback(context: *mut c_void, status: i32, error: *mut c_char) {
    catch_user_panic("servicemanagement::async_api::status_callback", || {
        if error.is_null() {
            // SAFETY: `context` comes from `AsyncCompletion::create()` and is consumed once here.
            unsafe { AsyncCompletion::<i32>::complete_ok(context, status) };
        } else {
            let message = bridge_error_message(error);
            // SAFETY: `context` comes from `AsyncCompletion::create()` and is consumed once here.
            unsafe { AsyncCompletion::<i32>::complete_err(context, message) };
        }
    });
}

fn register_future(service: &SMAppService) -> RegisterFuture {
    let (future, context) = AsyncCompletion::create();
    // SAFETY: `service.as_ptr()` is a live bridged `SMAppService` handle; `context` is consumed
    // exactly once by `unit_callback`.
    unsafe { ffi::sm_app_service_register_async(service.as_ptr(), context, unit_callback) };
    RegisterFuture { inner: future }
}

fn unregister_future(service: &SMAppService) -> UnregisterFuture {
    let (future, context) = AsyncCompletion::create();
    // SAFETY: `service.as_ptr()` is a live bridged `SMAppService` handle; `context` is consumed
    // exactly once by `unit_callback`.
    unsafe { ffi::sm_app_service_unregister_async(service.as_ptr(), context, unit_callback) };
    UnregisterFuture { inner: future }
}

/// Future returned by [`SMAppServiceAsyncExt::register_async`].
pub struct RegisterFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for RegisterFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterFuture").finish_non_exhaustive()
    }
}

impl Future for RegisterFuture {
    type Output = Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        poll_unit_future(&mut self.inner, cx, REGISTER_FUNCTION)
    }
}

/// Future returned by [`SMAppServiceAsyncExt::unregister_async`].
pub struct UnregisterFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for UnregisterFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnregisterFuture").finish_non_exhaustive()
    }
}

impl Future for UnregisterFuture {
    type Output = Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        poll_unit_future(&mut self.inner, cx, UNREGISTER_FUNCTION)
    }
}

/// Future returned by [`status_for_legacy_plist_async`].
pub struct StatusForLegacyPlistFuture {
    inner: AsyncCompletionFuture<i32>,
}

impl std::fmt::Debug for StatusForLegacyPlistFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusForLegacyPlistFuture")
            .finish_non_exhaustive()
    }
}

impl Future for StatusForLegacyPlistFuture {
    type Output = Result<SMAppServiceStatus>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        poll_status_future(&mut self.inner, cx)
    }
}

/// Async extension methods for [`SMAppService`] and wrapper types that deref to it.
pub trait SMAppServiceAsyncExt {
    /// Executor-agnostic async wrapper for `SMAppService.register()`.
    fn register_async(&self) -> RegisterFuture;

    /// Executor-agnostic async wrapper for `SMAppService.unregisterWithCompletionHandler(_:)`.
    fn unregister_async(&self) -> UnregisterFuture;
}

impl SMAppServiceAsyncExt for SMAppService {
    fn register_async(&self) -> RegisterFuture {
        register_future(self)
    }

    fn unregister_async(&self) -> UnregisterFuture {
        unregister_future(self)
    }
}

impl<T> SMAppServiceAsyncExt for T
where
    T: Deref<Target = SMAppService>,
{
    fn register_async(&self) -> RegisterFuture {
        register_future(self)
    }

    fn unregister_async(&self) -> UnregisterFuture {
        unregister_future(self)
    }
}

/// Executor-agnostic async wrapper for `SMAppService.statusForLegacyPlist(at:)`.
#[must_use]
pub fn status_for_legacy_plist_async(path: impl AsRef<Path>) -> StatusForLegacyPlistFuture {
    let path = match path_c_string(path.as_ref(), STATUS_FOR_LEGACY_PLIST_FUNCTION) {
        Ok(path) => path,
        Err(error) => {
            return StatusForLegacyPlistFuture {
                inner: ready_error_future(error),
            };
        }
    };

    let (future, context) = AsyncCompletion::create();
    // SAFETY: `path.as_ptr()` points to a valid, NUL-terminated path string for the duration of
    // the call; `context` is consumed exactly once by `status_callback`.
    unsafe {
        ffi::sm_app_service_status_for_legacy_plist_async(path.as_ptr(), context, status_callback);
    }
    StatusForLegacyPlistFuture { inner: future }
}
