# servicemanagement-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 40
VERIFIED: 31
GAPS: 0
EXEMPT: 9
COVERAGE_PCT: 100.00%

Assumptions used for this audit:

- Counts include exported macros/constants, enum types and cases, and callable `SMAppService` class/instance members.
- The `SMAppService` interface declaration is not counted separately because the Rust `SMAppService` wrapper exists to expose the member surface listed below.
- Deprecated SDK items stay visible in the audit, but they are tracked as **EXEMPT** per the audit instructions even when the crate still wraps them.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `kSMRightBlessPrivilegedHelper` | macro constant | `ServiceManagement.h` | `authorization::SM_RIGHT_BLESS_PRIVILEGED_HELPER` |
| `kSMRightModifySystemDaemons` | macro constant | `ServiceManagement.h` | `authorization::SM_RIGHT_MODIFY_SYSTEM_DAEMONS` |
| `kSMDomainSystemLaunchd` | constant | `ServiceManagement.h` | `sm_job_bless::LaunchdDomain::System` |
| `kSMDomainUserLaunchd` | constant | `ServiceManagement.h` | `sm_job_bless::LaunchdDomain::User` |
| `kSMErrorInternalFailure` | enum case | `SMErrors.h` | `SMErrorCode::InternalFailure` |
| `kSMErrorInvalidSignature` | enum case | `SMErrors.h` | `SMErrorCode::InvalidSignature` |
| `kSMErrorAuthorizationFailure` | enum case | `SMErrors.h` | `SMErrorCode::AuthorizationFailure` |
| `kSMErrorToolNotValid` | enum case | `SMErrors.h` | `SMErrorCode::ToolNotValid` |
| `kSMErrorJobNotFound` | enum case | `SMErrors.h` | `SMErrorCode::JobNotFound` |
| `kSMErrorServiceUnavailable` | enum case | `SMErrors.h` | `SMErrorCode::ServiceUnavailable` |
| `kSMErrorJobPlistNotFound` | enum case | `SMErrors.h` | `SMErrorCode::JobPlistNotFound` |
| `kSMErrorJobMustBeEnabled` | enum case | `SMErrors.h` | `SMErrorCode::JobMustBeEnabled` |
| `kSMErrorInvalidPlist` | enum case | `SMErrors.h` | `SMErrorCode::InvalidPlist` |
| `kSMErrorLaunchDeniedByUser` | enum case | `SMErrors.h` | `SMErrorCode::LaunchDeniedByUser` |
| `kSMErrorAlreadyRegistered` | enum case | `SMErrors.h` | `SMErrorCode::AlreadyRegistered` |
| `SMAppServiceStatus` | enum type | `SMAppService.h` | `SMAppServiceStatus` |
| `SMAppServiceStatusNotRegistered` | enum case | `SMAppService.h` | `SMAppServiceStatus::NotRegistered` |
| `SMAppServiceStatusEnabled` | enum case | `SMAppService.h` | `SMAppServiceStatus::Enabled` |
| `SMAppServiceStatusRequiresApproval` | enum case | `SMAppService.h` | `SMAppServiceStatus::RequiresApproval` |
| `SMAppServiceStatusNotFound` | enum case | `SMAppService.h` | `SMAppServiceStatus::NotFound` |
| `SMAppServiceErrorDomain` | constant | `SMAppService.h` | `app_service_error_domain()` |
| `SMAppService.mainApp` | class property | `SMAppService.h` | `SMAppService::main_app()`, `MainApp::new()` |
| `SMAppService.loginItem(identifier:)` | class method | `SMAppService.h` | `SMAppService::login_item()`, `LoginItem::new()` |
| `SMAppService.agent(plistName:)` | class method | `SMAppService.h` | `SMAppService::agent()`, `AgentService::new()` |
| `SMAppService.daemon(plistName:)` | class method | `SMAppService.h` | `SMAppService::daemon()`, `DaemonService::new()` |
| `SMAppService.register()` | instance method | `SMAppService.h` | `SMAppService::register()` |
| `SMAppService.unregister()` | instance method | `SMAppService.h` | `SMAppService::unregister()` |
| `SMAppService.unregisterWithCompletionHandler(_:)` | instance method | `SMAppService.h` | `SMAppService::unregister_with_completion_handler()` |
| `SMAppService.status` | instance property | `SMAppService.h` | `SMAppService::status()` |
| `SMAppService.statusForLegacyPlist(at:)` | class method | `SMAppService.h` | `status_for_legacy_plist()` |
| `SMAppService.openSystemSettingsLoginItems()` | class method | `SMAppService.h` | `open_system_settings_login_items()` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

