use super::*;
#[derive(Debug, Deserialize, Clone)]
pub struct Mobile(pub String);
impl Validate for Mobile {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();
        if self.0.len() != 10 {
            let mut error = validator::ValidationError::new("length");
            error.message = Some(std::borrow::Cow::from(format!(
                "Mobile number {} must be exactly 10 digits",
                self.0
            )));
            errors.add("mobile", error);
            return Err(errors);
        }
        if !self.0.chars().all(|c| c.is_ascii_digit()) {
            let mut error = validator::ValidationError::new("numeric");
            error.message = Some(std::borrow::Cow::from(format!(
                "Mobile number {} must contain only numerals",
                self.0
            )));
            errors.add("mobile", error);
            return Err(errors);
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct MultipleMobileSearchRequest {
    pub numbers: Vec<Mobile>,
    pub state: u8,
}
impl<'a> ValidateArgs<'a> for MultipleMobileSearchRequest {
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
            error.message = Some(std::borrow::Cow::from("No mobile numbers provided"));
            errors.add("mobile_numbers", error);
            return Err(errors);
        }

        // Validate each mobile number
        for mobile in &self.numbers {
            mobile.validate()?
        }

        Ok(())
    }
}
