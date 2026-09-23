use servicemanagement::authorization::EXTERNAL_FORM_LEN;
use servicemanagement::{Authorization, AuthorizationFlags};

#[test]
fn authorization_external_form_round_trips() {
    let authorization = Authorization::new(AuthorizationFlags::DEFAULTS)
        .expect("empty authorization should be creatable");
    let external_form = authorization
        .external_form()
        .expect("external form should be available");
    assert_eq!(external_form.len(), EXTERNAL_FORM_LEN);

    let imported =
        Authorization::from_external_form(&external_form).expect("external form should round-trip");
    assert_eq!(
        imported
            .external_form()
            .expect("imported external form")
            .len(),
        EXTERNAL_FORM_LEN
    );
    imported
        .destroy_rights()
        .expect("destroying rights should succeed for an empty authorization");
}

#[test]
fn external_forms_of_the_wrong_length_are_rejected() {
    assert!(Authorization::from_external_form(&[0_u8; EXTERNAL_FORM_LEN - 1]).is_err());
    assert!(Authorization::from_external_form(&[]).is_err());
}
