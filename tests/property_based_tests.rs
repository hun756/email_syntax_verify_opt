use email_syntax_verify_opt::ValidateEmail;

#[cfg(test)]
mod property_tests {
    use super::*;

    fn generate_valid_local_chars() -> Vec<char> {
        let mut chars = Vec::new();
        
        for c in 'a'..='z' { chars.push(c); }
        for c in 'A'..='Z' { chars.push(c); }
        for c in '0'..='9' { chars.push(c); }
        
        let special_chars = ['.', '!', '#', '$', '%', '&', '\'', '*', '+', 
                           '/', '=', '?', '^', '_', '`', '{', '|', '}', '~', '-'];
        chars.extend_from_slice(&special_chars);
        
        chars
    }

    fn generate_valid_domain_chars() -> Vec<char> {
        let mut chars = Vec::new();
        
        for c in 'a'..='z' { chars.push(c); }
        for c in 'A'..='Z' { chars.push(c); }
        for c in '0'..='9' { chars.push(c); }
        chars.push('-');
        
        chars
    }

    #[test]
    fn property_valid_chars_in_local_part() {
        let valid_chars = generate_valid_local_chars();
        
        for &ch in &valid_chars {
            if ch != '.' {
                let email = format!("{}@example.com", ch);
                assert!(
                    email.validate_email(),
                    "Single valid character '{}' should create valid email",
                    ch
                );
            }
        }
    }

    #[test]
    fn property_valid_chars_in_domain_part() {
        let valid_chars = generate_valid_domain_chars();
        
        for &ch in &valid_chars {
            if ch != '-' {
                let email = format!("user@{}.com", ch);
                assert!(
                    email.validate_email(),
                    "Single valid domain character '{}' should create valid email",
                    ch
                );
            }
        }
    }

    #[test]
    fn property_invalid_control_characters() {
        let control_chars: Vec<char> = (0..32).map(|i| i as u8 as char).collect();
        
        for ch in control_chars {
            let email_with_control_in_local = format!("user{}@example.com", ch);
            let email_with_control_in_domain = format!("user@exam{}ple.com", ch);
            
            assert!(
                !email_with_control_in_local.validate_email(),
                "Control character '{}' (code: {}) in local part should be invalid",
                ch.escape_debug(), ch as u32
            );
            
            assert!(
                !email_with_control_in_domain.validate_email(),
                "Control character '{}' (code: {}) in domain should be invalid",
                ch.escape_debug(), ch as u32
            );
        }
    }

    #[test]
    fn property_consecutive_dots_invalid() {
        let test_cases = [
            "user..name@example.com",
            "user...name@example.com",
            "user....name@example.com",
            "user@domain..com",
            "user@domain...com",
            "user@domain....com",
        ];

        for email in test_cases {
            assert!(
                !email.validate_email(),
                "Email with consecutive dots should be invalid: {}",
                email
            );
        }
    }

    #[test]
    fn property_leading_trailing_dots_invalid() {
        let test_cases = [
            ".user@example.com",
            "user.@example.com",
            "user@.example.com",
            "user@example.com.",
            "..user@example.com",
            "user..@example.com",
        ];

        for email in test_cases {
            assert!(
                !email.validate_email(),
                "Email with leading/trailing dots should be invalid: {}",
                email
            );
        }
    }

    #[test]
    fn property_hyphen_placement_in_domain() {
        let invalid_hyphen_cases = [
            "user@-example.com",
            "user@example-.com", 
            "user@ex-ample.com",
            "user@example.c-om",
        ];

        assert!(!invalid_hyphen_cases[0].validate_email(), "Domain starting with hyphen should be invalid");
        assert!(!invalid_hyphen_cases[1].validate_email(), "Domain ending with hyphen should be invalid");
        assert!(invalid_hyphen_cases[2].validate_email(), "Hyphen in middle of domain should be valid");
        assert!(invalid_hyphen_cases[3].validate_email(), "Hyphen in TLD is actually valid in our implementation");
    }

