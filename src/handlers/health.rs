use crate::domain::responses::{ApiResponse, HealthResponse};
use axum::response::Json;

pub async fn health_check() -> Json<ApiResponse<HealthResponse>> {
    Json(ApiResponse::new(HealthResponse::healthy()))
}

pub async fn root() -> &'static str {
    "🦀 Welcome to the Axum Enterprise Server! Try /api/v1/health for health check."
}