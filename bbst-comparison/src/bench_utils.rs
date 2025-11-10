use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, Clone, Copy)]
pub enum Query {
    Insert { index: usize, value: i32 },
    Remove { index: usize },
}

pub fn generate_queries() -> Vec<Query> {
    let mut rng = StdRng::seed_from_u64(42);
    let n_initial = 200_000;
    let len_max = 200_000;
    let q = 200_000;
    let value_lim = 1_000_000_000;

    let mut n = n_initial;
    std::iter::repeat_with(|| {
        if rng.random_ratio(n as u32, len_max) {
            let index = rng.random_range(0..n);
            n -= 1;
            Query::Remove { index }
        } else {
            let index = rng.random_range(0..=n);
            let value = rng.random_range(0..value_lim);
            n += 1;
            Query::Insert { index, value }
        }
    })
    .take(q)
    .collect()
}

pub fn generate_initial_values() -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(42);
    let n_initial = 200_000;
    let value_lim = 1_000_000_000;
    (0..n_initial)
        .map(|_| rng.random_range(0..value_lim))
        .collect()
}
