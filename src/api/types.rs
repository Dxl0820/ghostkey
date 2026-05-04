use serde::{Deserialize, Serialize};
use uuid::Uuid;

// --- Request types ---

#[derive(Deserialize)]
pub struct UnlockRequest {
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateEnvironmentRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateSecretRequest {
    pub environment_id: Uuid,
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateSecretRequest {
    pub value: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct ImportRequest {
    pub format: String,
    pub data: String,
    pub environment_id: Uuid,
}

// --- Response types ---

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct VaultStatusResponse {
    pub initialized: bool,
    pub locked: bool,
}

#[derive(Serialize)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub environment_count: usize,
    pub secret_count: usize,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct EnvironmentResponse {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub secret_count: usize,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct SecretResponse {
    pub id: String,
    pub environment_id: String,
    pub key: String,
    pub masked: bool,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct SecretValueResponse {
    pub id: String,
    pub key: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
