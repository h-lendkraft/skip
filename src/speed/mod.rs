mod types;
pub use types::*;
mod client;
pub(crate) mod handlers;

#[derive(Debug, Clone)]
pub struct SpeedState {
    pub client: reqwest::Client,
    pub base_url: String,
    pub passwd: String,
    pub user: String,
}
impl SpeedState {
    pub fn new(client: reqwest::Client, base_url: String, passwd: String, user: String) -> Self {
        Self {
            client,
            base_url,
            passwd,
            user,
        }
    }
}
