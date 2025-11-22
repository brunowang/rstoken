use crate::handler::healthy_handler::healthy;
use crate::model::app_model::AppState;
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn create_route(app_state: Arc<AppState>) -> Router {
    let router = Router::new()
        .route("/health", post(healthy))
        .with_state(app_state.clone());
    router
}
