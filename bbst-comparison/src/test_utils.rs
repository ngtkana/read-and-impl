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
