use servicemanagement::{LaunchdDomain, SMJobBless};

fn main() {
    match SMJobBless::copy_all_job_dictionaries(LaunchdDomain::User) {
        Ok(jobs) => println!("user launchd jobs discovered: {}", jobs.len()),
        Err(error) => println!("legacy launchd query reported: {error}"),
    }
}
