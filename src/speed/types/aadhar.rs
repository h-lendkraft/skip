use super::*;
#[derive(Debug, Deserialize, Clone)]
pub struct Aadhar(pub String);

impl Validate for Aadhar {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0.len() != 12 || !self.0.chars().all(|c| c.is_ascii_digit()) {
            let mut errors = validator::ValidationErrors::new();
            let mut error = validator::ValidationError::new("invalid_aadhar");
            error.message = Some(std::borrow::Cow::from(format!(
                "Aadhar number must be exactly 12 digits: {}",
                self.0
            )));
            errors.add("aadhar", error);
            return Err(errors);
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct MultipleAadharSearchRequest {
    pub numbers: Vec<Aadhar>,
    pub state: u8,
}

impl<'a> ValidateArgs<'a> for MultipleAadharSearchRequest {
    type Args = &'a [u8];
    fn validate_with_args(
        &self,
        valid_codes: Self::Args,
    ) -> Result<(), validator::ValidationErrors> {
        // Validate state code
        let mut errors = validator::ValidationErrors::new();
        if !valid_codes.contains(&self.state) {
            let mut error = validator::ValidationError::new("invalid_state");
            error.message = Some(std::borrow::Cow::from(format!(
                "Invalid state code: {}",
                self.state
            )));
            errors.add("state_code", error);
            return Err(errors);
        }

        if self.numbers.is_empty() {
            let mut error = validator::ValidationError::new("empty");
            error.message = Some(std::borrow::Cow::from("No Aadhar numbers provided"));
            errors.add("aadhar_numbers", error);
            return Err(errors);
        }

        // Validate each Aadhar number
        for aadhar in &self.numbers {
            aadhar.validate()?
        }

        Ok(())
    }
}
