use std::net::{Ipv4Addr, Ipv6Addr};

pub trait ValidateIp {
    fn validate_ip(&self) -> bool;
}

impl ValidateIp for str {
    #[inline]
    fn validate_ip(&self) -> bool {
        self.len() <= 45 && (
            self.parse::<Ipv4Addr>().is_ok() || 
            self.parse::<Ipv6Addr>().is_ok()
        )
    }
}

#[inline(always)]
pub fn is_valid_ipv4_char(byte: u8) -> bool {
    matches!(byte, b'0'..=b'9' | b'.')
}

#[inline(always)]
pub fn is_valid_ipv6_char(byte: u8) -> bool {
    matches!(byte, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' | b':')
}

pub fn fast_ip_precheck(bytes: &[u8]) -> bool {
    if bytes.is_empty() || bytes.len() > 45 {
        return false;
    }
    
    let has_colon = bytes.contains(&b':');
    let has_dot = bytes.contains(&b'.');
    
    if has_colon && has_dot {
        return bytes.iter().all(|&b| is_valid_ipv6_char(b) || b == b'.');
    } else if has_colon {
        return bytes.iter().all(|&b| is_valid_ipv6_char(b));
    } else if has_dot {
        return bytes.iter().all(|&b| is_valid_ipv4_char(b));
    }
    
    false
}