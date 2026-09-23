#![allow(deprecated)]

use servicemanagement::legacy::{self, LaunchdDomain};
use servicemanagement::{
    legacy_error_domain_framework, legacy_error_domain_ipc, legacy_error_domain_launchd,
};

#[test]
fn legacy_error_domains_are_readable() {
    assert_eq!(
        legacy_error_domain_framework().expect("framework domain"),
        "kSMErrorDomainFramework"
    );
    assert_eq!(
        legacy_error_domain_ipc().expect("IPC domain"),
        "kSMErrorDomainIPC"
    );
    assert_eq!(
        legacy_error_domain_launchd().expect("launchd domain"),
        "kSMErrorDomainLaunchd"
    );
}

#[test]
fn legacy_job_lookups_use_one_structured_listing() {
    let missing = legacy::job_copy_dictionary(
        LaunchdDomain::User,
        "com.example.servicemanagement.tests.missing",
    )
    .expect("missing lookups should not error");
    assert!(missing.is_none());

    let jobs = legacy::copy_all_job_dictionaries(LaunchdDomain::User)
        .expect("user launchd jobs should be listable");
    assert!(jobs.iter().all(|job| !job.plist_xml.is_empty()));
}

#[test]
fn legacy_labels_with_nul_bytes_are_rejected() {
    assert!(legacy::job_copy_dictionary(LaunchdDomain::User, "bad\0label").is_err());
}
