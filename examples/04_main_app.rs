use servicemanagement::MainApp;

fn main() {
    match MainApp::new() {
        Ok(service) => println!("main app wrapper status: {}", service.status().as_str()),
        Err(error) => println!("main app wrapper creation failed: {error}"),
    }
}
