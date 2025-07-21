use idna::domain_to_ascii;
use crate::constants::*;
use crate::types::ValidationResult;
use crate::ip::ValidateIp;

pub struct EmailValidator;

impl EmailValidator {
    #[inline(always)]
    const fn is_user_char(byte: u8) -> bool {
        matches!(byte,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' |
            b'.' | b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' |
            b'/' | b'=' | b'?' | b'^' | b'_' | b'`' | b'{' | b'|' | b'}' | b'~' | b'-'
        )
    }

    #[inline(always)]
    const fn is_alphanumeric_byte(byte: u8) -> bool {
        matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9')
    }

    #[inline(always)]
    const fn is_domain_char(byte: u8) -> bool {
        matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-')
    }

    #[inline(always)]
    const fn is_ip_char(byte: u8) -> bool {
        matches!(byte, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' | b':' | b'.')
    }

    #[cold]
    #[inline(never)]
    fn validate_user_part_slow_path(bytes: &[u8]) -> bool {
        if bytes.is_empty() {
            return false;
        }
        
        for &byte in bytes {
            if byte > 127 || !Self::is_user_char(byte) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn validate_user_part(bytes: &[u8]) -> bool {
        if bytes.len() > MAX_USER_LENGTH {
            return false;
        }
        
        if bytes.len() < 16 {
            return Self::validate_user_part_slow_path(bytes);
        }
        
        let chunks = bytes.chunks_exact(8);
        let remainder = chunks.remainder();
        
        for chunk in chunks {
            unsafe {
                let chunk_u64 = u64::from_ne_bytes(chunk.try_into().unwrap_unchecked());
                if (chunk_u64 & 0x8080808080808080) != 0 {
                    return Self::validate_user_part_slow_path(bytes);
                }
            }
            
            for &byte in chunk {
                if !Self::is_user_char(byte) {
                    return false;
                }
            }
        }
        
        for &byte in remainder {
            if byte > 127 || !Self::is_user_char(byte) {
                return false;
            }
        }
        
        true
    }

    #[inline]
    fn validate_domain_label(label: &[u8]) -> ValidationResult {
        let len = label.len();
        
        if len == 0 || len > MAX_LABEL_LENGTH {
            return ValidationResult::Invalid;
        }

        let first = label[0];
        let last = label[len - 1];

        if !Self::is_alphanumeric_byte(first) || !Self::is_alphanumeric_byte(last) {
            return ValidationResult::Invalid;
        }

        if len == 1 {
            return ValidationResult::Valid;
        }

        let mut has_non_ascii = false;
        
        for &byte in &label[1..len - 1] {
            if byte > 127 {
                has_non_ascii = true;
            } else if !Self::is_domain_char(byte) {
                return ValidationResult::Invalid;
            }
        }

        if has_non_ascii {
            ValidationResult::RequiresIdnCheck
        } else {
            ValidationResult::Valid
        }
    }

    #[inline]
    fn validate_domain_part(bytes: &[u8]) -> ValidationResult {
        if bytes.is_empty() || bytes.len() > MAX_DOMAIN_LENGTH {
            return ValidationResult::Invalid;
        }

        let mut start = 0;
        let mut requires_idn = false;

        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b'.' {
                let label_result = Self::validate_domain_label(&bytes[start..i]);
                match label_result {
                    ValidationResult::Invalid => return ValidationResult::Invalid,
                    ValidationResult::RequiresIdnCheck => requires_idn = true,
                    ValidationResult::Valid => {}
                }
                start = i + 1;
            }
        }

        let final_label_result = Self::validate_domain_label(&bytes[start..]);
        match final_label_result {
            ValidationResult::Invalid => ValidationResult::Invalid,
            ValidationResult::RequiresIdnCheck => ValidationResult::RequiresIdnCheck,
            ValidationResult::Valid => {
                if requires_idn {
                    ValidationResult::RequiresIdnCheck
                } else {
                    ValidationResult::Valid
                }
            }
        }
    }

    #[inline]
    fn validate_ip_literal(bytes: &[u8]) -> bool {
        let len = bytes.len();
        
        if len < 3 || bytes[0] != b'[' || bytes[len - 1] != b']' {
            return false;
        }

        let ip_bytes = &bytes[1..len - 1];
        
        for &byte in ip_bytes {
            if !Self::is_ip_char(byte) {
                return false;
            }
        }

        unsafe {
            let ip_str = std::str::from_utf8_unchecked(ip_bytes);
            ip_str.validate_ip()
        }
    }

    #[inline]
    fn find_last_at_position(bytes: &[u8]) -> Option<usize> {
        let len = bytes.len();
        let mut pos = len;

        while pos > 0 {
            pos -= 1;
            if bytes[pos] == b'@' {
                return if pos > 0 && pos < len - 1 {
                    Some(pos)
                } else {
                    None
                };
            }
        }
        None
    }

    #[inline]
    pub fn validate(email_bytes: &[u8]) -> bool {
        if email_bytes.is_empty() {
            return false;
        }

        let at_pos = match Self::find_last_at_position(email_bytes) {
            Some(pos) => pos,
            None => return false,
        };

        let user_bytes = &email_bytes[..at_pos];
        let domain_bytes = &email_bytes[at_pos + 1..];

        if !Self::validate_user_part(user_bytes) {
            return false;
        }

        match Self::validate_domain_part(domain_bytes) {
            ValidationResult::Valid => true,
            ValidationResult::Invalid => Self::validate_ip_literal(domain_bytes),
            ValidationResult::RequiresIdnCheck => {
                unsafe {
                    let domain_str = std::str::from_utf8_unchecked(domain_bytes);
                    match domain_to_ascii(domain_str) {
                        Ok(ascii_domain) => {
                            Self::validate_domain_part(ascii_domain.as_bytes()) == ValidationResult::Valid
                        }
                        Err(_) => false,
                    }
                }
            }
        }
    }
}