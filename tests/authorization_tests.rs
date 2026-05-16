use servicemanagement::{Authorization, AuthorizationFlags};

#[test]
fn authorization_external_form_round_trips() {
    let authorization = Authorization::new(AuthorizationFlags::DEFAULTS)
        .expect("empty authorization should be creatable");
    let external_form = authorization
        .external_form()
        .expect("external form should be available");
    assert!(!external_form.is_empty());

    let imported =
        Authorization::from_external_form(&external_form).expect("external form should round-trip");
    imported
        .destroy_rights()
        .expect("destroying rights should succeed for an empty authorization");
}
