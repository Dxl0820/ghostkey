use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use super::handlers;

pub fn create_router(state: handlers::AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(handlers::health))
        .route("/api/vault/status", get(handlers::vault_status))
        .route("/api/vault/unlock", post(handlers::unlock_vault))
        .route("/api/vault/lock", post(handlers::lock_vault))
        .route("/api/projects", get(handlers::list_projects).post(handlers::create_project))
        .route("/api/projects/:id", delete(handlers::delete_project))
        .route("/api/projects/:id/environments", get(handlers::list_environments).post(handlers::create_environment))
        .route("/api/secrets", get(handlers::list_secrets).post(handlers::create_secret))
        .route("/api/secrets/:id", put(handlers::update_secret).delete(handlers::delete_secret))
        .route("/api/secrets/:id/value", get(handlers::get_secret_value))
        .route("/api/export", get(handlers::export_secrets))
        .route("/api/import", post(handlers::import_secrets))
        .layer(cors)
        .with_state(state)
}
