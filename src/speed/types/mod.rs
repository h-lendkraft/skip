use serde::{Deserialize, Serialize};
use validator::{Validate, ValidateArgs};

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
#[derive(Debug, Clone)]
pub struct SpeedSearch {
    pub page: &'static str,
    pub form: &'static str,
    pub mobile: Option<&'static str>,
    pub namedob: Option<&'static str>,
}
impl SpeedSearch {
    pub fn new(
        page: &'static str,
        form: &'static str,
        mobile: Option<&'static str>,
        namedob: Option<&'static str>,
    ) -> Self {
        Self {
            page,
            form,
            mobile,
            namedob,
        }
    }
}
