#[cfg(test)]
mod tests {
    use crate::models::{Credential, CredentialType, Metadata};

    #[test]
    fn test_credential_type_display() {
        assert_eq!(format!("{}", CredentialType::Password), "Password");
        assert_eq!(format!("{}", CredentialType::ApiKey), "API Key");
        assert_eq!(format!("{}", CredentialType::SshKey), "SSH Key");
        assert_eq!(format!("{}", CredentialType::Token), "Token");
        assert_eq!(format!("{}", CredentialType::EnvVar), "Environment Variable");
        assert_eq!(format!("{}", CredentialType::Custom("test".to_string())), "Custom(test)");
    }

    #[test]
    fn test_credential_new() {
        let metadata = Metadata::default();
        let cred = Credential::new(
            "test".to_string(),
            CredentialType::Password,
            vec![1, 2, 3],
            vec![4, 5, 6],
            metadata,
        );

        assert_eq!(cred.name, "test");
        assert!(matches!(cred.credential_type, CredentialType::Password));
        assert_eq!(cred.encrypted_secret, vec![1, 2, 3]);
        assert_eq!(cred.nonce, vec![4, 5, 6]);
    }

    #[test]
    fn test_credential_add_tag() {
        let metadata = Metadata::default();
        let mut cred = Credential::new(
            "test".to_string(),
            CredentialType::Password,
            vec![],
            vec![],
            metadata,
        );

        cred.add_tag("tag1".to_string());
        assert_eq!(cred.metadata.tags, vec!["tag1"]);

        cred.add_tag("tag2".to_string());
        assert_eq!(cred.metadata.tags, vec!["tag1", "tag2"]);

        // Adding duplicate tag should not change
        cred.add_tag("tag1".to_string());
        assert_eq!(cred.metadata.tags, vec!["tag1", "tag2"]);
    }

    #[test]
    fn test_credential_remove_tag() {
        let metadata = Metadata {
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            ..Metadata::default()
        };
        let mut cred = Credential::new(
            "test".to_string(),
            CredentialType::Password,
            vec![],
            vec![],
            metadata,
        );

        assert!(cred.remove_tag("tag1"));
        assert_eq!(cred.metadata.tags, vec!["tag2"]);

        assert!(!cred.remove_tag("nonexistent"));
        assert_eq!(cred.metadata.tags, vec!["tag2"]);
    }

    #[test]
    fn test_metadata_default() {
        let metadata = Metadata::default();

        assert!(metadata.description.is_none());
        assert!(metadata.tags.is_empty());
        assert!(metadata.username.is_none());
        assert!(metadata.url.is_none());
        assert!(metadata.custom_fields.is_empty());
    }
}
