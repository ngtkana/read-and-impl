use std::{
    cmp::Ordering,
    ptr::{self, null_mut},
};

pub struct AvlTreeWithParent {
    root: *mut Node,
}
impl Default for AvlTreeWithParent {
    fn default() -> Self {
        Self::new()
    }
}

impl AvlTreeWithParent {
    pub fn new() -> Self {
        Self { root: null_mut() }
    }
    pub fn len(&self) -> usize {
        unsafe { self.root.as_ref().map_or(0, |r| r.len) }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn height(&self) -> u8 {
        unsafe { self.root.as_ref().map_or(0, |r| r.ht) }
    }
    pub fn insert(&mut self, index: usize, value: i32) {
        unsafe {
            let c = Box::leak(Box::new(Node {
                parent: null_mut(),
                left: null_mut(),
                right: null_mut(),
                value,
                len: 1,
                ht: 1,
            }));
            let (l, r) = split2(self.root, index);
            self.root = merge3(l, c, r);
        }
    }
    pub fn remove(&mut self, index: usize) -> i32 {
        unsafe {
            let (l, c, r) = split3(&mut *self.root, index);
            let c = Box::from_raw(c);
            self.root = merge2(l, r);
            c.value
        }
    }
}

impl FromIterator<i32> for AvlTreeWithParent {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        fn from_iter_recurse(values: &[i32]) -> *mut Node {
            unsafe {
                let n = values.len();
                if n == 0 {
                    return null_mut();
                }
                let l = from_iter_recurse(&values[..n / 2]);
                let c = Box::leak(Box::new(Node {
                    parent: null_mut(),
                    left: null_mut(),
                    right: null_mut(),
                    value: values[n / 2],
                    ht: 0,
                    len: 0,
                }));
                let r = from_iter_recurse(&values[n / 2 + 1..]);
                c.left = l;
                c.right = r;
                if let Some(l) = l.as_mut() {
                    l.parent = c;
                }
                if let Some(r) = r.as_mut() {
                    r.parent = c;
                }
                c.update();
                c
            }
        }
        let values: Vec<_> = iter.into_iter().collect();
        let root = from_iter_recurse(&values);
        Self { root }
    }
}

pub struct Node {
    parent: *mut Self,
    left: *mut Self,
    right: *mut Self,
    value: i32,
    len: usize,
    ht: u8,
}

