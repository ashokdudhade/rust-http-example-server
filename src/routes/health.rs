use crate::handlers::health;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(health::health_check))
}