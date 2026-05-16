use servicemanagement::AppService;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = AppService::main_app()?;
    println!("main app status: {:?}", service.status());
    println!("✅ servicemanagement smoke OK");
    Ok(())
}
