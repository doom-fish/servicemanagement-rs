use servicemanagement::AgentService;

fn main() {
    match AgentService::new("com.example.agent.plist") {
        Ok(service) => println!("agent service status: {}", service.status().as_str()),
        Err(error) => println!("agent service creation failed: {error}"),
    }
}
