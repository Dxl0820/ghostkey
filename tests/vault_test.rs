use std::fs;
use tempfile::TempDir;

// Helper to create a temporary ghostkey directory
fn setup_temp_dir() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ghostkey_dir = temp_dir.path().join(".ghostkey");
    fs::create_dir_all(&ghostkey_dir).expect("Failed to create ghostkey dir");
    temp_dir
}

#[test]
fn test_credential_serialization() {
    use ghostkey::models::{Credential, CredentialType, Metadata};
    use chrono::Utc;
    use uuid::Uuid;

    let metadata = Metadata {
        description: Some("Test description".to_string()),
        tags: vec!["test".to_string(), "example".to_string()],
        username: Some("testuser".to_string()),
        url: Some("https://example.com".to_string()),
        custom_fields: std::collections::HashMap::new(),
    };

    let credential = Credential {
        id: Uuid::new_v4(),
        name: "test-credential".to_string(),
        credential_type: CredentialType::ApiKey,
        encrypted_secret: vec![1, 2, 3, 4],
        nonce: vec![5, 6, 7, 8],
        metadata,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Serialize
    let json = serde_json::to_string(&credential).unwrap();
    assert!(!json.is_empty());

    // Deserialize
    let deserialized: Credential = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, credential.name);
    assert_eq!(deserialized.metadata.tags, credential.metadata.tags);
    assert_eq!(deserialized.metadata.username, credential.metadata.username);
}

#[test]
fn test_credential_type_serialization() {
    use ghostkey::models::CredentialType;

    let types = vec![
        CredentialType::Password,
        CredentialType::ApiKey,
        CredentialType::SshKey,
        CredentialType::Token,
        CredentialType::EnvVar,
        CredentialType::Custom("custom".to_string()),
    ];

    for cred_type in types {
        let json = serde_json::to_string(&cred_type).unwrap();
        let deserialized: CredentialType = serde_json::from_str(&json).unwrap();
        assert_eq!(format!("{}", cred_type), format!("{}", deserialized));
    }
}

#[test]
fn test_metadata_serialization() {
    use ghostkey::models::Metadata;

    let mut custom_fields = std::collections::HashMap::new();
    custom_fields.insert("key1".to_string(), "value1".to_string());
    custom_fields.insert("key2".to_string(), "value2".to_string());

    let metadata = Metadata {
        description: Some("Test".to_string()),
        tags: vec!["tag1".to_string(), "tag2".to_string()],
        username: Some("user".to_string()),
        url: Some("https://example.com".to_string()),
        custom_fields,
    };

    let json = serde_json::to_string(&metadata).unwrap();
    let deserialized: Metadata = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.description, metadata.description);
    assert_eq!(deserialized.tags, metadata.tags);
    assert_eq!(deserialized.username, metadata.username);
    assert_eq!(deserialized.url, metadata.url);
    assert_eq!(deserialized.custom_fields.len(), 2);
}

#[test]
fn test_csv_parsing() {
    // Test CSV escape function
    fn escape_csv(s: &str) -> String {
        if s.contains(',') || s.contains('"') || s.contains('\n') {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    }

    assert_eq!(escape_csv("simple"), "simple");
    assert_eq!(escape_csv("with,comma"), "\"with,comma\"");
    assert_eq!(escape_csv("with\"quote"), "\"with\"\"quote\"");
    assert_eq!(escape_csv("with\nnewline"), "\"with\nnewline\"");
}

#[test]
fn test_shell_escape() {
    fn escape_shell(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('$', "\\$")
            .replace('`', "\\`")
    }

    assert_eq!(escape_shell("simple"), "simple");
    assert_eq!(escape_shell("with\"quote"), "with\\\"quote");
    assert_eq!(escape_shell("with$dollar"), "with\\$dollar");
    assert_eq!(escape_shell("with`backtick"), "with\\`backtick");
    assert_eq!(escape_shell("with\\backslash"), "with\\\\backslash");
}
