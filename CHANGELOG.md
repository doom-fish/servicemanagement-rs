# Changelog

All notable changes to `servicemanagement-rs` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - Unreleased

### Security

- `SMJobBless` and the other Apple-deprecated legacy functions (`SMJobSubmit`, `SMJobRemove`,
  `SMJobCopyDictionary`, `SMCopyAllJobDictionaries`, `SMLoginItemSetEnabled`) are now
  `#[deprecated]`, and the README documents that a root helper must validate its XPC clients by
  audit token and code requirement (for example with security-rs `Code::guest_with_audit_token`).

### Fixed

- Errors kept only the localized message, so `SMErrorCode` was never produced. They now carry the
  NSError/CFError (or OSStatus) domain and code, and `ServiceManagementError::sm_error_code()` maps
  `SMAppServiceErrorDomain` and `kSMErrorDomainFramework` codes to `SMErrorCode`.
- `SMAppService::status()` reported a bridge failure as `SMAppServiceStatus::Unknown(-1)`; it now
  returns an error.
- `unregister_with_completion_handler` blocked forever if the completion never arrived.
- The bridge type-checks handles and clamps status values instead of trapping.

### Changed

- **Breaking:** `SMAppService::status()` returns `Result<SMAppServiceStatus>` (also through the
  `MainApp`, `AgentService`, `DaemonService` and `LoginItem` wrappers).
- **Breaking:** `unregister_with_completion_handler` takes a `timeout: Duration` and fails with an
  `ETIMEDOUT` error (`ServiceManagementError::is_timeout`) when it expires.
- **Breaking:** `Authorization::external_form()` returns the 32 `AuthorizationExternalForm` bytes
  (was a base64 `String`) and `from_external_form` takes `&[u8]`, matching security-rs so forms can
  be exchanged between the two crates.
- **Breaking:** `ServiceManagementError` has `domain` and `code` fields and is `#[non_exhaustive]`.
- **Breaking:** `legacy::copy_all_job_dictionaries` returns `Vec<LegacyJobDictionary>`; it is now
  the same function as `SMJobBless::copy_all_job_dictionaries`.
- The legacy helpers use apple-cf's `CFString`/`CFType`/`CFError` instead of a local copy.
- `rust-version` is 1.82; depends on apple-cf 0.11 and doom-fish-utils 0.4.1.
- The Swift bridge target no longer ships an empty C header or `publicHeadersPath`.

### Added

- `ServiceManagementError::{domain, code, sm_error_code, is_timeout}`.
- `authorization::EXTERNAL_FORM_LEN`.

### Removed

- **Breaking:** `legacy::copy_all_job_dictionaries_structured` (use
  `legacy::copy_all_job_dictionaries`) and the description-string variant of
  `legacy::copy_all_job_dictionaries`.

## [0.4.0] - 2026-05-20

### Added

- `async_api` module behind the `async` feature, providing executor-agnostic async wrappers for `SMAppService::register`, `SMAppService::unregister`, and `status_for_legacy_plist`. Uses `doom-fish-utils::completion`.

## [0.3.3] - 2026-05-18

### Changed

- Added concise rustdoc comments across the public ServiceManagement APIs outside `ffi`.

## [0.3.2] - 2026-05-18

### Changed

- Re-exported `Boolean` from `apple_cf::raw` in `ffi`, removing the remaining crate-local primitive alias.

## [0.3.1] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.3.0] - 2026-05-18

### Changed

- Added a direct `apple-cf` dependency for shared Core Foundation raw typedefs.
- Re-exported `CFArrayRef`, `CFDictionaryRef`, `CFErrorRef`, and `CFStringRef`
  from `apple-cf` and updated the legacy Core Foundation call sites to use the
  shared definitions.

### Breaking

- `servicemanagement::ffi` now exposes those four Core Foundation typedefs from
  `apple-cf`, including the SDK-backed `CFErrorRef` pointer type.

## [0.2.1] - 2025-05-17

### Fixed

- Added comprehensive SAFETY comments to all unsafe blocks, documenting the invariants and
  preconditions that make each unsafe operation sound. Includes clarification on pointer
  validity, lifetime management, and FFI boundary correctness.

## [0.2.0] - 2026-05-16

### Added

- Full `ServiceManagement.framework` SDK coverage, including split Swift bridge
  files and safe Rust modules for `SMAppService`, `SMAppServiceStatus`,
  `MainApp`, `AgentService`, `DaemonService`, `LoginItem`, `SMLoginItem`,
  `SMJobBless`, and `Authorization`.
- Typed Rust wrappers for the modern `SMAppService` constructors alongside the
  original `AppService` compatibility alias.
- Safe legacy helpers for `SMLoginItemSetEnabled`, `SMJobCopyDictionary`,
  `SMCopyAllJobDictionaries`, `SMJobSubmit`, `SMJobRemove`, and `SMJobBless`,
  plus `SMErrorCode` and legacy error-domain accessors.
- Nine numbered examples, nine integration-test files, and a crate-local
  `COVERAGE.md` audit.

## [0.1.0] - 2026-05-16

### Added

- Swift bridge for modern `SMAppService` creation, status inspection,
  registration, unregistration, and Login Items settings navigation.
- Safe `AppService` wrapper covering `mainApp`, `loginItem`, `agent`, and
  `daemon` helpers.
- Legacy `SMCopyAllJobDictionaries`, `SMJobBless`, `SMJobSubmit`, and
  `SMJobRemove` wrappers under the `legacy` module.
- `examples/01_smoke.rs` smoke example that prints `SMAppService.mainApp`
  status and exits cleanly.
