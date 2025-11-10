use std::ptr;

use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug, Clone, Copy)]
pub enum Query {
    Insert { index: usize, value: i32 },
    Remove { index: usize },
}

pub fn generate_benchmark_queries<T>() -> (T, Vec<Query>)
where
    T: FromIterator<i32>,
{
    let mut rng = StdRng::seed_from_u64(42);
    let n_initial = 200_000;
    let len_max = 200_000;
    let q = 200_000;
    let value_lim = 1_000_000_000;

    let initial_values: Vec<i32> = (0..n_initial)
        .map(|_| rng.random_range(0..value_lim))
        .collect();
    let tree: T = initial_values.into_iter().collect();

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

pub trait TreeNode {
    fn left(&self) -> Option<&Self>;
    fn right(&self) -> Option<&Self>;
    fn value(&self) -> i32;
}

pub trait HasRoot {
    type Node: TreeNode;
    fn root(&self) -> Option<&Self::Node>;
}

#[allow(clippy::len_without_is_empty)]
pub trait BenchmarkableTree: FromIterator<i32> {
    fn len(&self) -> usize;
    fn height(&self) -> u8;
    fn insert(&mut self, index: usize, value: i32);
    fn remove(&mut self, index: usize) -> i32;
}

pub fn analyze_tree_stats<T: BenchmarkableTree>() {
    pub const PHI: f64 = 1.618_033_988_749_895_f64;

    let (mut tree, queries) = generate_benchmark_queries::<T>();

    println!("Initial state:");
    println!("  len={}, height={}", tree.len(), tree.height());
    println!(
        "  Theoretical optimal height: ~{:.1}",
        (tree.len() as f64).log2()
    );
    println!();

    let mut min_len = tree.len();
    let mut max_len = tree.len();
    let mut min_height = tree.height();
    let mut max_height = tree.height();

    for (i, &query) in queries.iter().enumerate() {
        match query {
            Query::Insert { index, value } => tree.insert(index, value),
            Query::Remove { index } => {
                tree.remove(index);
            }
        }

        let len = tree.len();
        let height = tree.height();
        min_len = min_len.min(len);
        max_len = max_len.max(len);
        min_height = min_height.min(height);
        max_height = max_height.max(height);

        if (i + 1) % 20_000 == 0 {
            let optimal_height = (len as f64).log2();
            let limit_height = (len as f64).ln() / PHI.ln();
            println!(
                "After {:6} queries: len={:6}, height={:2} (optimal: ~{:.1}, limit: ~{:.1}, ratio: {:.2})",
                i + 1,
                len,
                height,
                optimal_height,
                limit_height,
                height as f64 / optimal_height
            );
        }
    }

    println!();
    println!("Final state:");
    println!("  len={}, height={}", tree.len(), tree.height());
    println!(
        "  Theoretical optimal height: ~{:.1}",
        (tree.len() as f64).log2()
    );
    println!();
    println!("Statistics:");
    println!("  Length range: {min_len} - {max_len}");
    println!("  Height range: {min_height} - {max_height}");
    println!(
        "  Max height / optimal ratio: {:.2}",
        max_height as f64 / (max_len as f64).log2()
    );
}

pub fn collect<T: HasRoot>(tree: &T) -> Vec<i32> {
    fn collect_recurse<N: TreeNode>(node: Option<&N>, out: &mut Vec<i32>) {
        let Some(node) = node else { return };
        collect_recurse(node.left(), out);
        out.push(node.value());
        collect_recurse(node.right(), out);
    }
    let mut out = vec![];
    collect_recurse(tree.root(), &mut out);
    out
}

pub trait Validatable: TreeNode {
    const HAS_PARENT_POINTER: bool;
    fn validate_balance(&self) -> bool;
    fn parent(&self) -> Option<&Self> {
        None
    }
}

pub fn validate<T: HasRoot>(tree: &T)
where
    T::Node: Validatable,
{
    fn validate_recurse<N: Validatable>(node: &N, parent: Option<&N>) {
        if N::HAS_PARENT_POINTER {
            let expected_parent = node.parent();
            match (expected_parent, parent) {
                (Some(exp), Some(par)) => assert!(ptr::eq(exp, par)),
                (None, None) => {}
                _ => panic!("Parent pointer mismatch"),
            }
        }

        assert!(node.validate_balance());

        if let Some(l) = node.left() {
            validate_recurse(l, Some(node));
        }
        if let Some(r) = node.right() {
            validate_recurse(r, Some(node));
        }
    }

    if let Some(root) = tree.root() {
        if T::Node::HAS_PARENT_POINTER {
            assert!(root.parent().is_none());
        }
        validate_recurse(root, None);
    }
}
