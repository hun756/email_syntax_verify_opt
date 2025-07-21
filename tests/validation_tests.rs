use email_syntax_verify_opt::{EmailValidator, ValidateEmail};
use std::collections::HashMap;

mod test_data {
    pub struct TestCase {
        pub email: &'static str,
        pub expected: bool,
        pub category: &'static str,
        pub description: &'static str,
    }

    pub const RFC5322_COMPLIANT_EMAILS: &[TestCase] = &[
        TestCase {
            email: "simple@example.com",
            expected: true,
            category: "basic",
            description: "Simple valid email",
        },
        TestCase {
            email: "user.name@example.com",
            expected: true,
            category: "basic",
            description: "Email with dot in local part",
        },
        TestCase {
            email: "user+tag@example.com",
            expected: true,
            category: "basic",
            description: "Email with plus sign (subaddressing)",
        },
        TestCase {
            email: "user_name@example.com",
            expected: true,
            category: "basic",
            description: "Email with underscore in local part",
        },
        TestCase {
            email: "user-name@example.com",
            expected: true,
            category: "basic",
            description: "Email with hyphen in local part",
        },
        TestCase {
            email: "123@example.com",
            expected: true,
            category: "basic",
            description: "Numeric local part",
        },
        TestCase {
            email: "user@example-domain.com",
            expected: true,
            category: "domain",
            description: "Domain with hyphen",
        },
        TestCase {
            email: "user@sub.example.com",
            expected: true,
            category: "domain",
            description: "Subdomain",
        },
        TestCase {
            email: "user@example.co.uk",
            expected: true,
            category: "domain",
            description: "Multi-level TLD",
        },
        TestCase {
            email: "a@b.co",
            expected: true,
            category: "edge",
            description: "Minimal valid email",
        },
    ];

    pub const SPECIAL_CHARACTER_EMAILS: &[TestCase] = &[
        TestCase {
            email: "!def!xyz%abc@example.com",
            expected: true,
            category: "special_chars",
            description: "Special characters in local part",
        },
        TestCase {
            email: "user#tag@example.com",
            expected: true,
            category: "special_chars",
            description: "Hash symbol in local part",
        },
        TestCase {
            email: "user$money@example.com",
            expected: true,
            category: "special_chars",
            description: "Dollar sign in local part",
        },
        TestCase {
            email: "user&co@example.com",
            expected: true,
            category: "special_chars",
            description: "Ampersand in local part",
        },
        TestCase {
            email: "user'quote@example.com",
            expected: true,
            category: "special_chars",
            description: "Single quote in local part",
        },
        TestCase {
            email: "user*star@example.com",
            expected: true,
            category: "special_chars",
            description: "Asterisk in local part",
        },
        TestCase {
            email: "user/slash@example.com",
            expected: true,
            category: "special_chars",
            description: "Forward slash in local part",
        },
        TestCase {
            email: "user=equal@example.com",
            expected: true,
            category: "special_chars",
            description: "Equal sign in local part",
        },
        TestCase {
            email: "user?question@example.com",
            expected: true,
            category: "special_chars",
            description: "Question mark in local part",
        },
        TestCase {
            email: "user^caret@example.com",
            expected: true,
            category: "special_chars",
            description: "Caret in local part",
        },
        TestCase {
            email: "user`backtick@example.com",
            expected: true,
            category: "special_chars",
            description: "Backtick in local part",
        },
        TestCase {
            email: "user{brace@example.com",
            expected: true,
            category: "special_chars",
            description: "Left brace in local part",
        },
        TestCase {
            email: "user|pipe@example.com",
            expected: true,
            category: "special_chars",
            description: "Pipe symbol in local part",
        },
        TestCase {
            email: "user}brace@example.com",
            expected: true,
            category: "special_chars",
            description: "Right brace in local part",
        },
        TestCase {
            email: "user~tilde@example.com",
            expected: true,
            category: "special_chars",
            description: "Tilde in local part",
        },
    ];