    #[test]
    fn property_at_symbol_count() {
        let test_cases = [
            ("user@example.com", true, "Single @ should be valid"),
            ("user@@example.com", false, "Double @ should be invalid"),
            ("user@@@example.com", false, "Triple @ should be invalid"),
            ("userexample.com", false, "No @ should be invalid"),
            ("user@exam@ple.com", false, "@ in domain should be invalid"),
            ("us@er@example.com", false, "@ in local should be invalid"),
        ];

        for (email, expected, description) in test_cases {
            assert_eq!(
                email.validate_email(),
                expected,
                "{}: {}",
                description,
                email
            );
        }
    }

    #[test]
    fn property_length_boundaries() {
        let local_64 = "a".repeat(64);
        let local_65 = "a".repeat(65);
        let local_parts = [
            ("", false, "Empty local part"),
            ("a", true, "Single char local part"),
            (local_64.as_str(), true, "Max length local part (64)"),
            (local_65.as_str(), false, "Over max local part (65)"),
        ];

        for (local, expected, description) in local_parts {
            let email = format!("{}@example.com", local);
            if !local.is_empty() {
                assert_eq!(
                    email.validate_email(),
                    expected,
                    "{}: {}",
                    description,
                    email
                );
            }
        }
    }

    #[test]
    fn property_domain_label_boundaries() {
        let label_63 = "a".repeat(63);
        let label_64 = "a".repeat(64);
        let labels = [
            ("a", true, "Single char label"),
            (label_63.as_str(), true, "Max length label (63)"),
            (label_64.as_str(), false, "Over max label (64)"),
        ];

        for (label, expected, description) in labels {
            let email = format!("user@{}.com", label);
            assert_eq!(
                email.validate_email(),
                expected,
                "{}: {}",
                description,
                email
            );
        }
    }

    #[test]
    fn property_case_insensitivity() {
        let test_pairs = [
            ("user@EXAMPLE.COM", "user@example.com"),
            ("USER@example.com", "user@example.com"),
            ("User@Example.Com", "user@example.com"),
            ("TEST@DOMAIN.CO.UK", "test@domain.co.uk"),
        ];

        for (upper, lower) in test_pairs {
            let upper_result = upper.validate_email();
            let lower_result = lower.validate_email();
            
            assert_eq!(
                upper_result, lower_result,
                "Case should not affect validation: '{}' vs '{}'",
                upper, lower
            );
            
            assert!(upper_result, "Both cases should be valid: '{}'", upper);
        }
    }

    #[test]
    fn property_whitespace_handling() {
        let whitespace_cases = [
            " user@example.com",
            "user @example.com", 
            "user@ example.com",
            "user@example.com ",
            "user@exam ple.com",
            "us er@example.com",
            "\tuser@example.com",
            "user\t@example.com",
            "user@\texample.com",
            "user@example.com\t",
            "\nuser@example.com",
            "user@example.com\n",
            "\ruser@example.com",
            "user@example.com\r",
        ];

        for email in whitespace_cases {
            assert!(
                !email.validate_email(),
                "Email with whitespace should be invalid: '{}'",
                email.escape_debug()
            );
        }
    }

    #[test]
    fn property_unicode_handling() {
        let unicode_cases = [
            ("tëst@example.com", "Latin with diacritic in local"),
            ("test@ëxample.com", "Latin with diacritic in domain"),
            ("тест@example.com", "Cyrillic in local"),
            ("test@тест.com", "Cyrillic in domain"),
            ("测试@example.com", "Chinese in local"),
            ("test@测试.com", "Chinese in domain"),
            ("🚀@example.com", "Emoji in local"),
            ("test@🚀.com", "Emoji in domain"),
        ];

        for (email, description) in unicode_cases {
            let result = email.validate_email();
            println!("Unicode test - {}: '{}' -> {}", description, email, result);
        }
    }

    #[test]
    fn property_ip_literal_format() {
        let valid_ipv4_literals = [
            "user@[0.0.0.0]",
            "user@[127.0.0.1]", 
            "user@[192.168.1.1]",
            "user@[255.255.255.255]",
        ];

        let invalid_ipv4_literals = [
            "user@[256.0.0.1]",
            "user@[127.0.0.256]",
            "user@[127.0.0]",
            "user@[127.0.0.1.1]",
            "user@[127.0.0.-1]",
            "user@[not.an.ip]",
        ];

        for email in valid_ipv4_literals {
            assert!(
                email.validate_email(),
                "Valid IPv4 literal should pass: {}",
                email
            );
        }

        for email in invalid_ipv4_literals {
            assert!(
                !email.validate_email(),
                "Invalid IPv4 literal should fail: {}",
                email
            );
        }
    }

