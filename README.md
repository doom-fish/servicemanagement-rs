# servicemanagement-rs

Safe Rust bindings for Apple’s `ServiceManagement.framework` on macOS.

`servicemanagement-rs` wraps every public symbol in Apple’s ServiceManagement
headers (see [`COVERAGE.md`](COVERAGE.md)), split into focused Swift bridge
files and Rust modules for each logical area:

- `SMAppService`
- `SMAppServiceStatus`
- `MainApp`
- `AgentService`
- `DaemonService`
- `LoginItem`
- legacy `SMLoginItem`
- legacy `SMJobBless` / launchd job helpers
- `Authorization`

It also preserves the original low-level `legacy` module for callers that still
need raw `CFDictionaryRef` / `AuthorizationRef` access, with the Core
Foundation typedefs re-exported from `apple-cf`.

## Installation

```toml
[dependencies]
servicemanagement-rs = "0.5"
```

Requires macOS 13 or later and Rust 1.82 or later.

## Quick start

```rust
use servicemanagement::MainApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_app = MainApp::new()?;
    println!("main app status: {}", main_app.status()?.as_str());
    Ok(())
}
```

## Highlights

- `SMAppService` plus typed wrappers for `MainApp`, `AgentService`,
  `DaemonService`, and `LoginItem`
- `SMAppServiceStatus` helpers, including `status_for_legacy_plist()`
- `Authorization` handles with right acquisition and external-form round trips
- legacy `SMLoginItem::set_enabled()`
- legacy `SMJobBless` helpers for copying, submitting, removing, and blessing
  launchd jobs
- legacy error domains and `SMErrorCode` constants

## Examples

```bash
cargo run --example 01_smoke
cargo run --example 02_authorization
cargo run --example 09_sm_job_bless_legacy
```

All numbered examples are headless-friendly and exit successfully on macOS even
when a given API requires additional signing, entitlement, or privileged-helper
setup.

## Errors

`ServiceManagementError` keeps the framework's error `domain` and `code` next to
the message. `sm_error_code()` maps codes from `SMAppServiceErrorDomain` and
`kSMErrorDomainFramework` to `SMErrorCode`, so callers can react to
`LaunchDeniedByUser`, `AlreadyRegistered`, `InvalidSignature` and the rest:

```rust,no_run
use servicemanagement::{SMAppService, SMErrorCode};

fn register_agent() -> servicemanagement::Result<()> {
    let agent = SMAppService::agent("com.example.agent.plist")?;
    match agent.register() {
        Err(error) if error.sm_error_code() == Some(SMErrorCode::AlreadyRegistered) => Ok(()),
        other => other,
    }
}
```

`SMAppService::unregister_with_completion_handler(timeout)` gives up after
`timeout`; `ServiceManagementError::is_timeout()` tells that case apart.

## Legacy privileged helpers

`SMJobBless`, `SMJobSubmit`, `SMJobRemove`, `SMJobCopyDictionary`,
`SMCopyAllJobDictionaries` and `SMLoginItemSetEnabled` are deprecated by Apple,
and their wrappers are `#[deprecated]`. Use `SMAppService` (`DaemonService`,
`AgentService`, `LoginItem`) instead.

A helper installed with `SMJobBless` or registered as a daemon runs as root and
accepts requests from any process that can reach its XPC service. It must check
every client before acting: identify the client by its audit token (never by
PID, which can be reused) and validate its code signature against a code
requirement. [security-rs](https://crates.io/crates/security-rs) provides this
with `Code::guest_with_audit_token`, `Code::check_validity` and
`Code::from_xpc_message`.

## Authorization

This crate keeps its own `Authorization` type: it links its own Swift bridge,
which cannot share handles with security-rs, and depending on security-rs would
pull that whole bridge into every user. The two interoperate through
`AuthorizationExternalForm`: `external_form()` returns the same 32 bytes as
security-rs `Authorization::external_form()`, and each crate's
`from_external_form` accepts the other's bytes.

## API notes

- `app_service_error_domain()` requires macOS 15+ at runtime because Apple only
  added `SMAppServiceErrorDomain` in the macOS 15 SDK/runtime.
- The safe legacy job helpers use XML property lists and JSON bridge payloads,
  while the `unsafe` `legacy::job_submit_raw`, `legacy::job_remove` and
  `legacy::job_bless` take raw CoreFoundation and `AuthorizationRef` values.

## Coverage audit

See [`COVERAGE.md`](COVERAGE.md) for the per-symbol audit against the SDK
headers.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
