use aes_gcm_keygen::{ring_aes_gcm_keygen, rust_crypto_aes_gcm_keygen};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Ring's AES-GCM keygen", |b| {
        b.iter(ring_aes_gcm_keygen)
    });
    c.bench_function("RustCrypto's AES-GCM keygen", |b| {
        b.iter(rust_crypto_aes_gcm_keygen)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
