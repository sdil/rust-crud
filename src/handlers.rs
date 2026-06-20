use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
// use tokio::sync::broadcast::error;
use uuid::Uuid;
use sea_orm::{ActiveModelTrait, Set};

use crate::{entity::games, AppState, schema::GameSchema};

pub async fn create_game_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<GameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4();
    // let now = chrono::Utc::now().naive_utc();

    let _ = games::ActiveModel {
        id: Set(id.to_string()),
        name: Set(body.name),
        plays: Set(body.plays),
        creator: Set(body.creator),
        ..Default::default()
    }
    .insert(&state.db).await.unwrap();

    let game_response = json!({
    "status": "success"});

    Ok(Json(game_response))
}