No gaps found.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `SMJobCopyDictionary` | function | `ServiceManagement.h` | Deprecated in the SDK; excluded from the denominator even though it is wrapped by `SMJobBless::copy_job_dictionary()` and `legacy::job_copy_dictionary()`. | `__OSX_AVAILABLE_BUT_DEPRECATED(__MAC_10_6, __MAC_10_10, __IPHONE_3_0, __IPHONE_8_0)` |
| `SMCopyAllJobDictionaries` | function | `ServiceManagement.h` | Deprecated in the SDK; excluded from the denominator even though it is wrapped by `SMJobBless::copy_all_job_dictionaries()` and `legacy::copy_all_job_dictionaries()`. | `__OSX_AVAILABLE_BUT_DEPRECATED(__MAC_10_6, __MAC_10_10, __IPHONE_3_0, __IPHONE_8_0)` |
| `SMJobSubmit` | function | `ServiceManagement.h` | Deprecated in the SDK; excluded from the denominator even though it is wrapped by `SMJobBless::job_submit_plist()` and `legacy::job_submit_raw()`. | `__OSX_AVAILABLE_BUT_DEPRECATED(__MAC_10_6, __MAC_10_10, __IPHONE_3_0, __IPHONE_8_0)` |
| `SMJobRemove` | function | `ServiceManagement.h` | Deprecated in the SDK; excluded from the denominator even though it is wrapped by `SMJobBless::job_remove()` and `legacy::job_remove()`. | `__OSX_AVAILABLE_BUT_DEPRECATED(__MAC_10_6, __MAC_10_10, __IPHONE_3_0, __IPHONE_8_0)` |
| `SMJobBless` | function | `ServiceManagement.h` | Deprecated in the SDK; excluded from the denominator even though it is wrapped by `SMJobBless::bless()` and `legacy::job_bless()`. | `__OSX_DEPRECATED(10.6, 13.0, "Please use SMAppService instead")` |
| `SMLoginItemSetEnabled` | function | `SMLoginItem.h` | Deprecated in the SDK; excluded from the denominator even though it is wrapped by `SMLoginItem::set_enabled()`. | `__OSX_DEPRECATED(10.6, 13.0, "Please use SMAppService instead")` |
| `kSMErrorDomainIPC` | constant | `SMErrors.h` | Deprecated in the SDK; excluded from the denominator even though it is surfaced by `legacy_error_domain_ipc()`. | `__OSX_AVAILABLE_BUT_DEPRECATED(__MAC_10_6, __MAC_10_10, __IPHONE_3_0, __IPHONE_8_0)` |
| `kSMErrorDomainFramework` | constant | `SMErrors.h` | Deprecated in the SDK; excluded from the denominator even though it is surfaced by `legacy_error_domain_framework()`. | `__OSX_AVAILABLE_BUT_DEPRECATED(__MAC_10_6, __MAC_10_10, __IPHONE_3_0, __IPHONE_8_0)` |
| `kSMErrorDomainLaunchd` | constant | `SMErrors.h` | Deprecated in the SDK; excluded from the denominator even though it is surfaced by `legacy_error_domain_launchd()`. | `__OSX_AVAILABLE_BUT_DEPRECATED(__MAC_10_6, __MAC_10_10, __IPHONE_3_0, __IPHONE_8_0)` |