impl Node {
    unsafe fn update(&mut self) {
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

unsafe fn split2(x: *mut Node, index: usize) -> (*mut Node, *mut Node) {
    if index == x.as_ref().map_or(0, |x| x.len) {
        return (x, null_mut());
    }
    let (l, c, r) = split3(&mut *x, index);
    (l, merge2(c, r))
}

unsafe fn split3(mut x: &mut Node, mut index: usize) -> (*mut Node, &mut Node, *mut Node) {
    loop {
        let llen = x.left.as_ref().map_or(0, |x| x.len);
        match index.cmp(&llen) {
            Ordering::Less => x = &mut *x.left,
            Ordering::Equal => break,
            Ordering::Greater => {
                x = &mut *x.right;
                index -= llen + 1;
            }
        }
    }
    while let Some(p) = x.parent.as_mut() {
        if let Some(pp) = p.parent.as_mut() {
            if ptr::eq(pp.left, p) {
                pp.left = x;
            } else {
                pp.right = x;
            }
        }
        x.parent = p.parent;
        if ptr::eq(p.left, x) {
            x.right = merge3(x.right, p, p.right);
        } else {
            assert!(ptr::eq(p.right, x));
            x.left = merge3(p.left, p, x.left);
        }
    }
    let l = x.left;
    let r = x.right;
    x.left = null_mut();
    x.right = null_mut();
    x.parent = null_mut();
    if let Some(l) = l.as_mut() {
        l.parent = null_mut();
    }
    if let Some(r) = r.as_mut() {
        r.parent = null_mut();
    }
    x.update();
    (l, x, r)
}

unsafe fn merge2(l: *mut Node, r: *mut Node) -> *mut Node {
    let Some(r) = r.as_mut() else { return l };
    let (_, c, r) = split3(r, 0);
    merge3(l, c, r)
}

unsafe fn merge3(mut l: *mut Node, mut c: &mut Node, mut r: *mut Node) -> &mut Node {
    c.parent = null_mut();
    if let Some(l) = l.as_mut() {
        l.parent = null_mut();
    }
    if let Some(r) = r.as_mut() {
        r.parent = null_mut();
    }
    match ht(l).cmp(&ht(r)) {
        Ordering::Less => {
            let mut p = null_mut();
            while ht(l) + 1 < ht(r) {
                p = r;
                r = (*r).left;
            }
            if let Some(p0) = p.as_mut().filter(|p| ht(p.left) > ht(p.right)) {
                rotate_right(p0);
                p = r;
                r = (*r).left;
            }
            c.parent = p;
        }
        Ordering::Equal => {}
        Ordering::Greater => {
            let mut p = null_mut();
            while ht(l) > ht(r) + 1 {
                p = l;
                l = (*l).right;
            }
            if let Some(p0) = p.as_mut().filter(|p| ht(p.left) < ht(p.right)) {
                rotate_left(p0);
                p = l;
                l = (*l).right;
            }
            c.parent = p;
        }
    }
    c.left = l;
    c.right = r;
    if let Some(l) = c.left.as_mut() {
        l.parent = c;
    }
    if let Some(r) = c.right.as_mut() {
        r.parent = c;
    }
    if let Some(p) = c.parent.as_mut() {
        if ptr::eq(p.left, r) {
            p.left = c;
        } else {
            p.right = c;
        }
    }
    c.update();
    while let Some(p) = c.parent.as_mut() {
        p.update();
        c = p;
    }
    c
}

unsafe fn ht(x: *const Node) -> u8 {
    x.as_ref().map_or(0, |x| x.ht)
}

unsafe fn rotate_left(x: &mut Node) -> &mut Node {
    let y = &mut *x.right;
    y.parent = x.parent;
    if let Some(p) = y.parent.as_mut() {
        if ptr::eq(p.left, x) {
            p.left = y;
        } else {
            p.right = y;
        }
    }
    x.right = y.left;
    if let Some(c) = x.right.as_mut() {
        c.parent = x;
    }
    x.parent = y;
    y.left = x;
    x.update();
    y.update();
    x
}

unsafe fn rotate_right(x: &mut Node) -> &mut Node {
    let y = &mut *x.left;
    y.parent = x.parent;
    if let Some(p) = y.parent.as_mut() {
        if ptr::eq(p.right, x) {
            p.right = y;
        } else {
            p.left = y;
        }
    }
    x.left = y.right;
    if let Some(c) = x.left.as_mut() {
        c.parent = x;
    }
    x.parent = y;
    y.right = x;
    x.update();
    y.update();
    x
}

#[allow(dead_code)]
fn pretty(x: *mut Node) -> String {
    fn pretty_recurse(x: &Node, s: &mut String, overlines: &mut Vec<bool>) {
        unsafe {
            use std::fmt::Write;
            let dir = match x.parent.as_ref() {
                Some(p) if ptr::eq(p.left, x) => 1,
                Some(p) if ptr::eq(p.right, x) => 2,
                _ => 0,
            };
            if let Some(l) = x.left.as_ref() {
                overlines.push(dir == 2);
                pretty_recurse(l, s, overlines);
                overlines.pop().unwrap();
            }
            writeln!(
                s,
                "{overlines}{corner}{branch} {padding} {value} {{ parent: {parent}, left: {left}, right: {right}, ht: {ht} }}",
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
                parent = x.parent.as_ref().map_or_else(||"*".to_owned(), |x| x.value.to_string()),
                left = x.left.as_ref().map_or_else(||"*".to_owned(), |x| x.value.to_string()),
                right = x.right.as_ref().map_or_else(||"*".to_owned(), |x| x.value.to_string()),
                ht = x.ht,
            )
            .unwrap();
            if let Some(r) = x.right.as_ref() {
                overlines.push(dir == 1);
                pretty_recurse(r, s, overlines);
                overlines.pop().unwrap();
            }
        }
    }
    unsafe {
        if let Some(x) = x.as_ref() {
            let mut s = String::new();
            pretty_recurse(x, &mut s, &mut vec![]);
            assert_eq!(s.pop().unwrap(), '\n');
            s
        } else {
            "(empty)".to_owned()
        }
    }
}

#[allow(dead_code)]
fn validate(x: *const Node) {
    fn validate_recurse(x: &Node) {
        unsafe {
            matches!(ht(x.left) as i8 - ht(x.right) as i8, -1..=1);
            assert_eq!(x.ht, ht(x.left).max(ht(x.right)) + 1);
            if let Some(l) = x.left.as_ref() {
                assert!(ptr::eq(l.parent, x));
                validate_recurse(l);
            }
            if let Some(r) = x.right.as_ref() {
                assert!(ptr::eq(r.parent, x));
                validate_recurse(r);
            }
        }
    }
    unsafe {
        if let Some(x) = x.as_ref() {
            assert!(x.parent.is_null());
            validate_recurse(x);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[derive(Debug, Clone, Copy)]
    enum Query {
        Insert { index: usize, value: i32 },
        Remove { index: usize },
    }

    fn collect(tree: &AvlTreeWithParent) -> Vec<i32> {
        fn collect_recurse(x: Option<&Node>, out: &mut Vec<i32>) {
            unsafe {
                let Some(x) = x.as_ref() else { return };
                collect_recurse(x.left.as_ref(), out);
                out.push(x.value);
                collect_recurse(x.right.as_ref(), out);
            }
        }
        unsafe {
            let mut out = vec![];
            collect_recurse(tree.root.as_ref(), &mut out);
            out
        }
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
            let tree: AvlTreeWithParent = vec.iter().copied().collect();
            eprintln!("vec = {vec:?}");
            eprintln!("tree\n{}", pretty(tree.root));
            validate(tree.root);
            eprintln!("tree validated!");
            assert_eq!(collect(&tree), vec);
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
            let mut tree = AvlTreeWithParent::new();
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
                eprintln!("tree\n{}", pretty(tree.root));
                validate(tree.root);
                eprintln!("tree validated!");
                assert_eq!(collect(&tree), vec);
                eprintln!();
            }
        }
    }

    fn generate_benchmark_queries() -> (AvlTreeWithParent, Vec<Query>) {
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
