
use idna::domain_to_ascii;
use crate::constants::*;
use crate::types::ValidationResult;
use crate::ip::{ValidateIp, fast_ip_precheck};

static USER_CHAR_TABLE: [bool; 256] = {
    let mut table = [false; 256];
    let mut i = 0;
    while i < 256 {
        table[i] = matches!(i as u8,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' |
            b'.' | b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' |
            b'/' | b'=' | b'?' | b'^' | b'_' | b'`' | b'{' | b'|' | b'}' | b'~' | b'-'
        );
        i += 1;
    }
    table
};

static ALPHANUMERIC_TABLE: [bool; 256] = {
    let mut table = [false; 256];
    let mut i = 0;
    while i < 256 {
        table[i] = matches!(i as u8, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9');
        i += 1;
    }
    table
};

static DOMAIN_CHAR_TABLE: [bool; 256] = {
    let mut table = [false; 256];
    let mut i = 0;
    while i < 256 {
        table[i] = matches!(i as u8, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-');
        i += 1;
    }
    table
};

pub struct EmailValidator;

impl EmailValidator {
    #[inline(always)]
    fn is_user_char(byte: u8) -> bool {
        unsafe { *USER_CHAR_TABLE.get_unchecked(byte as usize) }
    }

    #[inline(always)]
    fn is_alphanumeric_byte(byte: u8) -> bool {
        unsafe { *ALPHANUMERIC_TABLE.get_unchecked(byte as usize) }
    }

    #[inline(always)]
    fn is_domain_char(byte: u8) -> bool {
        unsafe { *DOMAIN_CHAR_TABLE.get_unchecked(byte as usize) }
    }

    #[cold]
    #[inline(never)]
    fn validate_user_part_slow_path(bytes: &[u8]) -> bool {
        if bytes.is_empty() {
            return false;
        }

        if bytes[0] == b'.' || bytes[bytes.len() - 1] == b'.' {
            return false;
        }

        let mut prev_was_dot = false;
        for &byte in bytes {
            if byte > 127 || !Self::is_user_char(byte) {
                return false;
            }
            
            if byte == b'.' {
                if prev_was_dot {
                    return false;
                }
                prev_was_dot = true;
            } else {
                prev_was_dot = false;
            }
        }
        
        true
    }

    #[inline]
    fn validate_user_part(bytes: &[u8]) -> bool {
        let len = bytes.len();
        if len == 0 || len > MAX_USER_LENGTH {
            return false;
        }
        
        if bytes[0] == b'.' || bytes[len - 1] == b'.' {
            return false;
        }
        
        if len < 8 {
            return Self::validate_user_part_slow_path(bytes);
        }
        
        let mut prev_was_dot = false;
        let (chunks, remainder) = bytes.split_at(len & !7);
        
        for chunk in chunks.chunks_exact(8) {
            unsafe {
                let chunk_u64 = u64::from_ne_bytes(chunk.try_into().unwrap_unchecked());
                if (chunk_u64 & ASCII_MASK) != 0 {
                    return false;
                }
            }
            
            for &byte in chunk {
                if !Self::is_user_char(byte) {
                    return false;
                }
                
                if byte == b'.' {
                    if prev_was_dot {
                        return false;
                    }
                    prev_was_dot = true;
                } else {
                    prev_was_dot = false;
                }
            }
        }
        
        for &byte in remainder {
            if byte > 127 || !Self::is_user_char(byte) {
                return false;
            }
            
            if byte == b'.' {
                if prev_was_dot {
                    return false;
                }
                prev_was_dot = true;
            } else {
                prev_was_dot = false;
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

        unsafe {
            let first = *label.get_unchecked(0);
            let last = *label.get_unchecked(len - 1);

            if !Self::is_alphanumeric_byte(first) || !Self::is_alphanumeric_byte(last) {
                return ValidationResult::Invalid;
            }
        }

        if len == 1 {
            return ValidationResult::Valid;
        }

        let middle = unsafe { label.get_unchecked(1..len - 1) };
        let mut has_non_ascii = false;
        
        for &byte in middle {
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
        let len = bytes.len();
        if len == 0 || len > MAX_DOMAIN_LENGTH {
            return ValidationResult::Invalid;
        }

        if bytes[0] == b'.' || bytes[len - 1] == b'.' {
            return ValidationResult::Invalid;
        }

        let mut start = 0;
        let mut requires_idn = false;
        let mut label_count = 0;

        for i in 0..len {
            unsafe {
                if *bytes.get_unchecked(i) == b'.' {
                    if i == start {
                        return ValidationResult::Invalid;
                    }
                    
                    let label_result = Self::validate_domain_label(bytes.get_unchecked(start..i));
                    match label_result {
                        ValidationResult::Invalid => return ValidationResult::Invalid,
                        ValidationResult::RequiresIdnCheck => requires_idn = true,
                        ValidationResult::Valid => {}
                    }
                    start = i + 1;
                    label_count += 1;
                }
            }
        }

        if start < len {
            let final_label_result = unsafe { Self::validate_domain_label(bytes.get_unchecked(start..)) };
            match final_label_result {
                ValidationResult::Invalid => return ValidationResult::Invalid,
                ValidationResult::RequiresIdnCheck => requires_idn = true,
                ValidationResult::Valid => {}
            }
            label_count += 1;
        }

        if label_count == 0 {
            return ValidationResult::Invalid;
        }

        if requires_idn {
            ValidationResult::RequiresIdnCheck
        } else {
            ValidationResult::Valid
        }
    }

    #[inline]
    fn validate_ip_literal(bytes: &[u8]) -> bool {
        let len = bytes.len();
        
        if len < 3 || len > 47 {
            return false;
        }
        
        unsafe {
            if *bytes.get_unchecked(0) != b'[' || *bytes.get_unchecked(len - 1) != b']' {
                return false;
            }

            let ip_bytes = bytes.get_unchecked(1..len - 1);
            
            if !fast_ip_precheck(ip_bytes) {
                return false;
            }

            let ip_str = std::str::from_utf8_unchecked(ip_bytes);
            ip_str.validate_ip()
        }
    }

    #[inline]
    fn find_last_at_position(bytes: &[u8]) -> Option<usize> {
        let len = bytes.len();
        
        if len < MIN_EMAIL_LENGTH {
            return None;
        }
        
        let mut pos = len;
        while pos > 1 {
            pos -= 1;
            unsafe {
                if *bytes.get_unchecked(pos) == b'@' && pos < len - 1 {
                    return Some(pos);
                }
            }
        }
        None
    }

    #[inline]
    pub fn validate(email_bytes: &[u8]) -> bool {
        let len = email_bytes.len();
        if len < MIN_EMAIL_LENGTH || len > MAX_EMAIL_LENGTH {
            return false;
        }

        let at_pos = match Self::find_last_at_position(email_bytes) {
            Some(pos) => pos,
            None => return false,
        };

        unsafe {
            let user_bytes = email_bytes.get_unchecked(..at_pos);
            let domain_bytes = email_bytes.get_unchecked(at_pos + 1..);

            if !Self::validate_user_part(user_bytes) {
                return false;
            }

            match Self::validate_domain_part(domain_bytes) {
                ValidationResult::Valid => true,
                ValidationResult::Invalid => Self::validate_ip_literal(domain_bytes),
                ValidationResult::RequiresIdnCheck => {
                    let domain_str = std::str::from_utf8_unchecked(domain_bytes);
                    match domain_to_ascii(domain_str) {
                        Ok(ascii_domain) => {
                            Self::validate_domain_part(ascii_domain.as_bytes()).is_valid()
                        }
                        Err(_) => false,
                    }
                }
            }
        }
    }
    
    #[inline]
    pub fn validate_str(email: &str) -> bool {
        Self::validate(email.as_bytes())
    }
    
    #[inline]
    pub fn validate_string(email: &String) -> bool {
        Self::validate(email.as_bytes())
    }
}