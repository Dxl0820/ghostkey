mod crypto_tests;
mod key_tests;
mod model_tests;

#[cfg(test)]
mod tests {
    use crate::vault::Vault;
    use crate::models::CredentialType;

    #[test]
    fn test_vault_init() {
        let vault = Vault::init();
        assert!(vault.is_ok());
    }

    #[test]
    fn test_error_types() {
        use crate::error::Error;

        let err = Error::VaultNotInitialized;
        assert!(err.to_string().contains("not initialized"));

        let err = Error::VaultLocked;
        assert!(err.to_string().contains("locked"));

        let err = Error::CredentialNotFound("test".to_string());
        assert!(err.to_string().contains("test"));
    }
}
