use super::*;
impl SpeedState {
    pub async fn search_multiple_name_dob(
        &self,
        request: MultipleNameDobSearchRequest,
    ) -> SpeedResult<Vec<SpeedUser>> {
        // Ensure we're logged in before attempting search
        self.ensure_logged_in().await?;

        let mut all_users = Vec::new();

        // Process each Aadhar number
        for aadhar in request.0 {
            // Search for this individual Aadhar number
            match self.search_name_dob(aadhar).await {
                Ok(users) => {
                    // Add all users found for this Aadhar
                    all_users.extend(users);
                }
                Err(e) => {
                    // Log error but continue with other numbers
                    tracing::warn!("Error searching for Aadhar number: {}", e);
                }
            }
        }

        Ok(all_users)
    }
    async fn search_name_dob(&self, name_dob: NameDobSearchRequest) -> SpeedResult<Vec<SpeedUser>> {
        // Ensure we're logged in before attempting search
        self.ensure_logged_in().await?;

        // Get the search page to extract new CSRF token
        let home_url = format!("{}/Home/Index", self.base_url);
        let search_page = self.client.get(home_url).send().await?.text().await?;

        // Extract token for search form
        let search_token = extract_csrf_token(&search_page)?;

        // Build search form data
        let mut form = std::collections::HashMap::new();
        form.insert("searchOption", "ChnDOB");
        form.insert("cnamedob", name_dob.name.0.as_str());
        form.insert("cdob", name_dob.dob.0.as_str());
        form.insert("__RequestVerificationToken", &search_token);

        // Perform search
        let response = self
            .client
            .post(self.base_url.clone() + &self.search_append)
            .form(&form)
            .send()
            .await?;

        let response_text = response.text().await?;

        // Parse response HTML and extract user data
        let users = extract_speed_user(&response_text)?;
        Ok(users)
    }
}
