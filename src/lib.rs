#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions, clippy::inline_always)]

pub mod constants;
pub mod error;
pub mod ip;
pub mod traits;
pub mod types;
pub mod validator;

pub use error::EmailValidationError;
pub use traits::ValidateEmail;
pub use types::ValidationResult;
pub use validator::EmailValidator;

#[inline]
pub fn validate_email(email: &str) -> bool {
    EmailValidator::validate_str(email)
}

#[inline]
pub fn validate_email_bytes(email: &[u8]) -> bool {
    EmailValidator::validate(email)
}
