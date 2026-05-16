use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use servicemanagement::{Authorization, AuthorizationFlags, LaunchdDomain, SMJobBless};

fn unique_label() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after the unix epoch")
        .as_nanos();
    format!(
        "com.doomfish.servicemanagement.tests.{}.{}",
        std::process::id(),
        nanos
    )
}

fn job_plist_xml(label: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>{label}</string>
  <key>ProgramArguments</key>
  <array>
    <string>/usr/bin/true</string>
  </array>
  <key>RunAtLoad</key>
  <true/>
</dict>
</plist>
"#
    )
}

struct SubmittedJobGuard {
    label: String,
}

impl Drop for SubmittedJobGuard {
    fn drop(&mut self) {
        let _ = SMJobBless::job_remove(LaunchdDomain::User, &self.label, None, true);
    }
}

#[test]
fn copy_job_dictionary_returns_none_for_missing_label() {
    let missing = SMJobBless::copy_job_dictionary(
        LaunchdDomain::User,
        "com.example.servicemanagement.tests.missing",
    )
    .expect("missing lookups should not error");
    assert!(missing.is_none());
}

#[test]
fn user_launchd_job_submit_and_remove_round_trips() {
    let label = unique_label();
    let _guard = SubmittedJobGuard {
        label: label.clone(),
    };

    SMJobBless::job_submit_plist(LaunchdDomain::User, &job_plist_xml(&label), None)
        .expect("user-domain job submission should succeed");

    let mut seen = None;
    for _ in 0..10 {
        seen = SMJobBless::copy_job_dictionary(LaunchdDomain::User, &label)
            .expect("submitted job lookup should not error");
        if seen.is_some() {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }

    let job = seen.expect("submitted job should become visible");
    assert_eq!(job.label.as_deref(), Some(label.as_str()));
    assert!(job.plist_xml.contains(&label));

    SMJobBless::job_remove(LaunchdDomain::User, &label, None, true)
        .expect("submitted job should be removable");
}

#[test]
#[ignore = "requires a signed privileged helper and user authorization"]
fn privileged_bless_requires_real_helper_assets() {
    let authorization = Authorization::new(AuthorizationFlags::DEFAULTS)
        .expect("empty authorization should be creatable");
    let result = SMJobBless::bless(
        LaunchdDomain::System,
        "com.example.missing-privileged-helper",
        Some(&authorization),
    );
    assert!(result.is_err());
}
