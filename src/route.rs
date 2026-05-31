use std::sync::Arc;

use axum::{Router, routing::post, routing::get};

use crate::{AppState, handlers::create_game_handler, handlers::get_game_handler};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/games", post(create_game_handler))
        .route("/api/games/{id}", get(get_game_handler))
        .with_state(app_state)
}
