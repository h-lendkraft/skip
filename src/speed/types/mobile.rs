use super::*;
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
