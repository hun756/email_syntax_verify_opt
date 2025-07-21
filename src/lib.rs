#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions, clippy::inline_always)]

pub mod constants;
pub mod types;
pub mod validator;
pub mod traits;
pub mod ip;
pub mod error;

pub use traits::ValidateEmail;
pub use validator::EmailValidator;
pub use types::ValidationResult;
pub use error::EmailValidationError;

#[inline]
pub fn validate_email(email: &str) -> bool {
    EmailValidator::validate_str(email)
}

#[inline]
pub fn validate_email_bytes(email: &[u8]) -> bool {
    EmailValidator::validate(email)
}