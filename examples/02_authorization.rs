use servicemanagement::{Authorization, AuthorizationFlags};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let authorization = Authorization::new(AuthorizationFlags::DEFAULTS)?;
    let external_form = authorization.external_form()?;
    let imported = Authorization::from_external_form(&external_form)?;
    imported.destroy_rights()?;
    println!(
        "authorization external form length: {}",
        external_form.len()
    );
    Ok(())
}
