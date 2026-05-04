use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl Environment {
    pub fn new(project_id: Uuid, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            project_id,
            name,
            created_at: Utc::now(),
        }
    }
}
