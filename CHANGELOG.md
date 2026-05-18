# Changelog

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
