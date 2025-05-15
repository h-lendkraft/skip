mod types;
pub use types::*;
mod client;
pub(crate) mod handlers;
use crate::error::{SpeedError, SpeedResult};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SpeedState {
    pub client: reqwest::Client,
    pub base_url: String,
    pub passwd: String,
    pub user: String,
    pub region_map: HashMap<u8, Arc<SpeedSearch>>,
}
impl SpeedState {
    pub fn new(client: reqwest::Client, base_url: String, passwd: String, user: String) -> Self {
        //State GST codes
        let mut region_map = HashMap::new();
        region_map.insert(33, Arc::new(SpeedSearch::new("Home/Index", "Home/Search"))); // TN
        region_map.insert(
            29,
            Arc::new(SpeedSearch::new("HYD/HYDSearch", "HYD/Search")),
        ); // AP
        region_map.insert(
            37,
            Arc::new(SpeedSearch::new("BNG/BNGSearch", "BNG/Search")),
        ); // KA
        Self {
            client,
            base_url,
            passwd,
            user,
            region_map,
        }
    }
    fn get_region_path(&self, region_code: u8) -> SpeedResult<Arc<SpeedSearch>> {
        self.region_map
            .get(&region_code)
            .ok_or_else(|| SpeedError::InvalidRegion(region_code.to_string()))
            .cloned()
    }
}
