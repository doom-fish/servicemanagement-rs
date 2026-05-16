# Changelog

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
