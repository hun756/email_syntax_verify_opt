pub trait ValidateIp {
    fn validate_ip(&self) -> bool;
}

impl ValidateIp for str {
    fn validate_ip(&self) -> bool {
        self.parse::<std::net::IpAddr>().is_ok()
    }
}