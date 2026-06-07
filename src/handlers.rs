use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use uuid::Uuid;
use serde_json::json;

use crate::{AppState, schema::GameSchema};

pub async fn create_game_handler(
    State(_data): State<Arc<AppState>>,
    Json(_body): Json<GameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4();

    let game_response = json!({
    "status": "success",
    "data": json!({
        "id": id
    })
    });

    Ok(Json(game_response))
}
