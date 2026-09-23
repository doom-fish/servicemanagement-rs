use servicemanagement::AgentService;

fn main() {
    match AgentService::new("com.example.agent.plist") {
        Ok(service) => match service.status() {
            Ok(status) => println!("agent service status: {}", status.as_str()),
            Err(error) => println!("agent service status unavailable: {error}"),
        },
        Err(error) => println!("agent service creation failed: {error}"),
    }
}
