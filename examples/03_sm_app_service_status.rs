use servicemanagement::status_for_legacy_plist;

fn main() {
    let path = "/Library/LaunchDaemons/com.example.servicemanagement.missing.plist";
    match status_for_legacy_plist(path) {
        Ok(status) => println!("legacy plist status for {path}: {}", status.as_str()),
        Err(error) => println!("legacy plist status lookup reported: {error}"),
    }
}
