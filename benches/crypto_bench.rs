use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ghostkey::vault::{crypto, key};

fn bench_encrypt(c: &mut Criterion) {
    let key = [0u8; 32];
    let data = vec![0xAB; 1024]; // 1KB

    c.bench_function("encrypt_1kb", |b| {
        b.iter(|| {
            crypto::encrypt(black_box(&data), black_box(&key)).unwrap()
        })
    });
}

fn bench_decrypt(c: &mut Criterion) {
    let key = [0u8; 32];
    let data = vec![0xAB; 1024];
    let (encrypted, nonce) = crypto::encrypt(&data, &key).unwrap();

    c.bench_function("decrypt_1kb", |b| {
        b.iter(|| {
            crypto::decrypt(black_box(&encrypted), black_box(&key), black_box(&nonce)).unwrap()
        })
    });
}

fn bench_key_derivation(c: &mut Criterion) {
    let password = "test_password";
    let salt = vec![0u8; 32];

    c.bench_function("key_derivation", |b| {
        b.iter(|| {
            key::derive_key(black_box(password), black_box(&salt)).unwrap()
        })
    });
}

fn bench_encrypt_large(c: &mut Criterion) {
    let key = [0u8; 32];
    let data = vec![0xAB; 1024 * 1024]; // 1MB

    c.bench_function("encrypt_1mb", |b| {
        b.iter(|| {
            crypto::encrypt(black_box(&data), black_box(&key)).unwrap()
        })
    });
}

criterion_group!(
    benches,
    bench_encrypt,
    bench_decrypt,
    bench_key_derivation,
    bench_encrypt_large
);
criterion_main!(benches);
