pub mod constants;
pub mod types;
pub mod validator;
pub mod traits;
pub mod ip;

pub use traits::ValidateEmail;
pub use validator::EmailValidator;
pub use types::ValidationResult;