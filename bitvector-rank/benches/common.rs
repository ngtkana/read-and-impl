use bitvector_rank::test_utils::RankDataStructure;
use criterion::measurement::WallTime;
use criterion::{BenchmarkGroup, black_box};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Query {
    Rank { index: usize },
}

pub struct TestCase {
    pub a: Vec<bool>,
    pub queries: Vec<Query>,
}

impl TestCase {
    pub fn generate(rng: &mut impl Rng) -> Self {
        let n = 100_000_000;
        let a: Vec<_> = std::iter::repeat_with(|| rng.random_ratio(1, 2))
            .take(n)
            .collect();
        let q = 10_000_000;
        let queries = std::iter::repeat_with(|| {
            let index = rng.random_range(0..=n);
            Query::Rank { index }
        })
        .take(q)
        .collect();
        Self { a, queries }
    }
}

pub fn bench_construct<T: FromIterator<bool>>(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    data: &[bool],
) {
    group.bench_function(name, |b| {
        b.iter(|| {
            let rank_structure: T = data.iter().copied().collect();
            black_box(rank_structure)
        });
    });
}

pub fn bench_rank<T: RankDataStructure>(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    instance: &T,
    queries: &[Query],
) {
    group.bench_function(name, |b| {
        b.iter(|| {
            for &query in queries {
                match query {
                    Query::Rank { index } => {
                        let result = instance.rank(index);
                        black_box(result);
                    }
                }
            }
        });
    });
}
