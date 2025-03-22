use super::*;
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
