# ServiceManagement.framework coverage audit

Audited against:

- `ServiceManagement.framework/Headers/ServiceManagement.h`
- `ServiceManagement.framework/Headers/SMAppService.h`
- `ServiceManagement.framework/Headers/SMLoginItem.h`
- `ServiceManagement.framework/Headers/SMErrors.h`

| SDK symbol | Status | Rust surface |
| --- | --- | --- |
| `kSMRightBlessPrivilegedHelper` | ✅ implemented | `authorization::SM_RIGHT_BLESS_PRIVILEGED_HELPER` |
| `kSMRightModifySystemDaemons` | ✅ implemented | `authorization::SM_RIGHT_MODIFY_SYSTEM_DAEMONS` |
| `kSMDomainSystemLaunchd` | ✅ implemented | `LaunchdDomain::System` |
| `kSMDomainUserLaunchd` | ✅ implemented | `LaunchdDomain::User` |
| `SMJobCopyDictionary` | ✅ implemented | `SMJobBless::copy_job_dictionary`, `legacy::job_copy_dictionary` |
| `SMCopyAllJobDictionaries` | ✅ implemented | `SMJobBless::copy_all_job_dictionaries`, `legacy::copy_all_job_dictionaries` |
| `SMJobSubmit` | ✅ implemented | `SMJobBless::job_submit_plist`, `legacy::job_submit_raw` |
| `SMJobRemove` | ✅ implemented | `SMJobBless::job_remove`, `legacy::job_remove` |
| `SMJobBless` | ✅ implemented | `SMJobBless::bless`, `legacy::job_bless` |
| `SMLoginItemSetEnabled` | ✅ implemented | `SMLoginItem::set_enabled` |
| `kSMErrorDomainIPC` | ✅ implemented | `legacy_error_domain_ipc()` |
| `kSMErrorDomainFramework` | ✅ implemented | `legacy_error_domain_framework()` |
| `kSMErrorDomainLaunchd` | ✅ implemented | `legacy_error_domain_launchd()` |
| `kSMErrorInternalFailure` | ✅ implemented | `SMErrorCode::InternalFailure` |
| `kSMErrorInvalidSignature` | ✅ implemented | `SMErrorCode::InvalidSignature` |
| `kSMErrorAuthorizationFailure` | ✅ implemented | `SMErrorCode::AuthorizationFailure` |
| `kSMErrorToolNotValid` | ✅ implemented | `SMErrorCode::ToolNotValid` |
| `kSMErrorJobNotFound` | ✅ implemented | `SMErrorCode::JobNotFound` |
| `kSMErrorServiceUnavailable` | ✅ implemented | `SMErrorCode::ServiceUnavailable` |
| `kSMErrorJobPlistNotFound` | ✅ implemented | `SMErrorCode::JobPlistNotFound` |
| `kSMErrorJobMustBeEnabled` | ✅ implemented | `SMErrorCode::JobMustBeEnabled` |
| `kSMErrorInvalidPlist` | ✅ implemented | `SMErrorCode::InvalidPlist` |
| `kSMErrorLaunchDeniedByUser` | ✅ implemented | `SMErrorCode::LaunchDeniedByUser` |
| `kSMErrorAlreadyRegistered` | ✅ implemented | `SMErrorCode::AlreadyRegistered` |
| `SMAppService.Status` | ✅ implemented | `SMAppServiceStatus` |
| `SMAppServiceErrorDomain` | ✅ implemented | `app_service_error_domain()` |
| `SMAppService.mainApp` | ✅ implemented | `SMAppService::main_app()`, `MainApp::new()` |
| `SMAppService.loginItem(identifier:)` | ✅ implemented | `SMAppService::login_item()`, `LoginItem::new()` |
| `SMAppService.agent(plistName:)` | ✅ implemented | `SMAppService::agent()`, `AgentService::new()` |
| `SMAppService.daemon(plistName:)` | ✅ implemented | `SMAppService::daemon()`, `DaemonService::new()` |
| `SMAppService.register()` | ✅ implemented | `SMAppService::register()` |
| `SMAppService.unregister()` | ✅ implemented | `SMAppService::unregister()` |
| `SMAppService.unregisterWithCompletionHandler(_:)` | ✅ implemented | `SMAppService::unregister_with_completion_handler()` |
| `SMAppService.status` | ✅ implemented | `SMAppService::status()` |
| `SMAppService.statusForLegacyPlist(at:)` | ✅ implemented | `status_for_legacy_plist()` |
| `SMAppService.openSystemSettingsLoginItems()` | ✅ implemented | `open_system_settings_login_items()` |

Deferred/skipped: none.
