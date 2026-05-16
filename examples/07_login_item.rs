use servicemanagement::LoginItem;

fn main() {
    match LoginItem::new("com.example.login-item") {
        Ok(service) => println!("login item status: {}", service.status().as_str()),
        Err(error) => println!("login item creation failed: {error}"),
    }
}
