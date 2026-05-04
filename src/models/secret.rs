use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub id: Uuid,
    pub environment_id: Uuid,
    pub key: String,
    #[serde(with = "base64_bytes")]
    pub encrypted_value: Vec<u8>,
    #[serde(with = "base64_bytes")]
    pub nonce: Vec<u8>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Secret {
    pub fn new(
        environment_id: Uuid,
        key: String,
        encrypted_value: Vec<u8>,
        nonce: Vec<u8>,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            environment_id,
            key,
            encrypted_value,
            nonce,
            description,
            created_at: now,
            updated_at: now,
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
