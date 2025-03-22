use serde::{Deserialize, Serialize};
use validator::Validate;

mod mobile;
pub use mobile::*;
mod aadhar;
pub use aadhar::*;
mod namedob;
pub use namedob::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedUser {
    // Make all fields Optional with String
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default)]
    pub dob: Option<String>,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub father: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub permanent_address: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub alt_number: Option<String>,
    #[serde(default)]
    pub identity: Option<String>,
}
// impl Validate for Mobile {
//     fn validate(&self) -> Result<(), validator::ValidationErrors> {
//         if self.0.len() != 10 {
//             let mut errors = validator::ValidationErrors::new();
//             let mut error = validator::ValidationError::new("length");
//             error.message = Some(std::borrow::Cow::from("Mobile number must be exactly 10 digits"));
//             errors.add("mobile", error);
//             return Err(errors);
//         }
//         Ok(())
//     }
// }
