#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ValidationResult {
    Valid = 0,
    Invalid = 1,
    RequiresIdnCheck = 2,
}

impl ValidationResult {
    #[inline(always)]
    pub const fn is_valid(self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    #[inline(always)]
    pub const fn is_invalid(self) -> bool {
        matches!(self, ValidationResult::Invalid)
    }

    #[inline(always)]
    pub const fn requires_idn_check(self) -> bool {
        matches!(self, ValidationResult::RequiresIdnCheck)
    }
}