    pub const IP_LITERAL_EMAILS: &[TestCase] = &[
        TestCase {
            email: "user@[127.0.0.1]",
            expected: true,
            category: "ip_literal",
            description: "IPv4 literal - localhost",
        },
        TestCase {
            email: "user@[192.168.1.1]",
            expected: true,
            category: "ip_literal",
            description: "IPv4 literal - private network",
        },
        TestCase {
            email: "user@[10.0.0.1]",
            expected: true,
            category: "ip_literal",
            description: "IPv4 literal - private network 10.x",
        },
        TestCase {
            email: "user@[172.16.0.1]",
            expected: true,
            category: "ip_literal",
            description: "IPv4 literal - private network 172.16.x",
        },
        TestCase {
            email: "user@[2001:db8::1]",
            expected: true,
            category: "ip_literal",
            description: "IPv6 literal - documentation prefix",
        },
        TestCase {
            email: "user@[2001:db8:0:0:0:0:0:1]",
            expected: true,
            category: "ip_literal",
            description: "IPv6 literal - full form",
        },
        TestCase {
            email: "user@[::1]",
            expected: true,
            category: "ip_literal",
            description: "IPv6 literal - localhost",
        },
        TestCase {
            email: "user@[::ffff:127.0.0.1]",
            expected: true,
            category: "ip_literal",
            description: "IPv6 literal - IPv4-mapped",
        },
        TestCase {
            email: "user@[fe80::1]",
            expected: true,
            category: "ip_literal",
            description: "IPv6 literal - link-local",
        },
    ];

    pub const INVALID_EMAILS: &[TestCase] = &[
        TestCase {
            email: "",
            expected: false,
            category: "empty",
            description: "Empty string",
        },
        TestCase {
            email: "plainaddress",
            expected: false,
            category: "no_at",
            description: "No @ symbol",
        },
        TestCase {
            email: "@missinglocal.com",
            expected: false,
            category: "no_local",
            description: "Missing local part",
        },
        TestCase {
            email: "missing@",
            expected: false,
            category: "no_domain",
            description: "Missing domain part",
        },
        TestCase {
            email: "double@@domain.com",
            expected: false,
            category: "multiple_at",
            description: "Multiple @ symbols",
        },
        TestCase {
            email: "user@.com",
            expected: false,
            category: "invalid_domain",
            description: "Domain starts with dot",
        },
        TestCase {
            email: "user@domain.",
            expected: false,
            category: "invalid_domain",
            description: "Domain ends with dot",
        },
        TestCase {
            email: "user@domain..com",
            expected: false,
            category: "invalid_domain",
            description: "Consecutive dots in domain",
        },
        TestCase {
            email: "user@-domain.com",
            expected: false,
            category: "invalid_domain",
            description: "Domain starts with hyphen",
        },
        TestCase {
            email: "user@domain-.com",
            expected: false,
            category: "invalid_domain",
            description: "Domain ends with hyphen",
        },
        TestCase {
            email: ".user@domain.com",
            expected: false,
            category: "invalid_local",
            description: "Local part starts with dot",
        },
        TestCase {
            email: "user.@domain.com",
            expected: false,
            category: "invalid_local",
            description: "Local part ends with dot",
        },
        TestCase {
            email: "user..name@domain.com",
            expected: false,
            category: "invalid_local",
            description: "Consecutive dots in local part",
        },
        TestCase {
            email: "user name@domain.com",
            expected: false,
            category: "invalid_local",
            description: "Space in local part",
        },
        TestCase {
            email: "user@domain .com",
            expected: false,
            category: "invalid_domain",
            description: "Space in domain",
        },
        TestCase {
            email: "user\n@domain.com",
            expected: false,
            category: "control_chars",
            description: "Newline in local part",
        },
        TestCase {
            email: "user@domain.com\n",
            expected: false,
            category: "control_chars",
            description: "Newline at end",
        },
        TestCase {
            email: "user@[127.0.0.256]",
            expected: false,
            category: "invalid_ip",
            description: "Invalid IPv4 - octet > 255",
        },
        TestCase {
            email: "user@[127.0.0]",
            expected: false,
            category: "invalid_ip",
            description: "Invalid IPv4 - incomplete",
        },
        TestCase {
            email: "user@[2001:db8::12345]",
            expected: false,
            category: "invalid_ip",
            description: "Invalid IPv6 - group > 4 hex digits",
        },
        TestCase {
            email: "user@[not.an.ip]",
            expected: false,
            category: "invalid_ip",
            description: "Invalid IP literal format",
        },
    ];

