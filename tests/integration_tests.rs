use email_syntax_verify_opt::{EmailValidator, ValidateEmail};
use std::borrow::Cow;

const TEST_CASES: &[(&str, bool)] = &[
    ("email@here.com", true),
    ("weirder-email@here.and.there.com", true),
    ("!def!xyz%abc@example.com", true),
    ("email@[127.0.0.1]", true),
    ("email@[2001:dB8::1]", true),
    ("email@[2001:dB8:0:0:0:0:0:1]", true),
    ("email@[::fffF:127.0.0.1]", true),
    ("example@valid-----hyphens.com", true),
    ("example@valid-with-hyphens.com", true),
    ("\"test@test\"@example.com", false),
    (
        "a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        true,
    ),
    (
        "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.atm",
        true,
    ),
    (
        "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbb.atm",
        true,
    ),
    (
        "a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        false,
    ),
    ("", false),
    ("abc", false),
    ("abc@", false),
    ("abc@bar", true),
    ("a @x.cz", false),
    ("abc@.com", false),
    ("something@@somewhere.com", false),
    ("email@127.0.0.1", true),
    ("email@[127.0.0.256]", false),
    ("email@[2001:db8::12345]", false),
    ("email@[2001:db8:0:0:0:0:1]", false),
    ("email@[::ffff:127.0.0.256]", false),
    ("example@invalid-.com", false),
    ("example@-invalid.com", false),
    ("example@invalid.com-", false),
    ("example@inv-.alid-.com", false),
    ("example@inv-.-alid.com", false),
    ("test@example.com\\n\\n<script src=\"x.js\">", false),
    ("\"\\\\\\011\"@here.com", false),
    ("\"\\\\\\012\"@here.com", false),
    ("trailingdot@shouldfail.com.", false),
    ("a@b.com\n", false),
    ("a\n@b.com", false),
    ("\"test@test\"\\n@example.com", false),
    ("a@[127.0.0.1]\n", false),
    ("John.Doe@exam_ple.com", false),
];

#[test]
fn test_validate_email() {
    for &(input, expected) in TEST_CASES {
        assert_eq!(
            input.validate_email(),
            expected,
            "Email `{}` validation failed",
            input
        );
    }
}

#[test]
fn test_validate_email_cow() {
    let cases = [("email@here.com", true), ("a@[127.0.0.1]\n", false)];

    for &(email, expected) in &cases {
        let test_borrowed: Cow<'static, str> = email.into();
        assert_eq!(test_borrowed.validate_email(), expected);

        let test_owned: Cow<'static, str> = String::from(email).into();
        assert_eq!(test_owned.validate_email(), expected);
    }
}

#[test]
fn test_validate_email_rfc5321() {
    let long_local = "a".repeat(65) + "@mail.com";
    assert!(!long_local.validate_email());

    let long_domain = "a@".to_string() + &"a".repeat(252) + ".com";
    assert!(!long_domain.validate_email());
}

#[test]
fn test_edge_cases() {
    assert!(!EmailValidator::validate(b"@"));
    assert!(!EmailValidator::validate(b"test@"));
    assert!(!EmailValidator::validate(b"@test"));
    assert!(!EmailValidator::validate(b""));
}

#[test]
fn test_option_types() {
    let some_email: Option<&str> = Some("test@example.com");
    assert!(some_email.validate_email());

    let none_email: Option<&str> = None;
    assert!(none_email.validate_email());
}
