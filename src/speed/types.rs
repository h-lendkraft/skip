use serde::{Deserialize, Serialize};

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
use validator::Validate;
#[derive(Debug, Deserialize, Clone)]
pub struct Mobile(pub String);
impl Validate for Mobile {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0.len() != 10 {
            let mut errors = validator::ValidationErrors::new();
            errors.add("mobile", validator::ValidationError::new("length"));
            return Err(errors);
        }
        Ok(())
    }
}

impl From<String> for Mobile {
    fn from(s: String) -> Self {
        Mobile(s)
    }
}

impl AsRef<str> for Mobile {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct MultipleMobileSearchRequest(pub Vec<Mobile>);
impl Validate for MultipleMobileSearchRequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0.is_empty() {
            let mut errors = validator::ValidationErrors::new();
            errors.add("mobile_numbers", validator::ValidationError::new("length"));
            return Err(errors);
        }
        // Validate each mobile number
        for mobile in &self.0 {
            mobile.validate()?
        }
        Ok(())
    }
}
#[derive(Debug, Deserialize, Clone)]
pub struct Aadhar(pub String);

impl Validate for Aadhar {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0.len() != 12 {
            let mut errors = validator::ValidationErrors::new();
            errors.add("aadhar", validator::ValidationError::new("length"));
            return Err(errors);
        }
        Ok(())
    }
}

impl From<String> for Aadhar {
    fn from(s: String) -> Self {
        Aadhar(s)
    }
}

impl AsRef<str> for Aadhar {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct MultipleAadharSearchRequest(pub Vec<Aadhar>);

impl Validate for MultipleAadharSearchRequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0.is_empty() {
            let mut errors = validator::ValidationErrors::new();
            errors.add("aadhar_numbers", validator::ValidationError::new("length"));
            return Err(errors);
        }
        // Validate each Aadhar number
        for aadhar in &self.0 {
            aadhar.validate()?
        }
        Ok(())
    }
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
