use criterion::{criterion_group, criterion_main, Criterion};
use hash_fns::{hash_random_blake3_256, hash_random_sha2_256, hash_random_sha3_256};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Blake3 hash", |b| b.iter(|| hash_random_blake3_256()));
    c.bench_function("SHA3-256 hash", |b| b.iter(|| hash_random_sha3_256()));
    c.bench_function("SHA-256 hash", |b| b.iter(|| hash_random_sha2_256()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
