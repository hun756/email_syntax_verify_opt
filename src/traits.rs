use std::borrow::Cow;
use crate::validator::EmailValidator;

pub trait ValidateEmail {
    fn validate_email(&self) -> bool {
        match self.as_email_string() {
            Some(email) => EmailValidator::validate(email.as_bytes()),
            None => true,
        }
    }

    fn as_email_string(&self) -> Option<Cow<str>>;
}



impl ValidateEmail for String {
    #[inline]
    fn as_email_string(&self) -> Option<Cow<str>> {
        Some(Cow::Borrowed(self.as_str()))
    }
}

impl ValidateEmail for &str {
    #[inline]
    fn as_email_string(&self) -> Option<Cow<str>> {
        Some(Cow::Borrowed(*self))
    }
}

impl<'a> ValidateEmail for Cow<'a, str> {
    #[inline]
    fn as_email_string(&self) -> Option<Cow<str>> {
        Some(self.clone())
    }
}

impl<T> ValidateEmail for &T
where
    T: ValidateEmail,
{
    #[inline]
    fn as_email_string(&self) -> Option<Cow<str>> {
        T::as_email_string(self)
    }
}

impl<T> ValidateEmail for Option<T>
where
    T: ValidateEmail,
{
    #[inline]
    fn as_email_string(&self) -> Option<Cow<str>> {
        self.as_ref().and_then(T::as_email_string)
    }
}