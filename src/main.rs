use email_syntax_verify_opt::ValidateEmail;

fn main() {
    let test_emails = [
        "test@example.com",
        "invalid.email",
        "user@domain.co.uk",
        "email@[127.0.0.1]",
    ];

    for email in test_emails {
        println!("{}: {}", email, email.validate_email());
    }
}
