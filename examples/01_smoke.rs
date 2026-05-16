use servicemanagement::{app_service_error_domain, SMAppService};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = SMAppService::main_app()?;
    println!("main app status: {}", service.status().as_str());
    match app_service_error_domain() {
        Ok(domain) => println!("SMAppService error domain: {domain}"),
        Err(error) => println!("SMAppService error domain unavailable: {error}"),
    }
    println!("✅ servicemanagement smoke OK");
    Ok(())
}