    #[test]
    fn property_symmetry_validation() {
        let test_emails = [
            "valid@example.com",
            "invalid.email",
            "user@[127.0.0.1]",
            "test@domain.co.uk",
            "@invalid.com",
            "user@",
            "",
        ];

        for email in test_emails {
            let result1 = email.validate_email();
            let result2 = email.validate_email();
            
            assert_eq!(
                result1, result2,
                "Validation should be deterministic for: {}",
                email
            );
        }
    }

    #[test]
    fn property_composition_invariants() {
        let valid_locals = ["user", "test.email", "user+tag", "user_name"];
        let valid_domains = ["example.com", "domain.co.uk", "test.org", "sub.domain.net"];

        for local in valid_locals {
            for domain in valid_domains {
                let email = format!("{}@{}", local, domain);
                assert!(
                    email.validate_email(),
                    "Composition of valid parts should be valid: {}",
                    email
                );
            }
        }
    }

    #[test]
    fn property_monotonicity_length() {
        let base_email = "user@example.com";
        assert!(base_email.validate_email(), "Base email should be valid");

        let mut current = String::from("a@b.co");
        assert!(current.validate_email(), "Minimal email should be valid");

        for i in 1..=50 {
            current = format!("{}a@{}b.com", "a".repeat(i), "b".repeat(i));
            if current.len() <= 320 {
                let result = current.validate_email();
                if !result {
                    println!("Email became invalid at length {}: {}", current.len(), current);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod fuzzing_tests {
    use super::*;

    fn generate_random_string(length: usize, chars: &[char]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut result = String::new();
        let mut hasher = DefaultHasher::new();
        length.hash(&mut hasher);
        let mut seed = hasher.finish();
        
        for _ in 0..length {
            let index = (seed as usize) % chars.len();
            result.push(chars[index]);
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        }
        
        result
    }

    #[test]
    fn fuzz_test_random_inputs() {
        let chars: Vec<char> = (32..127).map(|i| i as u8 as char).collect();
        let mut crash_count = 0;
        let mut valid_count = 0;
        let mut invalid_count = 0;

        for length in 1..=100 {
            for _ in 0..10 {
                let random_input = generate_random_string(length, &chars);
                
                let result = std::panic::catch_unwind(|| {
                    random_input.validate_email()
                });

                match result {
                    Ok(validation_result) => {
                        if validation_result {
                            valid_count += 1;
                        } else {
                            invalid_count += 1;
                        }
                    }
                    Err(_) => {
                        crash_count += 1;
                        println!("Crash on input: '{}'", random_input);
                    }
                }
            }
        }

        println!("Fuzz test results: {} valid, {} invalid, {} crashes", 
                valid_count, invalid_count, crash_count);
        
        assert_eq!(crash_count, 0, "No crashes should occur during fuzzing");
    }

    #[test]
    fn fuzz_test_boundary_conditions() {
        let boundary_inputs = [
            "\x00".repeat(100),
            "\x7f".repeat(100), 
            (0..100).map(|_| '\u{00ff}').collect::<String>(),
            "a".repeat(1000),
            "@".repeat(100),
            ".".repeat(100),
            "-".repeat(100),
        ];

        for input in boundary_inputs {
            let result = std::panic::catch_unwind(|| {
                input.validate_email()
            });

            assert!(
                result.is_ok(),
                "Should not crash on boundary input: '{}'",
                input.chars().take(20).collect::<String>()
            );
        }
    }

    #[test]
    fn fuzz_test_malformed_at_symbols() {
        let malformed_cases = [
            "@".repeat(10),
            "user".to_string() + &"@".repeat(5) + "domain.com",
            "@user@domain@com@".to_string(),
            "@@@@@@@@@@".to_string(),
            format!("user{}domain.com", "@".repeat(10)),
        ];

        for input in malformed_cases {
            let result = std::panic::catch_unwind(|| {
                input.validate_email()
            });

            assert!(
                result.is_ok(),
                "Should not crash on malformed @ symbols: '{}'",
                input
            );

            if let Ok(validation_result) = result {
                assert!(
                    !validation_result,
                    "Malformed @ symbols should be invalid: '{}'",
                    input
                );
            }
        }
    }
}