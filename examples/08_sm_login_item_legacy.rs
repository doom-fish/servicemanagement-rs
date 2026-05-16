use servicemanagement::SMLoginItem;

fn main() {
    match SMLoginItem::set_enabled("com.example.legacy-login-item", false) {
        Ok(()) => println!("legacy login item toggle succeeded"),
        Err(error) => println!("legacy login item returned expected error: {error}"),
    }
}
