use servicemanagement::LoginItem;

fn main() {
    match LoginItem::new("com.example.login-item") {
        Ok(service) => match service.status() {
            Ok(status) => println!("login item status: {}", status.as_str()),
            Err(error) => println!("login item status unavailable: {error}"),
        },
        Err(error) => println!("login item creation failed: {error}"),
    }
}
