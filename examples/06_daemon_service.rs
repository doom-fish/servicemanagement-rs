use servicemanagement::DaemonService;

fn main() {
    match DaemonService::new("com.example.daemon.plist") {
        Ok(service) => println!("daemon service status: {}", service.status().as_str()),
        Err(error) => println!("daemon service creation failed: {error}"),
    }
}