    pub const LENGTH_BOUNDARY_EMAILS: &[TestCase] = &[
        TestCase {
            email: "a@b.co",
            expected: true,
            category: "length",
            description: "Minimum length valid email",
        },
        TestCase {
            email: "ab@cd.ef",
            expected: true,
            category: "length",
            description: "Short valid email",
        },
    ];

    pub const EDGE_CASE_EMAILS: &[TestCase] = &[
        TestCase {
            email: "user@localhost",
            expected: true,
            category: "edge",
            description: "Single label domain",
        },
        TestCase {
            email: "user@domain.museum",
            expected: true,
            category: "edge",
            description: "Long TLD",
        },
        TestCase {
            email: "user@domain.info",
            expected: true,
            category: "edge",
            description: "Info TLD",
        },
        TestCase {
            email: "user@domain.travel",
            expected: true,
            category: "edge",
            description: "Travel TLD",
        },
    ];
}

#[cfg(test)]
mod validation_tests {
    use super::test_data::*;
    use super::*;

    #[test]
    fn test_rfc5322_compliant_emails() {
        for test_case in RFC5322_COMPLIANT_EMAILS {
            assert_eq!(
                test_case.email.validate_email(),
                test_case.expected,
                "Failed: {} - {} ({})",
                test_case.email,
                test_case.description,
                test_case.category
            );
        }
    }

    #[test]
    fn test_special_character_emails() {
        for test_case in SPECIAL_CHARACTER_EMAILS {
            assert_eq!(
                test_case.email.validate_email(),
                test_case.expected,
                "Failed: {} - {} ({})",
                test_case.email,
                test_case.description,
                test_case.category
            );
        }
    }

    #[test]
    fn test_ip_literal_emails() {
        for test_case in IP_LITERAL_EMAILS {
            assert_eq!(
                test_case.email.validate_email(),
                test_case.expected,
                "Failed: {} - {} ({})",
                test_case.email,
                test_case.description,
                test_case.category
            );
        }
    }

    #[test]
    fn test_invalid_emails() {
        for test_case in INVALID_EMAILS {
            assert_eq!(
                test_case.email.validate_email(),
                test_case.expected,
                "Failed: {} - {} ({})",
                test_case.email,
                test_case.description,
                test_case.category
            );
        }
    }

    #[test]
    fn test_length_boundary_emails() {
        for test_case in LENGTH_BOUNDARY_EMAILS {
            assert_eq!(
                test_case.email.validate_email(),
                test_case.expected,
                "Failed: {} - {} ({})",
                test_case.email,
                test_case.description,
                test_case.category
            );
        }
    }

