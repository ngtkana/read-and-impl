#![allow(clippy::unnecessary_box_returns)]
use std::cmp::Ordering;

pub struct AvlTreeByBox {
    root: Option<Box<Node>>,
}
impl Default for AvlTreeByBox {
    fn default() -> Self {
        Self::new()
    }
}

impl AvlTreeByBox {
    pub fn new() -> Self {
        Self { root: None }
    }
    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |r| r.len)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn height(&self) -> u8 {
        self.root.as_ref().map_or(0, |r| r.ht)
    }
    pub fn insert(&mut self, index: usize, value: i32) {
        let c = Box::new(Node {
            left: None,
            right: None,
            value,
            len: 1,
            ht: 1,
        });
        let (l, r) = split2(self.root.take(), index);
        self.root = Some(merge3(l, c, r));
    }
    pub fn remove(&mut self, index: usize) -> i32 {
        let (l, c, r) = split3(self.root.take().unwrap(), index);
        self.root = merge2(l, r);
        c.value
    }
}

impl FromIterator<i32> for AvlTreeByBox {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        fn from_iter_recurse(values: &[i32]) -> Option<Box<Node>> {
            let n = values.len();
            if n == 0 {
                return None;
            }
            let left = from_iter_recurse(&values[..n / 2]);
            let right = from_iter_recurse(&values[n / 2 + 1..]);
            let mut c = Box::new(Node {
                left,
                right,
                value: values[n / 2],
                ht: 0,
                len: 1,
            });
            c.update();
            Some(c)
        }
        let values: Vec<_> = iter.into_iter().collect();
        let root = from_iter_recurse(&values);
        Self { root }
    }
}

pub struct Node {
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    value: i32,
    len: usize,
    ht: u8,
}

impl Node {
    fn update(&mut self) {
        self.len = 1;
        self.ht = 1;
        if let Some(l) = self.left.as_ref() {
            self.len += l.len;
            self.ht = self.ht.max(l.ht + 1);
        }
        if let Some(r) = self.right.as_ref() {
            self.len += r.len;
            self.ht = self.ht.max(r.ht + 1);
        }
    }
}

fn split2(x: Option<Box<Node>>, index: usize) -> (Option<Box<Node>>, Option<Box<Node>>) {
    if index == x.as_ref().map_or(0, |x| x.len) {
        return (x, None);
    }
    let (l, c, r) = split3(x.unwrap(), index);
    (l, merge2(Some(c), r))
}

fn split3(mut x: Box<Node>, index: usize) -> (Option<Box<Node>>, Box<Node>, Option<Box<Node>>) {
    let llen = x.left.as_ref().map_or(0, |x| x.len);
    let l = x.left.take();
    let r = x.right.take();
    x.update();
    match index.cmp(&llen) {
        Ordering::Less => {
            let (l0, l1, l2) = split3(l.unwrap(), index);
            (l0, l1, Some(merge3(l2, x, r)))
        }
        Ordering::Equal => (l, x, r),
        Ordering::Greater => {
            let (r0, r1, r2) = split3(r.unwrap(), index - llen - 1);
            (Some(merge3(l, x, r0)), r1, r2)
        }
    }
}

fn merge2(l: Option<Box<Node>>, r: Option<Box<Node>>) -> Option<Box<Node>> {
    let Some(r) = r else { return l };
    let (_, c, r) = split3(r, 0);
    Some(merge3(l, c, r))
}

fn merge3(l: Option<Box<Node>>, mut c: Box<Node>, r: Option<Box<Node>>) -> Box<Node> {
    match ht(l.as_deref()).cmp(&ht(r.as_deref())) {
        Ordering::Less => {
            let mut r = r.unwrap();
            r.left = Some(merge3(l, c, r.left.take()));
            balance(r)
        }
        Ordering::Equal => {
            c.left = l;
            c.right = r;
            c.update();
            c
        }
        Ordering::Greater => {
            let mut l = l.unwrap();
            l.right = Some(merge3(l.right.take(), c, r));
            balance(l)
        }
    }
}

fn balance(mut x: Box<Node>) -> Box<Node> {
    match ht(x.left.as_deref()) as i8 - ht(x.right.as_deref()) as i8 {
        -2 => {
            x.right = x.right.map(|r| {
                if ht(r.left.as_deref()) > ht(r.right.as_deref()) {
                    rotate_right(r)
                } else {
                    r
                }
            });
            x = rotate_left(x);
        }
        -1..=1 => x.update(),
        2 => {
            x.left = x.left.map(|l| {
                if ht(l.left.as_deref()) < ht(l.right.as_deref()) {
                    rotate_left(l)
                } else {
                    l
                }
            });
            x = rotate_right(x);
        }
        _ => unreachable!(),
    }
    x
}

fn ht(x: Option<&Node>) -> u8 {
    x.map_or(0, |x| x.ht)
}

fn rotate_left(mut x: Box<Node>) -> Box<Node> {
    let mut y = x.right.take().unwrap();
    x.right = y.left.take();
    x.update();
    y.left = Some(x);
    y.update();
    y
}

fn rotate_right(mut x: Box<Node>) -> Box<Node> {
    let mut y = x.left.take().unwrap();
    x.left = y.right.take();
    x.update();
    y.right = Some(x);
    y.update();
    y
}

#[allow(dead_code)]
fn validate(x: Option<&Node>) {
    fn validate_recurse(x: &Node) {
        matches!(
            ht(x.left.as_deref()) as i8 - ht(x.right.as_deref()) as i8,
            -1..=1
        );
        assert_eq!(x.ht, ht(x.left.as_deref()).max(ht(x.right.as_deref())) + 1);
        if let Some(l) = x.left.as_ref() {
            validate_recurse(l);
        }
        if let Some(r) = x.right.as_ref() {
            validate_recurse(r);
        }
    }
    if let Some(x) = x {
        validate_recurse(x);
    }
}

