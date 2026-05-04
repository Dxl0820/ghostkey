use crate::error::Result;
use crate::models::{Credential, Environment, Project, Secret};
use super::storage::VaultData;

pub fn migrate_v1_to_v2(credentials: Vec<Credential>) -> Result<VaultData> {
    let mut project = Project::new("Default".to_string(), Some("Migrated from v1".to_string()));
    project.created_at = credentials.first().map(|c| c.created_at).unwrap_or_else(chrono::Utc::now);

    let environment = Environment::new(project.id, "default".to_string());

    let secrets: Vec<Secret> = credentials.into_iter().map(|cred| {
        let key = cred.name.to_uppercase().replace('-', "_");
        let mut secret = Secret::new(
            environment.id,
            key,
            cred.encrypted_secret,
            cred.nonce,
            cred.metadata.description,
        );
        secret.created_at = cred.created_at;
        secret.updated_at = cred.updated_at;
        secret
    }).collect();

    Ok(VaultData {
        version: 2,
        projects: vec![project],
        environments: vec![environment],
        secrets,
    })
}
