# servicemanagement-rs

Safe Rust bindings for Apple’s `ServiceManagement.framework` on macOS.

`servicemanagement-rs` 0.3 covers the full public framework surface exposed by
Apple’s SDK, split into focused Swift bridge files and Rust modules for each
logical area:

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
servicemanagement-rs = "0.3"
```

## Quick start

```rust
use servicemanagement::MainApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_app = MainApp::new()?;
    println!("main app status: {}", main_app.status().as_str());
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

## API notes

- `SMAppService` requires macOS 13+ at runtime.
- `app_service_error_domain()` requires macOS 15+ at runtime because Apple only
  added `SMAppServiceErrorDomain` in the macOS 15 SDK/runtime.
- The safe legacy job helpers use XML property lists and JSON bridge payloads,
  while the original raw `legacy::*_raw` functions remain available for direct
  CoreFoundation interop.
- `SMJobBless`, `SMJobSubmit`, `SMJobRemove`, and `SMLoginItemSetEnabled` are
  deprecated by Apple but retained here for full framework coverage.

## Coverage audit

See [`COVERAGE.md`](COVERAGE.md) for the per-symbol audit against the SDK
headers.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
