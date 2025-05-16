use super::*;
impl SpeedState {
    pub async fn search_multiple_aadhar(
        &self,
        request: MultipleAadharSearchRequest,
    ) -> SpeedResult<Vec<SpeedUser>> {
        // Ensure we're logged in before attempting search
        self.ensure_logged_in().await?;

        let mut all_users = Vec::new();
        let state_details = self.get_region_path(request.state)?;

        // Process each Aadhar number
        for aadhar in request.numbers {
            // Search for this individual Aadhar number
            let users = self.search_aadhar(state_details.clone(), aadhar).await?;
            all_users.extend(users);
        }

        Ok(all_users)
    }

    async fn search_aadhar(
        &self,
        state_details: std::sync::Arc<SpeedSearch>,
        aadhar: Aadhar,
    ) -> SpeedResult<Vec<SpeedUser>> {
        // Ensure we're logged in before attempting search
        self.ensure_logged_in().await?;

        // Get the search page to extract new CSRF token
        let state_url = format!("{}/{}", self.base_url, state_details.page);
        let search_page = self.client.get(&state_url).send().await?.text().await?;

        // Extract token for search form
        let search_token = extract_csrf_token(&search_page)?;

        // Build search form data
        let mut form = std::collections::HashMap::new();
        form.insert("searchOption", "Chnidentity");
        form.insert("searchValue", aadhar.0.as_str());
        form.insert("__RequestVerificationToken", &search_token);

        // Perform search
        let response = self
            .client
            .post(self.base_url.clone() + "/" + state_details.form)
            .form(&form)
            .send()
            .await?;

        let response_text = response.text().await?;

        // Parse response HTML and extract user data
        let users = extract_speed_user(&response_text)?;
        Ok(users)
    }
}
