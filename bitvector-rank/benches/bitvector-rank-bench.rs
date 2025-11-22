use bitvector_rank::{Rank1, Rank64, Rank25664};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rand::{Rng, SeedableRng, rngs::StdRng};

fn bench_bitvector_construct(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bitvector construct (n = 10⁸)");

    let mut rng = StdRng::seed_from_u64(42);
    let TestCase { a, queries: _ } = TestCase::generate(&mut rng);

    group.bench_function("Rank1", |b| {
        b.iter(|| {
            let rank1: Rank1 = a.iter().copied().collect();
            black_box(rank1)
        });
    });

    group.bench_function("Rank64", |b| {
        b.iter(|| {
            let rank64: Rank64 = a.iter().copied().collect();
            black_box(rank64)
        });
    });

    group.bench_function("Rank25664", |b| {
        b.iter(|| {
            let rank25664: Rank25664 = a.iter().copied().collect();
            black_box(rank25664)
        });
    });

    group.finish();
}

fn bench_bitvector_rank(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bitvector rank (N = 10⁸, Q = 10⁶)");
    let mut rng = StdRng::seed_from_u64(42);
    let TestCase { a, queries } = TestCase::generate(&mut rng);

    let rank1: Rank1 = a.iter().copied().collect();
    group.bench_function("Rank1", |b| {
        b.iter(|| {
            for &query in &queries {
                match query {
                    Query::Rank { index } => {
                        let result = rank1.rank(index);
                        black_box(result);
                    }
                }
            }
        });
    });

    let rank64: Rank64 = a.iter().copied().collect();
    group.bench_function("Rank64", |b| {
        b.iter(|| {
            for &query in &queries {
                match query {
                    Query::Rank { index } => {
                        let result = rank64.rank(index);
                        black_box(result);
                    }
                }
            }
        });
    });

    let rank25664: Rank25664 = a.iter().copied().collect();
    group.bench_function("Rank25664", |b| {
        b.iter(|| {
            for &query in &queries {
                match query {
                    Query::Rank { index } => {
                        let result = rank25664.rank(index);
                        black_box(result);
                    }
                }
            }
        });
    });

    group.finish();
}

#[derive(Clone, Copy)]
enum Query {
    Rank { index: usize },
}

struct TestCase {
    a: Vec<bool>,
    queries: Vec<Query>,
}
impl TestCase {
    fn generate(rng: &mut impl Rng) -> Self {
        let n = 100_000_000;
        let a: Vec<_> = std::iter::repeat_with(|| rng.random_ratio(1, 2))
            .take(n)
            .collect();
        let q = 1_000_000;
        let queries = std::iter::repeat_with(|| {
            let index = rng.random_range(0..=n);
            Query::Rank { index }
        })
        .take(q)
        .collect();
        Self { a, queries }
    }
}

criterion_group!(benches, bench_bitvector_construct, bench_bitvector_rank);
criterion_main!(benches);
