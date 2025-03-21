use crate::speed::MultipleMobileSearchRequest;
use axum::{
    extract::{self, State},
    response::Json,
};
use serde_json::{json, Value};
use validator::Validate;

use crate::error::SpeedResult;
use crate::speed::{SpeedState, SpeedUser};

#[axum::debug_handler]
pub async fn login(State(state): State<SpeedState>) -> SpeedResult<Json<Value>> {
    state.login().await?;
    Ok(Json(json!({ "status": "success" })))
}

pub async fn search(
    State(state): State<SpeedState>,
    extract::Json(numbers): extract::Json<MultipleMobileSearchRequest>,
) -> SpeedResult<Json<Vec<SpeedUser>>> {
    numbers.validate()?;
    let persons = state.search_multiple(numbers).await?;
    Ok(Json(persons))
}
use axum::routing::{get, post};
pub fn routes(state: SpeedState) -> axum::Router {
    axum::Router::new()
        .route("/speed/login", get(login))
        .route("/speed/search", post(search))
        .with_state(state)
}
