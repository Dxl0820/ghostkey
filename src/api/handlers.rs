use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use super::types::*;
use crate::vault::Vault;
use crate::error::Error;

pub type AppState = Arc<tokio::sync::Mutex<Vault>>;

// --- Error handling ---

impl crate::error::Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::VaultNotInitialized => StatusCode::BAD_REQUEST,
            Error::VaultAlreadyInitialized(_) => StatusCode::CONFLICT,
            Error::VaultLocked => StatusCode::UNAUTHORIZED,
            Error::InvalidPassword => StatusCode::UNAUTHORIZED,
            Error::SecretNotFound(_) => StatusCode::NOT_FOUND,
            Error::SecretAlreadyExists(_) => StatusCode::CONFLICT,
            Error::ProjectNotFound(_) => StatusCode::NOT_FOUND,
            Error::EnvironmentNotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

fn error_response(err: Error) -> (StatusCode, Json<ErrorResponse>) {
    let status = err.status_code();
    (status, Json(ErrorResponse { error: err.to_string() }))
}

// --- Health ---

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

// --- Vault ---

pub async fn vault_status(State(state): State<AppState>) -> Json<VaultStatusResponse> {
    let vault = state.lock().await;
    Json(VaultStatusResponse {
        initialized: true,
        locked: vault.is_locked(),
    })
}

pub async fn unlock_vault(
    State(state): State<AppState>,
    Json(req): Json<UnlockRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut vault = state.lock().await;
    vault.unlock_with_password(&req.password).map_err(error_response)?;
    Ok(Json(MessageResponse {
        message: "Vault unlocked".to_string(),
    }))
}

pub async fn lock_vault(
    State(state): State<AppState>,
) -> Json<MessageResponse> {
    let mut vault = state.lock().await;
    vault.lock();
    Json(MessageResponse {
        message: "Vault locked".to_string(),
    })
}

// --- Projects ---

pub async fn list_projects(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProjectResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let vault = state.lock().await;
    let projects = vault.list_projects().map_err(error_response)?;

    let responses: Vec<ProjectResponse> = projects.iter().map(|p| {
        let env_count = vault.list_environments(p.id).map(|e| e.len()).unwrap_or(0);
        let secret_count = vault.list_environments(p.id)
            .map(|envs| envs.iter().map(|e| {
                vault.list_secrets(e.id).map(|s| s.len()).unwrap_or(0)
            }).sum())
            .unwrap_or(0);

        ProjectResponse {
            id: p.id.to_string(),
            name: p.name.clone(),
            description: p.description.clone(),
            environment_count: env_count,
            secret_count,
            created_at: p.created_at.to_rfc3339(),
            updated_at: p.updated_at.to_rfc3339(),
        }
    }).collect();

    Ok(Json(responses))
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(req): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<ProjectResponse>), (StatusCode, Json<ErrorResponse>)> {
    let mut vault = state.lock().await;
    let project = vault.create_project(req.name, req.description).map_err(error_response)?;

    Ok((StatusCode::CREATED, Json(ProjectResponse {
        id: project.id.to_string(),
        name: project.name,
        description: project.description,
        environment_count: 0,
        secret_count: 0,
        created_at: project.created_at.to_rfc3339(),
        updated_at: project.updated_at.to_rfc3339(),
    })))
}

pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    let uuid = Uuid::parse_str(&id).map_err(|_| {
        error_response(Error::ProjectNotFound(id))
    })?;

    let mut vault = state.lock().await;
    vault.delete_project(uuid).map_err(error_response)?;

    Ok(Json(MessageResponse {
        message: "Project deleted".to_string(),
    }))
}

// --- Environments ---

pub async fn list_environments(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<EnvironmentResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let uuid = Uuid::parse_str(&project_id).map_err(|_| {
        error_response(Error::ProjectNotFound(project_id))
    })?;

    let vault = state.lock().await;
    let environments = vault.list_environments(uuid).map_err(error_response)?;

    let responses: Vec<EnvironmentResponse> = environments.iter().map(|e| {
        let secret_count = vault.list_secrets(e.id).map(|s| s.len()).unwrap_or(0);
        EnvironmentResponse {
            id: e.id.to_string(),
            project_id: e.project_id.to_string(),
            name: e.name.clone(),
            secret_count,
            created_at: e.created_at.to_rfc3339(),
        }
    }).collect();

    Ok(Json(responses))
}

