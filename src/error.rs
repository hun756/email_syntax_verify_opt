#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EmailValidationError {
    Empty = 0,
    TooShort = 1,
    TooLong = 2,
    NoAtSymbol = 3,
    MultipleAtSymbols = 4,
    InvalidUserPart = 5,
    InvalidDomainPart = 6,
    InvalidIpLiteral = 7,
    IdnProcessingFailed = 8,
}

impl EmailValidationError {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "Email is empty",
            Self::TooShort => "Email is too short",
            Self::TooLong => "Email is too long",
            Self::NoAtSymbol => "Email missing @ symbol",
            Self::MultipleAtSymbols => "Email has multiple @ symbols",
            Self::InvalidUserPart => "Invalid user part",
            Self::InvalidDomainPart => "Invalid domain part",
            Self::InvalidIpLiteral => "Invalid IP literal",
            Self::IdnProcessingFailed => "IDN processing failed",
        }
    }
}
