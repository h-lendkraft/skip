use super::*;
#[derive(Debug, Deserialize, Clone)]
pub struct Name(pub String);
impl Validate for Name {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0.is_empty() {
            let mut errors = validator::ValidationErrors::new();
            errors.add("name", validator::ValidationError::new("empty"));
            return Err(errors);
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dob(pub String);
impl Dob {
    // Normalize different DOB formats to dd/mm/yyyy
    pub fn normalize(&self) -> Self {
        // The input can be in formats like 21.09.1979 or 21-9-1979
        // We need to convert it to dd/mm/yyyy
        let s = self.0.clone();

        // Remove all separators first
        let s = s.replace('.', "-").replace('/', "-");

        // Split by separator
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return self.to_owned(); // Return as is if format is unexpected
        }

        // Parse parts
        let day = parts[0].parse::<u8>().unwrap_or(0);
        let month = parts[1].parse::<u8>().unwrap_or(0);
        let year = parts[2].parse::<u32>().unwrap_or(0);

        // Ensure we have valid values
        if day == 0 || month == 0 || year == 0 {
            return self.clone();
        }

        // Format as dd/mm/yyyy
        Dob(format!("{:02}/{:02}/{:04}", day, month, year))
    }
}
use time::{Date, Month};
impl Validate for Dob {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let dob_pattern = regex::Regex::new(r"^\d{1,2}[./-]\d{1,2}[./-]\d{4}$").unwrap();
        if !dob_pattern.is_match(&self.0) {
            let mut errors = validator::ValidationErrors::new();
            errors.add("dob", validator::ValidationError::new("format"));
            return Err(errors);
        }

        // Try to parse the normalized date to validate it's a real date
        let normalized = self.normalize();
        let parts: Vec<&str> = normalized.0.split('/').collect();

        if parts.len() != 3 {
            let mut errors = validator::ValidationErrors::new();
            errors.add("dob", validator::ValidationError::new("format"));
            return Err(errors);
        }

        let day = parts[0].parse::<u8>().unwrap_or(0);
        let month_num = parts[1].parse::<u8>().unwrap_or(0);
        let year = parts[2].parse::<i32>().unwrap_or(0);

        // Convert month number to Month enum
        let month = match month_num {
            1 => Some(Month::January),
            2 => Some(Month::February),
            3 => Some(Month::March),
            4 => Some(Month::April),
            5 => Some(Month::May),
            6 => Some(Month::June),
            7 => Some(Month::July),
            8 => Some(Month::August),
            9 => Some(Month::September),
            10 => Some(Month::October),
            11 => Some(Month::November),
            12 => Some(Month::December),
            _ => None,
        };

        if month.is_none() {
            let mut errors = validator::ValidationErrors::new();
            errors.add("dob", validator::ValidationError::new("invalid_month"));
            return Err(errors);
        }

        // Try to create a valid date
        match Date::from_calendar_date(year, month.unwrap(), day) {
            Ok(_) => Ok(()),
            Err(_) => {
                let mut errors = validator::ValidationErrors::new();
                errors.add("dob", validator::ValidationError::new("invalid_date"));
                Err(errors)
            }
        }
    }
}

impl From<String> for Dob {
    fn from(s: String) -> Self {
        Dob(s)
    }
}

impl AsRef<str> for Dob {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Request type for Name and DOB search
#[derive(Debug, Deserialize)]
pub struct NameDobSearchRequest {
    pub name: Name,
    pub dob: Dob,
}

impl Validate for NameDobSearchRequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        // Validate name
        self.name.validate()?;

        // Validate DOB
        self.dob.validate()?;

        Ok(())
    }
}
// Add this to your src/speed/types.rs file

#[derive(Debug, Deserialize)]
pub struct MultipleNameDobSearchRequest(pub Vec<NameDobSearchRequest>);

impl Validate for MultipleNameDobSearchRequest {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0.is_empty() {
            let mut errors = validator::ValidationErrors::new();
            errors.add("name_dob_pairs", validator::ValidationError::new("empty"));
            return Err(errors);
        }

        // Validate each name-dob pair
        for pair in &self.0 {
            pair.validate()?;
        }

        Ok(())
    }
}
