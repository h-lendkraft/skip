use super::*;

impl SpeedState {
    pub async fn search_multiple_number(
        &self,
        request: MultipleMobileSearchRequest,
    ) -> SpeedResult<Vec<SpeedUser>> {
        // Ensure we're logged in before attempting search
        self.ensure_logged_in().await?;

        let mut all_users = Vec::new();

        // Process each mobile number
        for mobile in request.0 {
            // Search for this individual mobile number
            match self.search_number(mobile).await {
                Ok(users) => {
                    // Add all users found for this mobile
                    all_users.extend(users);
                }
                Err(e) => {
                    // Log error but continue with other numbers
                    tracing::warn!("Error searching for mobile number: {}", e);
                }
            }
        }

        Ok(all_users)
    }

    async fn search_number(&self, mobile: Mobile) -> SpeedResult<Vec<SpeedUser>> {
        // Ensure we're logged in before attempting search
        self.ensure_logged_in().await?;

        // Get the search page to extract new CSRF token
        let home_url = format!("{}/Home/Index", self.base_url);
        let search_page = self.client.get(home_url).send().await?.text().await?;

        // Extract token for search form
        let search_token = extract_csrf_token(&search_page)?;

        // Build search form data
        let mut form = std::collections::HashMap::new();
        form.insert("searchOption", "ChnMobileno");
        form.insert("searchMobile", mobile.0.as_str());
        form.insert("__RequestVerificationToken", &search_token);

        // Perform search
        let search_url = "https://search.findcustomersdata.online/Home/Search";
        let response = self.client.post(search_url).form(&form).send().await?;

        let response_text = response.text().await?;

        // Parse response HTML and extract user data
        let users = extract_speed_user(&response_text)?;
        Ok(users)
    }
}