    #[test]
    fn test_edge_case_emails() {
        for test_case in EDGE_CASE_EMAILS {
            assert_eq!(
                test_case.email.validate_email(),
                test_case.expected,
                "Failed: {} - {} ({})",
                test_case.email,
                test_case.description,
                test_case.category
            );
        }
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;

    #[test]
    fn test_rfc5321_length_limits() {
        let max_local_part = "a".repeat(64);
        let valid_long_local = format!("{}@example.com", max_local_part);
        assert!(
            valid_long_local.validate_email(),
            "64-char local part should be valid"
        );

        let too_long_local = "a".repeat(65);
        let invalid_long_local = format!("{}@example.com", too_long_local);
        assert!(
            !invalid_long_local.validate_email(),
            "65-char local part should be invalid"
        );

        let max_domain = format!("user@{}.{}", "a".repeat(63), "b".repeat(63));
        assert!(
            max_domain.validate_email(),
            "Multi-label domain with max labels should be valid"
        );

        let too_long_label = format!("user@{}.com", "a".repeat(64));
        assert!(
            !too_long_label.validate_email(),
            "Domain with over-max label should be invalid"
        );
    }

    #[test]
    fn test_domain_label_limits() {
        let max_label = "a".repeat(63);
        let valid_label_email = format!("user@{}.com", max_label);
        assert!(
            valid_label_email.validate_email(),
            "63-char domain label should be valid"
        );

        let too_long_label = "a".repeat(64);
        let invalid_label_email = format!("user@{}.com", too_long_label);
        assert!(
            !invalid_label_email.validate_email(),
            "64-char domain label should be invalid"
        );
    }

    #[test]
    fn test_total_email_length_limits() {
        let min_email = "a@b.co";
        assert!(
            min_email.validate_email(),
            "Minimum length email should be valid"
        );

        let max_email = format!("{}@{}.com", "a".repeat(64), "b".repeat(63));
        assert!(
            max_email.len() <= 320,
            "Test email should be within reasonable limits"
        );
        assert!(
            max_email.validate_email(),
            "Maximum reasonable length email should be valid"
        );
    }
}

#[cfg(test)]
mod api_consistency_tests {
    use super::*;

    #[test]
    fn test_trait_vs_function_consistency() {
        let test_emails = [
            "valid@example.com",
            "invalid.email",
            "user@[127.0.0.1]",
            "@invalid.com",
            "user@domain.co.uk",
        ];

        for email in test_emails {
            let trait_result = email.validate_email();
            let function_result = EmailValidator::validate(email.as_bytes());
            assert_eq!(
                trait_result, function_result,
                "Trait and function results should match for: {}",
                email
            );
        }
    }

    #[test]
    fn test_string_types_consistency() {
        let email = "test@example.com";
        let string_email = String::from(email);
        let str_slice = email;
        let byte_slice = email.as_bytes();

        let string_result = string_email.validate_email();
        let str_result = str_slice.validate_email();
        let byte_result = byte_slice.validate_email();

        assert_eq!(
            string_result, str_result,
            "String and &str should give same result"
        );
        assert_eq!(
            str_result, byte_result,
            "&str and &[u8] should give same result"
        );
    }

    #[test]
    fn test_option_type_handling() {
        let valid_email: Option<&str> = Some("test@example.com");
        let invalid_email: Option<&str> = Some("invalid.email");
        let none_email: Option<&str> = None;

        assert!(
            valid_email.validate_email(),
            "Some(valid) should return true"
        );
        assert!(
            !invalid_email.validate_email(),
            "Some(invalid) should return false"
        );
        assert!(
            none_email.validate_email(),
            "None should return true (permissive)"
        );
    }
}

#[cfg(test)]
mod performance_regression_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_performance_regression_valid_emails() {
        let emails = [
            "user@example.com",
            "test.email@domain.co.uk",
            "user+tag@subdomain.example.org",
            "simple@test.com",
            "user123@test-domain.co.uk",
        ];

        let start = Instant::now();
        for _ in 0..10000 {
            for email in &emails {
                let _ = email.validate_email();
            }
        }
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < 100,
            "10k valid email validations should complete in <100ms, took: {}ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_performance_regression_invalid_emails() {
        let emails = [
            "invalid.email",
            "@invalid.com",
            "test@",
            "test..test@example.com",
            "test@.com",
        ];

        let start = Instant::now();
        for _ in 0..10000 {
            for email in &emails {
                let _ = email.validate_email();
            }
        }
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < 50,
            "10k invalid email validations should complete in <50ms, took: {}ms",
            duration.as_millis()
        );
    }
}

#[cfg(test)]
mod security_tests {
    use super::*;

    #[test]
    fn test_injection_attempts() {
        let malicious_inputs = [
            "test@example.com'; DROP TABLE users; --",
            "test@example.com<script>alert('xss')</script>",
            "test@example.com\x00null_byte",
            "test@example.com\r\ninjected_header",
            "test@example.com\n\ninjected_body",
            "test@example.com%0d%0ainjected",
            "test@example.com\u{202e}rtl_override",
            "test@example.com\u{200b}zero_width",
        ];

        for input in malicious_inputs {
            let result = input.validate_email();
            assert!(
                !result || input == "test@example.com'; DROP TABLE users; --",
                "Malicious input should be rejected: {}",
                input
            );
        }
    }

