use servicemanagement::async_api::{status_for_legacy_plist_async, SMAppServiceAsyncExt};
use servicemanagement::SMAppService;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        let service =
            SMAppService::agent("com.example.servicemanagement.examples.missing.agent.plist")?;
        match service.unregister_async().await {
            Ok(()) => println!("service unregistered"),
            Err(error) => println!("async unregister reported: {error}"),
        }

        let status = status_for_legacy_plist_async(
            "/Library/LaunchDaemons/com.example.servicemanagement.examples.missing.plist",
        )
        .await?;
        println!("async legacy plist status: {}", status.as_str());
        Ok::<(), Box<dyn std::error::Error>>(())
    })
}
