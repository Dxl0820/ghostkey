use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialType {
    Password,
    ApiKey,
    SshKey,
    Token,
    EnvVar,
    Custom(String),
}

impl fmt::Display for CredentialType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CredentialType::Password => write!(f, "Password"),
            CredentialType::ApiKey => write!(f, "API Key"),
            CredentialType::SshKey => write!(f, "SSH Key"),
            CredentialType::Token => write!(f, "Token"),
            CredentialType::EnvVar => write!(f, "Environment Variable"),
            CredentialType::Custom(name) => write!(f, "Custom({})", name),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub username: Option<String>,
    pub url: Option<String>,
    pub custom_fields: std::collections::HashMap<String, String>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            description: None,
            tags: Vec::new(),
            username: None,
            url: None,
            custom_fields: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: Uuid,
    pub name: String,
    pub credential_type: CredentialType,
    #[serde(with = "base64_bytes")]
    pub encrypted_secret: Vec<u8>,
    #[serde(with = "base64_bytes")]
    pub nonce: Vec<u8>,
    pub metadata: Metadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Credential {
    pub fn new(
        name: String,
        credential_type: CredentialType,
        encrypted_secret: Vec<u8>,
        nonce: Vec<u8>,
        metadata: Metadata,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            credential_type,
            encrypted_secret,
            nonce,
            metadata,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_secret(&mut self, encrypted_secret: Vec<u8>, nonce: Vec<u8>) {
        self.encrypted_secret = encrypted_secret;
        self.nonce = nonce;
        self.updated_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.metadata.tags.contains(&tag) {
            self.metadata.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_tag(&mut self, tag: &str) -> bool {
        let len_before = self.metadata.tags.len();
        self.metadata.tags.retain(|t| t != tag);
        if self.metadata.tags.len() < len_before {
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }
}

mod base64_bytes {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use base64::{Engine as _, engine::general_purpose};

    pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = general_purpose::STANDARD.encode(bytes);
        encoded.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;
        general_purpose::STANDARD.decode(&encoded).map_err(serde::de::Error::custom)
    }
}
