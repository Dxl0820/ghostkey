#[cfg(test)]
mod tests {
    use crate::vault::key;

    #[test]
    fn test_derive_key_deterministic() {
        let password = "test_password";
        let salt = vec![1u8; 32];

        let key1 = key::derive_key(password, &salt).unwrap();
        let key2 = key::derive_key(password, &salt).unwrap();

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_derive_key_different_password() {
        let salt = vec![1u8; 32];

        let key1 = key::derive_key("password1", &salt).unwrap();
        let key2 = key::derive_key("password2", &salt).unwrap();

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_derive_key_different_salt() {
        let password = "test_password";
        let salt1 = vec![1u8; 32];
        let salt2 = vec![2u8; 32];

        let key1 = key::derive_key(password, &salt1).unwrap();
        let key2 = key::derive_key(password, &salt2).unwrap();

        assert_ne!(key1, key2);
    }

    #[test]
    fn test_generate_salt_randomness() {
        let salt1 = key::generate_salt();
        let salt2 = key::generate_salt();

        assert_eq!(salt1.len(), 32);
        assert_eq!(salt2.len(), 32);
        assert_ne!(salt1, salt2);
    }

    #[test]
    fn test_derive_key_length() {
        let password = "test";
        let salt = vec![0u8; 32];

        let key = key::derive_key(password, &salt).unwrap();

        assert_eq!(key.len(), 32);
    }
}
