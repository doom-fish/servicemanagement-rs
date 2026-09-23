use servicemanagement::DaemonService;

fn main() {
    match DaemonService::new("com.example.daemon.plist") {
        Ok(service) => match service.status() {
            Ok(status) => println!("daemon service status: {}", status.as_str()),
            Err(error) => println!("daemon service status unavailable: {error}"),
        },
        Err(error) => println!("daemon service creation failed: {error}"),
    }
}
