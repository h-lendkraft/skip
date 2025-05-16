use crate::speed::{
    MultipleAadharSearchRequest, MultipleMobileSearchRequest, MultipleNameDobSearchRequest,
};
use axum::{
    extract::{self, State},
    response::Json,
};
use serde_json::{json, Value};
use validator::{Validate, ValidateArgs};

use crate::error::SpeedResult;
use crate::speed::{SpeedState, SpeedUser};

#[axum::debug_handler]
pub async fn login(State(state): State<SpeedState>) -> SpeedResult<Json<Value>> {
    state.login().await?;
    Ok(Json(json!({ "status": "success" })))
}

#[axum::debug_handler]
pub async fn search_mobile(
    State(state): State<SpeedState>,
    extract::Json(numbers): extract::Json<MultipleMobileSearchRequest>,
) -> SpeedResult<Json<Vec<SpeedUser>>> {
    let valid_keys: Vec<u8> = state.region_map.keys().copied().collect();
    numbers.validate_with_args(&valid_keys)?;
    let persons = state.search_multiple_number(numbers).await?;
    Ok(Json(persons))
}
pub async fn search_aadhar(
    State(state): State<SpeedState>,
    extract::Json(numbers): extract::Json<MultipleAadharSearchRequest>,
) -> SpeedResult<Json<Vec<SpeedUser>>> {
    numbers.validate()?;
    let persons = state.search_multiple_aadhar(numbers).await?;
    Ok(Json(persons))
}
pub async fn search_name_dob(
    State(state): State<SpeedState>,
    extract::Json(namedobs): extract::Json<MultipleNameDobSearchRequest>,
) -> SpeedResult<Json<Vec<SpeedUser>>> {
    let valid_keys: Vec<u8> = state.region_map.keys().copied().collect();
    namedobs.validate_with_args(&valid_keys)?;
    let persons = state.search_multiple_name_dob(namedobs).await?;
    Ok(Json(persons))
}

use axum::routing::{get, post};
pub fn routes(state: SpeedState) -> axum::Router {
    axum::Router::new()
        .route("/speed/login", get(login))
        .route("/speed/search/mobile", post(search_mobile))
        .route("/speed/search/aadhar", post(search_aadhar))
        .route("/speed/search/name-dob", post(search_name_dob))
        .with_state(state)
}
