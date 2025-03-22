use crate::error::{SpeedError, SpeedResult};
use crate::speed::Mobile;
use crate::speed::{Aadhar, MultipleAadharSearchRequest, MultipleMobileSearchRequest, SpeedUser};
use crate::SpeedState;
use scraper::{Html, Selector};
use std::collections::HashMap;

fn extract_csrf_token(html_content: &str) -> SpeedResult<String> {
    let document = Html::parse_document(html_content);
    let csrf_selector = Selector::parse("input[name='__RequestVerificationToken']")?;

    let token = document
        .select(&csrf_selector)
        .next()
        .ok_or_else(|| SpeedError::CsrfToken("CSRF token not found".to_string()))?
        .value()
        .attr("value")
        .ok_or_else(|| SpeedError::CsrfToken("CSRF token value not found".to_string()))?
        .to_string();

    Ok(token)
}
// Safely extract text from a cell
fn extract_cell_text(
    row: &scraper::element_ref::ElementRef,
    index: usize,
    cell_selector: &Selector,
) -> Option<String> {
    row.select(cell_selector)
        .nth(index)
        .map(|cell| {
            cell.text()
                .collect::<String>()
                .trim()
                .replace("&#x2B;", "+")
                .replace("&amp;", "&")
                .replace("!,", ",")
                .to_string()
        })
        .filter(|s| !s.is_empty())
}

pub fn extract_speed_user(html: &str) -> SpeedResult<Vec<SpeedUser>> {
    let document = Html::parse_document(html);

    // First check if we have a table with results
    if !html.contains("<tbody>") {
        tracing::debug!("No results found in HTML response");
        return Ok(Vec::new());
    }

    let row_selector = Selector::parse("tbody tr").map_err(SpeedError::Selector)?;
    let cell_selector = Selector::parse("td").map_err(SpeedError::Selector)?;

    let mut users = Vec::new();

    for row in document.select(&row_selector) {
        // Create a new user with all fields optional
        let user = SpeedUser {
            name: extract_cell_text(&row, 0, &cell_selector),
            mobile: extract_cell_text(&row, 1, &cell_selector),
            dob: extract_cell_text(&row, 2, &cell_selector),
            gender: extract_cell_text(&row, 3, &cell_selector),
            father: extract_cell_text(&row, 4, &cell_selector),
            address: extract_cell_text(&row, 5, &cell_selector),
            permanent_address: extract_cell_text(&row, 6, &cell_selector),
            email: extract_cell_text(&row, 7, &cell_selector),
            alt_number: extract_cell_text(&row, 8, &cell_selector),
            identity: extract_cell_text(&row, 9, &cell_selector),
        };

        // Only add users that have at least some data
        if user.name.is_some() || user.mobile.is_some() || user.identity.is_some() {
            users.push(user);
        }
    }

    tracing::debug!("Extracted {} users from HTML response", users.len());
    Ok(users)
}

impl SpeedState {
    async fn search_aadhar(&self, aadhar: Aadhar) -> SpeedResult<Vec<SpeedUser>> {
        // Ensure we're logged in before attempting search
        self.ensure_logged_in().await?;

        // Get the search page to extract new CSRF token
        let home_url = format!("{}/Home/Index", self.base_url);
        let search_page = self.client.get(home_url).send().await?.text().await?;

        // Extract token for search form
        let search_token = extract_csrf_token(&search_page)?;

        // Build search form data
        let mut form = std::collections::HashMap::new();
        form.insert("searchOption", "Chnidentity");
        form.insert("searchValue", aadhar.0.as_str());
        form.insert("__RequestVerificationToken", &search_token);

        // Perform search
        let search_url = "https://search.findcustomersdata.online/Home/Search";
        let response = self.client.post(search_url).form(&form).send().await?;

        let response_text = response.text().await?;

        // Parse response HTML and extract user data
        let users = extract_speed_user(&response_text)?;
        Ok(users)
    }

    pub async fn search_multiple_aadhar(
        &self,
        request: MultipleAadharSearchRequest,
    ) -> SpeedResult<Vec<SpeedUser>> {
        // Ensure we're logged in before attempting search
        self.ensure_logged_in().await?;

        let mut all_users = Vec::new();

        // Process each Aadhar number
        for aadhar in request.0 {
            // Search for this individual Aadhar number
            match self.search_aadhar(aadhar).await {
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
    pub async fn ensure_logged_in(&self) -> SpeedResult<()> {
        // Try accessing the home page to check login status
        let home_url = format!("{}/Home/Index", self.base_url);
        let home_resp = self.client.get(home_url).send().await?;
        let body = home_resp.text().await?;
        // If not logged in, perform login
        if !body.contains("Logout") {
            self.login().await?;
        }

        Ok(())
    }
    pub async fn search_multiple(
        &self,
        request: MultipleMobileSearchRequest,
    ) -> SpeedResult<Vec<SpeedUser>> {
        // Ensure we're logged in before attempting search
        self.ensure_logged_in().await?;

        let mut all_users = Vec::new();

        // Process each mobile number
        for mobile in request.0 {
            // Search for this individual mobile number
            match self.search(mobile).await {
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

    async fn search(&self, mobile: Mobile) -> SpeedResult<Vec<SpeedUser>> {
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

    pub async fn login(&self) -> SpeedResult<()> {
        // Get login page
        let login_page = self.client.get(&self.base_url).send().await?.text().await?;

        // Extract token using synchronous function
        let token = extract_csrf_token(&login_page)?;

        // Build login form
        let mut form = HashMap::new();
        form.insert("Username", self.user.as_str());
        form.insert("Password", self.passwd.as_str());
        form.insert("__RequestVerificationToken", &token);

        // Perform login
        let _ = self.client.post(&self.base_url).form(&form).send().await?;

        // Try accessing the home page directly
        let home_url = format!("{}/Home/Index", self.base_url);
        let home_resp = self.client.get(home_url).send().await?;

        tracing::debug!("Home page response status: {}", home_resp.status());
        let body = home_resp.text().await?;

        // Step 3: Check if we're logged in by looking for "Logout" text
        if body.contains("Logout") {
            tracing::info!("Login successful!");
        } else {
            tracing::error!("Login failed. Please check credentials.");
            return Err(SpeedError::Authentication(
                "Login failed - invalid credentials".to_string(),
            ));
        }

        Ok(())
    }
}
