use bbst_comparison::avl_tree_by_box::AvlTreeByBox;
use bbst_comparison::avl_tree_with_parent::AvlTreeWithParent;
use bbst_comparison::bench_utils::{generate_initial_values, generate_queries, Query};
use bbst_comparison::rb_tree_with_parent::RbTreeWithParent;
use bbst_comparison::splay_tree_with_parent::SplayTreeWithParent;
use bbst_comparison::unbalanced_tree_with_parent::UnbalancedTreeWithParent;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_avl_tree_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bbst_comparison");

    group.bench_function("avl_with_parent", |b| {
        b.iter(|| {
            let initial_values = generate_initial_values();
            let queries = generate_queries();
            let mut tree: AvlTreeWithParent = initial_values.into_iter().collect();
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

    group.bench_function("avl_by_box", |b| {
        b.iter(|| {
            let initial_values = generate_initial_values();
            let queries = generate_queries();
            let mut tree: AvlTreeByBox = initial_values.into_iter().collect();
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

    group.bench_function("rb_with_parent", |b| {
        b.iter(|| {
            let initial_values = generate_initial_values();
            let queries = generate_queries();
            let mut tree: RbTreeWithParent = initial_values.into_iter().collect();
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

    group.bench_function("splay_with_parent", |b| {
        b.iter(|| {
            let initial_values = generate_initial_values();
            let queries = generate_queries();
            let mut tree: SplayTreeWithParent = initial_values.into_iter().collect();
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

    group.bench_function("unbalanced_with_parent", |b| {
        b.iter(|| {
            let initial_values = generate_initial_values();
            let queries = generate_queries();
            let mut tree: UnbalancedTreeWithParent = initial_values.into_iter().collect();
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

    group.finish();
}

criterion_group!(benches, bench_avl_tree_operations);
criterion_main!(benches);
