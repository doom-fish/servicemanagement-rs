# servicemanagement-rs

Safe Rust bindings for Apple’s `ServiceManagement.framework` on macOS.

`servicemanagement-rs` covers the modern helper-service workflow first, while
also exposing the legacy launchd APIs that are still encountered in existing
macOS codebases:

- `AppService` for `SMAppService.mainApp`, login items, agents, and daemons
- `register`, `unregister`, `status`, and `open_system_settings_login_items`
- legacy `SMJobBless`, `SMJobSubmit`, `SMJobRemove`, and
  `SMCopyAllJobDictionaries`

## Status

Initial `0.1.0` coverage focuses on the modern `SMAppService` surface (via a
Swift bridge) plus raw / completeness wrappers for the deprecated C APIs.

## Installation

```toml
[dependencies]
servicemanagement-rs = "0.1"
```

## Quick start

```rust
use servicemanagement::AppService;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = AppService::main_app()?;
    println!("main app status: {:?}", service.status());
    Ok(())
}
```

## Highlights

- Swift-backed `AppService` wrappers for `mainApp`, `loginItem`, `agent`, and
  `daemon`
- Safe `register`, `unregister`, and `status` helpers
- `open_system_settings_login_items()` convenience wrapper
- `legacy` module exposing `SMCopyAllJobDictionaries`, `SMJobBless`,
  `SMJobSubmit`, and `SMJobRemove`

## API notes

- The `legacy` job-control helpers are intentionally low-level and retain the
  underlying `AuthorizationRef` / `CFDictionaryRef` requirements.
- `SMJobBless` and the `SMJob*` launchd APIs are deprecated by Apple but kept
  here for completeness.
- `AppService` requires macOS 13+ at runtime; older systems return descriptive
  bridge errors.

## Smoke example

```bash
cargo run --example 01_smoke
```

Expected tail output:

```text
✅ servicemanagement smoke OK
```

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
