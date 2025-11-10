use std::{
    cmp::Ordering,
    ptr::{self, null_mut},
};

pub struct UnbalancedTreeWithParent {
    root: *mut Node,
}
impl Default for UnbalancedTreeWithParent {
    fn default() -> Self {
        Self::new()
    }
}

impl UnbalancedTreeWithParent {
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
        0
    }
    pub fn insert(&mut self, index: usize, value: i32) {
        unsafe {
            let c = Box::leak(Box::new(Node {
                parent: null_mut(),
                left: null_mut(),
                right: null_mut(),
                value,
                len: 1,
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

impl FromIterator<i32> for UnbalancedTreeWithParent {
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
}

impl Node {
    unsafe fn update(&mut self) {
        self.len = 1;
        if let Some(l) = self.left.as_ref() {
            self.len += l.len;
        }
        if let Some(r) = self.right.as_ref() {
            self.len += r.len;
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
        x.parent = p.parent;
        if ptr::eq(p.left, x) {
            p.left = x.right;
            if let Some(y) = p.left.as_mut() {
                y.parent = p;
            }
            x.right = p;
        } else {
            assert!(std::ptr::eq(p.right, x));
            p.right = x.left;
            if let Some(y) = p.right.as_mut() {
                y.parent = p;
            }
            x.left = p;
        }
        if let Some(pp) = p.parent.as_mut() {
            if ptr::eq(pp.left, p) {
                pp.left = x;
            } else {
                assert!(ptr::eq(pp.right, p));
                pp.right = x;
            }
        }
        p.parent = x;
        p.update();
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

unsafe fn merge3(l: *mut Node, c: &mut Node, r: *mut Node) -> &mut Node {
    c.parent = null_mut();
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
                "{overlines}{corner}{branch} {padding} {value} {{ parent: {parent}, left: {left}, right: {right}  }}",
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

impl crate::test_utils::TreeNode for Node {
    fn left(&self) -> Option<&Self> {
        unsafe { self.left.as_ref() }
    }
    fn right(&self) -> Option<&Self> {
        unsafe { self.right.as_ref() }
    }
    fn value(&self) -> i32 {
        self.value
    }
}

impl crate::test_utils::Validatable for Node {
    const HAS_PARENT_POINTER: bool = true;

    fn validate_balance(&self) -> bool {
        true
    }
    fn parent(&self) -> Option<&Self> {
        unsafe { self.parent.as_ref() }
    }
}

impl crate::test_utils::Tree for UnbalancedTreeWithParent {
    type Node = Node;

    fn root(&self) -> Option<&Self::Node> {
        unsafe { self.root.as_ref() }
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn height(&self) -> u8 {
        self.height()
    }
    fn insert(&mut self, index: usize, value: i32) {
        self.insert(index, value);
    }
    fn remove(&mut self, index: usize) -> i32 {
        self.remove(index)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::{self, Query};
    use rand::{rngs::StdRng, Rng, SeedableRng};

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
            let tree: UnbalancedTreeWithParent = vec.iter().copied().collect();
            eprintln!("vec = {vec:?}");
            eprintln!("tree\n{}", pretty(tree.root));
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
            let mut tree = UnbalancedTreeWithParent::new();
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
                test_utils::validate(&tree);
                eprintln!("tree validated!");
                assert_eq!(test_utils::collect(&tree), vec);
                eprintln!();
            }
        }
    }

    #[test]
    fn analyze_tree_stats() {
        test_utils::analyze_tree_stats::<UnbalancedTreeWithParent>();
    }
}
