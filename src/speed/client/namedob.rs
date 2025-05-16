use super::*;
impl SpeedState {
    pub async fn search_multiple_name_dob(
        &self,
        request: MultipleNameDobSearchRequest,
    ) -> SpeedResult<Vec<SpeedUser>> {
        // Ensure we're logged in before attempting search
        self.ensure_logged_in().await?;

        let mut all_users = Vec::new();
        let state_details = self.get_region_path(request.state)?;

        // Process each name-dob pair
        for pair in request.pairs {
            // Search for this individual name-dob pair
            let users = self.search_name_dob(state_details.clone(), pair).await?;
            all_users.extend(users);
        }

        Ok(all_users)
    }
    async fn search_name_dob(
        &self,
        state_details: std::sync::Arc<SpeedSearch>,
        name_dob: NameDobSearchRequest,
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
        form.insert("searchOption", "ChnDOB");
        form.insert("cnamedob", name_dob.name.0.as_str());
        let dob_normalized = name_dob.dob.normalize();
        form.insert("cdob", dob_normalized.0.as_str());
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
