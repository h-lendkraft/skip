mod types;
pub use types::*;
mod client;
pub(crate) mod handlers;

#[derive(Debug, Clone)]
pub struct SpeedState {
    pub client: reqwest::Client,
    pub base_url: String,
    pub search_append: String,
    pub passwd: String,
    pub user: String,
}
impl SpeedState {
    pub fn new(
        client: reqwest::Client,
        base_url: String,
        search_append: String,
        passwd: String,
        user: String,
    ) -> Self {
        Self {
            client,
            base_url,
            search_append,
            passwd,
            user,
        }
    }
}
