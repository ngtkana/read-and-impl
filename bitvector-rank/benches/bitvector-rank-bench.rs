use bitvector_rank::Rank25664;
use criterion::{Criterion, criterion_group, criterion_main};
use rand::{Rng, SeedableRng, rngs::StdRng};

fn bench_bitvector_rank(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitvector-rank (n = 10⁷)");

    group.bench_function("{ s: 256, w: 64 }", |b| {
        b.iter(|| {
            let mut rng = StdRng::seed_from_u64(42);
            let TestCase { a, queries } = TestCase::generate(&mut rng);
            let bvec: Rank25664 = a.iter().copied().collect();
            for &query in &queries {
                match query {
                    Query::Rank { index } => {
                        let _ = bvec.rank(index);
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
        let n = 10_000_000;
        let a: Vec<_> = std::iter::repeat_with(|| rng.random_ratio(1, 2))
            .take(n)
            .collect();
        let q = 100;
        let queries = std::iter::repeat_with(|| {
            let index = rng.random_range(0..=n);
            Query::Rank { index }
        })
        .take(q)
        .collect();
        Self { a, queries }
    }
}

criterion_group!(benches, bench_bitvector_rank);
criterion_main!(benches);
