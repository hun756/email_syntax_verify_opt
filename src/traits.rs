use crate::validator::EmailValidator;

pub trait ValidateEmail {
    fn validate_email(&self) -> bool;
}

impl ValidateEmail for str {
    #[inline]
    fn validate_email(&self) -> bool {
        EmailValidator::validate_str(self)
    }
}

impl ValidateEmail for String {
    #[inline]
    fn validate_email(&self) -> bool {
        EmailValidator::validate_string(self)
    }
}

impl ValidateEmail for &str {
    #[inline]
    fn validate_email(&self) -> bool {
        EmailValidator::validate_str(self)
    }
}

impl<T> ValidateEmail for Option<T>
where
    T: ValidateEmail,
{
    #[inline]
    fn validate_email(&self) -> bool {
        self.as_ref().map_or(true, |email| email.validate_email())
    }
}

impl<T> ValidateEmail for Box<T>
where
    T: ValidateEmail,
{
    #[inline]
    fn validate_email(&self) -> bool {
        T::validate_email(self)
    }
}

impl ValidateEmail for [u8] {
    #[inline]
    fn validate_email(&self) -> bool {
        EmailValidator::validate(self)
    }
}

impl ValidateEmail for &[u8] {
    #[inline]
    fn validate_email(&self) -> bool {
        EmailValidator::validate(self)
    }
}

impl ValidateEmail for Vec<u8> {
    #[inline]
    fn validate_email(&self) -> bool {
        EmailValidator::validate(self)
    }
}