#[allow(dead_code)]
fn pretty(x: Option<&Node>) -> String {
    fn pretty_recurse(x: &Node, s: &mut String, overlines: &mut Vec<bool>, dir: u8) {
        use std::fmt::Write;
        if let Some(l) = x.left.as_ref() {
            overlines.push(dir == 2);
            pretty_recurse(l, s, overlines, 1);
            overlines.pop().unwrap();
        }
        writeln!(
                s,
                "{overlines}{corner}{branch} {padding} {value} {{ left: {left}, right: {right}, ht: {ht}, len: {len} }}",
                overlines = overlines
                    .iter()
                    .map(|&b| if b { "│" } else { " " })
                    .collect::<String>(),
                corner = match dir {
                    0 => "─",
                    1 => "┌",
                    2 => "└",
                    _ => unreachable!(),
                },
                branch = match (x.left.as_ref(), x.right.as_ref()) {
                    (None, None) => '╴',
                    (Some(_), None) => '┘',
                    (None, Some(_)) => '┐',
                    (Some(_), Some(_)) => '┤',
                },
                padding = "┄".repeat(4_usize.saturating_sub(overlines.len())),
                value = x.value,
                left = x.left.as_ref().map_or_else(||"*".to_owned(), |x| x.value.to_string()),
                right = x.right.as_ref().map_or_else(||"*".to_owned(), |x| x.value.to_string()),
                ht = x.ht,
                len = x.len,
            )
            .unwrap();
        if let Some(r) = x.right.as_ref() {
            overlines.push(dir == 1);
            pretty_recurse(r, s, overlines, 2);
            overlines.pop().unwrap();
        }
    }
    if let Some(x) = x {
        let mut s = String::new();
        pretty_recurse(x, &mut s, &mut vec![], 0);
        assert_eq!(s.pop().unwrap(), '\n');
        s
    } else {
        "(empty)".to_owned()
    }
}

impl crate::test_utils::TreeNode for Node {
    fn left(&self) -> Option<&Self> {
        self.left.as_deref()
    }
    fn right(&self) -> Option<&Self> {
        self.right.as_deref()
    }
    fn value(&self) -> i32 {
        self.value
    }
}

impl crate::test_utils::Validatable for Node {
    const HAS_PARENT_POINTER: bool = false;

    fn validate_balance(&self) -> bool {
        let balance = ht(self.left.as_deref()) as i8 - ht(self.right.as_deref()) as i8;
        matches!(balance, -1..=1)
            && self.ht == ht(self.left.as_deref()).max(ht(self.right.as_deref())) + 1
    }
}

impl crate::test_utils::HasRoot for AvlTreeByBox {
    type Node = Node;
    fn root(&self) -> Option<&Self::Node> {
        self.root.as_deref()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[derive(Debug, Clone, Copy)]
    enum Query {
        Insert { index: usize, value: i32 },
        Remove { index: usize },
    }

    #[test]
    fn test_from_iter() {
        let mut rng = StdRng::seed_from_u64(42);
        for tid in 1..=200 {
            eprintln!("==== Case #{tid}");
            let n = rng.random_range(0..=20);
            let value_lim = 20;
            let vec = std::iter::repeat_with(|| rng.random_range(0..value_lim))
                .take(n)
                .collect::<Vec<_>>();
            let tree: AvlTreeByBox = vec.iter().copied().collect();
            eprintln!("vec = {vec:?}");
            eprintln!("tree:\n{}", pretty(tree.root.as_deref()));
            test_utils::validate(&tree);
            eprintln!("tree validated!");
            assert_eq!(test_utils::collect(&tree), vec);
        }
    }

    #[test]
    fn test_random() {
        let mut rng = StdRng::seed_from_u64(42);
        for tid in 1..=200 {
            let q = 200;
            let len_max = rng.random_range(5..=50);
            let value_lim = 20;
            let mut n = 0_usize;
            let queries: Vec<_> = std::iter::repeat_with(|| {
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
            let mut tree = AvlTreeByBox::new();
            let mut vec = vec![];
            for (qid, &query) in (1..).zip(&queries) {
                eprintln!("==== Case #{tid}.{qid}: {query:?}");
                match query {
                    Query::Insert { index, value } => {
                        tree.insert(index, value);
                        vec.insert(index, value);
                    }
                    Query::Remove { index } => {
                        let result = tree.remove(index);
                        let expected = vec.remove(index);
                        assert_eq!(result, expected);
                    }
                }
                eprintln!("vec = {vec:?}");
                eprintln!("tree:\n{}", pretty(tree.root.as_deref()));
                test_utils::validate(&tree);
                eprintln!("tree validated!");
                assert_eq!(test_utils::collect(&tree), vec);
                eprintln!();
            }
        }
    }

    fn generate_benchmark_queries() -> (AvlTreeByBox, Vec<Query>) {
        let mut rng = StdRng::seed_from_u64(42);
        let n_initial = 200_000;
        let len_max = 200_000;
        let q = 200_000;
        let value_lim = 1_000_000_000;

        let initial_values: Vec<i32> = (0..n_initial)
            .map(|_| rng.random_range(0..value_lim))
            .collect();
        let tree: AvlTreeByBox = initial_values.into_iter().collect();

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

    #[test]
    fn analyze_tree_stats() {
        pub const PHI: f64 = 1.618_033_988_749_895_f64;

        let (mut tree, queries) = generate_benchmark_queries();

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
}