pub async fn create_environment(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(req): Json<CreateEnvironmentRequest>,
) -> Result<(StatusCode, Json<EnvironmentResponse>), (StatusCode, Json<ErrorResponse>)> {
    let uuid = Uuid::parse_str(&project_id).map_err(|_| {
        error_response(Error::ProjectNotFound(project_id))
    })?;

    let mut vault = state.lock().await;
    let env = vault.create_environment(uuid, req.name).map_err(error_response)?;

    Ok((StatusCode::CREATED, Json(EnvironmentResponse {
        id: env.id.to_string(),
        project_id: env.project_id.to_string(),
        name: env.name,
        secret_count: 0,
        created_at: env.created_at.to_rfc3339(),
    })))
}

// --- Secrets ---

#[derive(Deserialize)]
pub struct ListSecretsQuery {
    pub environment_id: String,
}

pub async fn list_secrets(
    State(state): State<AppState>,
    Query(query): Query<ListSecretsQuery>,
) -> Result<Json<Vec<SecretResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let env_id = Uuid::parse_str(&query.environment_id).map_err(|_| {
        error_response(Error::EnvironmentNotFound(query.environment_id))
    })?;

    let vault = state.lock().await;
    let secrets = vault.list_secrets(env_id).map_err(error_response)?;

    let responses: Vec<SecretResponse> = secrets.iter().map(|s| {
        SecretResponse {
            id: s.id.to_string(),
            environment_id: s.environment_id.to_string(),
            key: s.key.clone(),
            masked: true,
            description: s.description.clone(),
            created_at: s.created_at.to_rfc3339(),
            updated_at: s.updated_at.to_rfc3339(),
        }
    }).collect();

    Ok(Json(responses))
}

pub async fn create_secret(
    State(state): State<AppState>,
    Json(req): Json<CreateSecretRequest>,
) -> Result<(StatusCode, Json<MessageResponse>), (StatusCode, Json<ErrorResponse>)> {
    let mut vault = state.lock().await;
    vault.create_secret(req.environment_id, req.key, req.value, req.description)
        .map_err(error_response)?;

    Ok((StatusCode::CREATED, Json(MessageResponse {
        message: "Secret created".to_string(),
    })))
}

pub async fn update_secret(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateSecretRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    let uuid = Uuid::parse_str(&id).map_err(|_| {
        error_response(Error::SecretNotFound(id))
    })?;

    let mut vault = state.lock().await;
    vault.update_secret(uuid, req.value, req.description.map(Some))
        .map_err(error_response)?;

    Ok(Json(MessageResponse {
        message: "Secret updated".to_string(),
    }))
}

pub async fn delete_secret(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    let uuid = Uuid::parse_str(&id).map_err(|_| {
        error_response(Error::SecretNotFound(id))
    })?;

    let mut vault = state.lock().await;
    vault.delete_secret(uuid).map_err(error_response)?;

    Ok(Json(MessageResponse {
        message: "Secret deleted".to_string(),
    }))
}

pub async fn get_secret_value(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<SecretValueResponse>, (StatusCode, Json<ErrorResponse>)> {
    let uuid = Uuid::parse_str(&id).map_err(|_| {
        error_response(Error::SecretNotFound(id))
    })?;

    let vault = state.lock().await;
    let secret = vault.get_secret(uuid).map_err(error_response)?;
    let value = vault.get_secret_value(uuid).map_err(error_response)?;

    Ok(Json(SecretValueResponse {
        id: secret.id.to_string(),
        key: secret.key,
        value,
    }))
}

// --- Export ---

#[derive(Deserialize)]
pub struct ExportQuery {
    pub environment_id: String,
    pub format: Option<String>,
}

pub async fn export_secrets(
    State(state): State<AppState>,
    Query(query): Query<ExportQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let env_id = Uuid::parse_str(&query.environment_id).map_err(|_| {
        error_response(Error::EnvironmentNotFound(query.environment_id))
    })?;

    let vault = state.lock().await;
    let secrets = vault.get_all_secrets_decrypted(env_id).map_err(error_response)?;

    let items: Vec<serde_json::Value> = secrets.iter().map(|(key, value)| {
        serde_json::json!({
            "key": key,
            "value": value,
        })
    }).collect();

    Ok(Json(serde_json::json!({
        "secrets": items,
    })))
}

// --- Import ---

pub async fn import_secrets(
    State(state): State<AppState>,
    Json(req): Json<ImportRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut vault = state.lock().await;

    let count = match req.format.as_str() {
        "env" => {
            let mut n = 0;
            for line in req.data.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim();
                    if !key.is_empty() && !value.is_empty() {
                        vault.import_secret_from_env(req.environment_id, key, value)
                            .map_err(error_response)?;
                        n += 1;
                    }
                }
            }
            n
        }
        _ => {
            return Err(error_response(Error::ConfigError(
                "Only 'env' format is supported for import via API".to_string()
            )));
        }
    };

    Ok(Json(MessageResponse {
        message: format!("Imported {} secret(s)", count),
    }))
}
