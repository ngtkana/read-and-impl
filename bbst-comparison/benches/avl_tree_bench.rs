use bbst_comparison::avl_tree_with_parent::AvlTreeWithParent;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, Clone, Copy)]
enum Query {
    Insert { index: usize, value: i32 },
    Remove { index: usize },
}

fn generate_queries() -> (AvlTreeWithParent, Vec<Query>) {
    let mut rng = StdRng::seed_from_u64(42);
    let n_initial = 200_000;
    let len_max = 200_000;
    let q = 200_000;
    let value_lim = 1_000_000_000;

    let initial_values: Vec<i32> = (0..n_initial)
        .map(|_| rng.random_range(0..value_lim))
        .collect();
    let tree: AvlTreeWithParent = initial_values.into_iter().collect();

    let mut n = n_initial;
    let queries: Vec<Query> = std::iter::repeat_with(|| {
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
    .collect();

    (tree, queries)
}

fn bench_avl_tree_operations(c: &mut Criterion) {
    c.bench_function("avl_tree_insert_remove_200k", |b| {
        b.iter(|| {
            let (mut tree, queries) = generate_queries();
            for &query in &queries {
                match query {
                    Query::Insert { index, value } => {
                        tree.insert(black_box(index), black_box(value));
                    }
                    Query::Remove { index } => {
                        tree.remove(black_box(index));
                    }
                }
            }
        });
    });
}

criterion_group!(benches, bench_avl_tree_operations);
criterion_main!(benches);
