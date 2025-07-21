use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use email_syntax_verify_opt::validate_email as our_validate_email;
use regex::Regex;

fn validator_crate_validate(email: &str) -> bool {
    use validator::Validate;

    #[derive(Validate)]
    struct EmailStruct {
        #[validate(email)]
        email: String,
    }

    let email_struct = EmailStruct {
        email: email.to_string(),
    };
    email_struct.validate().is_ok()
}

const VALID_EMAILS: &[&str] = &[
    "test@example.com",
    "user.name@domain.co.uk",
    "email@[127.0.0.1]",
    "complex.email+tag@subdomain.example.org",
    "simple@test.com",
    "user123@test-domain.co.uk",
    "firstname.lastname@company.com",
    "email@123.123.123.123",
    "user+tag@example.org",
    "test.email.with+symbol@example.com",
    "x@example.com",
    "example@s.example",
    "test@example-one.com",
    "test@example.name",
    "test@example.museum",
];

const INVALID_EMAILS: &[&str] = &[
    "invalid.email",
    "@invalid.com",
    "test@",
    "test..test@example.com",
    "test@.com",
    ".test@example.com",
    "test.@example.com",
    "test@example.",
    "test@example..com",
    "test@-example.com",
    "test@example-.com",
    "",
    "test",
    "test@",
    "@test.com",
    "test@@example.com",
];

const REALISTIC_EMAILS: &[&str] = &[
    "john.doe@company.com",
    "jane.smith@university.edu",
    "support@helpdesk.org",
    "noreply@notifications.service.com",
    "admin@system.internal",
    "user123@social-network.co.uk",
    "newsletter@marketing.agency",
    "contact@small-business.local",
    "info@government.gov",
    "sales@enterprise.corporation",
];

fn create_regex_validator() -> Regex {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
}

fn bench_comparison_valid_emails(c: &mut Criterion) {
    let regex = create_regex_validator();
    let mut group = c.benchmark_group("valid_emails_comparison");

    group.bench_function("our_validator", |b| {
        b.iter(|| {
            for email in VALID_EMAILS {
                black_box(our_validate_email(black_box(email)));
            }
        })
    });

    group.bench_function("validator_crate", |b| {
        b.iter(|| {
            for email in VALID_EMAILS {
                black_box(validator_crate_validate(black_box(email)));
            }
        })
    });

    group.bench_function("regex_validator", |b| {
        b.iter(|| {
            for email in VALID_EMAILS {
                black_box(regex.is_match(black_box(email)));
            }
        })
    });

    group.finish();
}

fn bench_comparison_invalid_emails(c: &mut Criterion) {
    let regex = create_regex_validator();
    let mut group = c.benchmark_group("invalid_emails_comparison");

    group.bench_function("our_validator", |b| {
        b.iter(|| {
            for email in INVALID_EMAILS {
                black_box(our_validate_email(black_box(email)));
            }
        })
    });

    group.bench_function("validator_crate", |b| {
        b.iter(|| {
            for email in INVALID_EMAILS {
                black_box(validator_crate_validate(black_box(email)));
            }
        })
    });

    group.bench_function("regex_validator", |b| {
        b.iter(|| {
            for email in INVALID_EMAILS {
                black_box(regex.is_match(black_box(email)));
            }
        })
    });

    group.finish();
}

fn bench_comparison_realistic_emails(c: &mut Criterion) {
    let regex = create_regex_validator();
    let mut group = c.benchmark_group("realistic_emails_comparison");

    group.bench_function("our_validator", |b| {
        b.iter(|| {
            for email in REALISTIC_EMAILS {
                black_box(our_validate_email(black_box(email)));
            }
        })
    });

    group.bench_function("validator_crate", |b| {
        b.iter(|| {
            for email in REALISTIC_EMAILS {
                black_box(validator_crate_validate(black_box(email)));
            }
        })
    });

    group.bench_function("regex_validator", |b| {
        b.iter(|| {
            for email in REALISTIC_EMAILS {
                black_box(regex.is_match(black_box(email)));
            }
        })
    });

    group.finish();
}

fn bench_single_email_comparison(c: &mut Criterion) {
    let regex = create_regex_validator();
    let test_email = "user.name@example.com";
    let mut group = c.benchmark_group("single_email_comparison");

    group.bench_function("our_validator", |b| {
        b.iter(|| black_box(our_validate_email(black_box(test_email))))
    });

    group.bench_function("validator_crate", |b| {
        b.iter(|| black_box(validator_crate_validate(black_box(test_email))))
    });

    group.bench_function("regex_validator", |b| {
        b.iter(|| black_box(regex.is_match(black_box(test_email))))
    });

    group.finish();
}

fn bench_email_length_impact(c: &mut Criterion) {
    let short_email = "a@b.co";
    let medium_email = "user.name@example.com";
    let long_email = "very.long.email.address.with.many.dots@subdomain.example-domain.co.uk";

    let mut group = c.benchmark_group("email_length_impact");

    for (name, email) in [
        ("short", short_email),
        ("medium", medium_email),
        ("long", long_email),
    ] {
        group.bench_with_input(
            BenchmarkId::new("our_validator", name),
            email,
            |b, email| b.iter(|| black_box(our_validate_email(black_box(email)))),
        );

        group.bench_with_input(
            BenchmarkId::new("validator_crate", name),
            email,
            |b, email| b.iter(|| black_box(validator_crate_validate(black_box(email)))),
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_comparison_valid_emails,
    bench_comparison_invalid_emails,
    bench_comparison_realistic_emails,
    bench_single_email_comparison,
    bench_email_length_impact
);
criterion_main!(benches);
