#[cfg(test)]
mod tests {
    use crate::vault::crypto;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [0u8; 32];
        let data = b"Hello, World! This is a test secret.";

        let (encrypted, nonce) = crypto::encrypt(data, &key).unwrap();
        let decrypted = crypto::decrypt(&encrypted, &key, &nonce).unwrap();

        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_encrypt_produces_different_output() {
        let key = [0u8; 32];
        let data = b"Same data";

        let (encrypted1, nonce1) = crypto::encrypt(data, &key).unwrap();
        let (encrypted2, nonce2) = crypto::encrypt(data, &key).unwrap();

        // Nonces should be different (random)
        assert_ne!(nonce1, nonce2);
        // Encrypted data should be different (different nonces)
        assert_ne!(encrypted1, encrypted2);
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let key1 = [0u8; 32];
        let key2 = [1u8; 32];
        let data = b"Secret data";

        let (encrypted, nonce) = crypto::encrypt(data, &key1).unwrap();
        let result = crypto::decrypt(&encrypted, &key2, &nonce);

        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_wrong_nonce_fails() {
        let key = [0u8; 32];
        let data = b"Secret data";

        let (encrypted, nonce) = crypto::encrypt(data, &key).unwrap();
        let wrong_nonce = vec![0u8; 12];
        let result = crypto::decrypt(&encrypted, &key, &wrong_nonce);

        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_tampered_data_fails() {
        let key = [0u8; 32];
        let data = b"Secret data";

        let (mut encrypted, nonce) = crypto::encrypt(data, &key).unwrap();
        // Tamper with encrypted data
        if !encrypted.is_empty() {
            encrypted[0] ^= 0xFF;
        }
        let result = crypto::decrypt(&encrypted, &key, &nonce);

        assert!(result.is_err());
    }

    #[test]
    fn test_generate_nonce_length() {
        let nonce = crypto::generate_nonce();
        assert_eq!(nonce.len(), 12);
    }

    #[test]
    fn test_encrypt_with_nonce() {
        let key = [0u8; 32];
        let nonce = vec![1u8; 12];
        let data = b"Test data";

        let encrypted = crypto::encrypt_with_nonce(data, &key, &nonce).unwrap();
        let decrypted = crypto::decrypt(&encrypted, &key, &nonce).unwrap();

        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_empty_data() {
        let key = [0u8; 32];
        let data = b"";

        let (encrypted, nonce) = crypto::encrypt(data, &key).unwrap();
        let decrypted = crypto::decrypt(&encrypted, &key, &nonce).unwrap();

        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_large_data() {
        let key = [0u8; 32];
        let data = vec![0xAB; 1024 * 1024]; // 1MB

        let (encrypted, nonce) = crypto::encrypt(&data, &key).unwrap();
        let decrypted = crypto::decrypt(&encrypted, &key, &nonce).unwrap();

        assert_eq!(data, decrypted);
    }
}
