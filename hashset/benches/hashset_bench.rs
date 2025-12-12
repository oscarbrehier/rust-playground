use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use hashset::HashSet;

fn bench_insert(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(123);
    let data: Vec<u64> = (0..10_000).map(|_| rng.r#gen()).collect();

    c.bench_function("my HashSet insert", |b| {
        b.iter(|| {
            let mut set = HashSet::new();
            for &x in &data {
                set.insert(x);
            }
        })
    });

    c.bench_function("std HashSet insert", |b| {
        use std::collections::HashSet as StdHashSet;
        b.iter(|| {
            let mut set = StdHashSet::new();
            for &x in &data {
                set.insert(x);
            }
        })
    });
}

fn bench_contains(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(456);
    let data: Vec<u64> = (0..10_000).map(|_| rng.r#gen()).collect();

    let mut my_set = HashSet::new();
    let mut std_set = std::collections::HashSet::new();

    for &x in &data {
        my_set.insert(x);
        std_set.insert(x);
    }

    c.bench_function("my HashSet contains", |b| {
        b.iter(|| black_box(my_set.contains(black_box(&500))))
    });

    c.bench_function("std HashSet contains", |b| {
        b.iter(|| black_box(std_set.contains(black_box(&500))))
    });
}

criterion_group!(benches, bench_insert, bench_contains);
criterion_main!(benches);