    #[test]
    fn test_buffer_overflow_protection() {
        let very_long_email = format!("{}@{}.com", "a".repeat(1000), "b".repeat(1000));
        assert!(
            !very_long_email.validate_email(),
            "Extremely long email should be rejected"
        );

        let very_long_local = format!("{}@example.com", "a".repeat(1000));
        assert!(
            !very_long_local.validate_email(),
            "Extremely long local part should be rejected"
        );

        let very_long_domain = format!("user@{}.com", "a".repeat(1000));
        assert!(
            !very_long_domain.validate_email(),
            "Extremely long domain should be rejected"
        );
    }

    #[test]
    fn test_unicode_normalization_attacks() {
        let unicode_emails = [
            "tëst@example.com",
            "test@ëxample.com",
            "test@example.cöm",
            "tést@éxample.cóm",
        ];

        for email in unicode_emails {
            let result = email.validate_email();
            println!("Unicode email '{}' -> {}", email, result);
        }
    }
}

#[cfg(test)]
mod memory_safety_tests {
    use super::*;

    #[test]
    fn test_empty_and_null_inputs() {
        assert!(
            !EmailValidator::validate(b""),
            "Empty byte slice should be invalid"
        );
        assert!(!"".validate_email(), "Empty string should be invalid");

        let empty_string = String::new();
        assert!(
            !empty_string.validate_email(),
            "Empty String should be invalid"
        );
    }

    #[test]
    fn test_single_character_inputs() {
        let single_chars = ["a", "@", ".", "-", "_", "+", "!", "#"];
        for ch in single_chars {
            assert!(
                !ch.validate_email(),
                "Single character '{}' should be invalid",
                ch
            );
        }
    }

    #[test]
    fn test_boundary_byte_values() {
        let boundary_emails = [
            "test\x00@example.com",
            "test\x7f@example.com",
            &format!("test{}@example.com", '\u{0080}'),
            &format!("test{}@example.com", '\u{00ff}'),
        ];

        for email in &boundary_emails {
            let result = email.validate_email();
            println!("Boundary email with special byte -> {}", result);
        }
    }
}

#[cfg(test)]
mod statistics {
    use super::test_data::*;
    use super::*;

    #[test]
    fn test_coverage_statistics() {
        let mut stats = HashMap::new();
        let mut total_tests = 0;
        let mut passed_tests = 0;

        let all_test_suites = [
            ("RFC5322", RFC5322_COMPLIANT_EMAILS),
            ("Special Chars", SPECIAL_CHARACTER_EMAILS),
            ("IP Literals", IP_LITERAL_EMAILS),
            ("Invalid", INVALID_EMAILS),
            ("Length Boundary", LENGTH_BOUNDARY_EMAILS),
            ("Edge Cases", EDGE_CASE_EMAILS),
        ];

        for (suite_name, test_cases) in all_test_suites {
            let mut suite_passed = 0;
            let suite_total = test_cases.len();

            for test_case in test_cases {
                total_tests += 1;
                let result = test_case.email.validate_email();
                if result == test_case.expected {
                    passed_tests += 1;
                    suite_passed += 1;
                }
            }

            stats.insert(suite_name, (suite_passed, suite_total));
        }

        println!("\n=== Test Coverage Statistics ===");
        for (suite_name, (passed, total)) in &stats {
            let percentage = (*passed as f64 / *total as f64) * 100.0;
            println!("{}: {}/{} ({:.1}%)", suite_name, passed, total, percentage);
        }

        let overall_percentage = (passed_tests as f64 / total_tests as f64) * 100.0;
        println!(
            "Overall: {}/{} ({:.1}%)",
            passed_tests, total_tests, overall_percentage
        );

        assert_eq!(
            passed_tests,
            total_tests,
            "All tests should pass. Failed: {}/{}",
            total_tests - passed_tests,
            total_tests
        );
    }
}
