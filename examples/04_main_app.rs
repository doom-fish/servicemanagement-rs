use servicemanagement::MainApp;

fn main() {
    match MainApp::new() {
        Ok(service) => match service.status() {
            Ok(status) => println!("main app wrapper status: {}", status.as_str()),
            Err(error) => println!("main app wrapper status unavailable: {error}"),
        },
        Err(error) => println!("main app wrapper creation failed: {error}"),
    }
}
