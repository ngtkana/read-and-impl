use std::{
    cmp::Ordering,
    ptr::{self, null_mut},
};
use Color::{Black, Red};

pub struct RbTreeWithParent {
    root: *mut Node,
}
impl Default for RbTreeWithParent {
    fn default() -> Self {
        Self::new()
    }
}

impl RbTreeWithParent {
    pub fn new() -> Self {
        Self { root: null_mut() }
    }
    pub fn len(&self) -> usize {
        unsafe { self.root.as_ref().map_or(0, |r| r.len) }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn insert(&mut self, index: usize, value: i32) {
        unsafe {
            let c = Box::leak(Box::new(Node {
                parent: null_mut(),
                left: null_mut(),
                right: null_mut(),
                value,
                len: 1,
                bh: 1,
                color: Red,
            }));
            let (l, r) = split2(self.root, index);
            self.root = merge3(l, c, r);
        }
    }
    pub fn remove(&mut self, index: usize) -> i32 {
        unsafe {
            let (l, c, r) = split3(&mut *self.root, index);
            self.root = merge2(l, r);
            Box::from_raw(c).value
        }
    }
}

impl FromIterator<i32> for RbTreeWithParent {
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
                    bh: 0,
                    len: 0,
                    color: Red,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

pub struct Node {
    parent: *mut Self,
    left: *mut Self,
    right: *mut Self,
    value: i32,
    len: usize,
    bh: u8,
    color: Color,
}

impl Node {
    unsafe fn update(&mut self) {
        self.len = 1;
        self.bh = 1;
        if let Some(l) = self.left.as_ref() {
            self.len += l.len;
            self.bh = self.bh.max(l.bh + u8::from(l.color == Black));
        }
        if let Some(r) = self.right.as_ref() {
            self.len += r.len;
            self.bh = self.bh.max(r.bh + u8::from(r.color == Black));
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
    c.color = Red;
    c.parent = null_mut();
    if let Some(l) = l.as_mut() {
        l.color = Black;
        l.parent = null_mut();
    }
    if let Some(r) = r.as_mut() {
        r.color = Black;
        r.parent = null_mut();
    }
    c.update();
    let mut is_less = false;
    match bh(l).cmp(&bh(r)) {
        Ordering::Less => {
            let mut p = null_mut();
            while bh(l) < bh(r) {
                p = r;
                r = (*r).left;
            }
            c.parent = p;
            is_less = true;
        }
        Ordering::Equal => {}
        Ordering::Greater => {
            let mut p = null_mut();
            while bh(l) > bh(r) {
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
        if is_less {
            assert!(ptr::eq(p.left, r));
            p.left = c;
        } else {
            assert!(ptr::eq(p.right, l));
            p.right = c;
        }
    }
    c.update();
    while c.color == Red && color(c.parent) == Red {
        let p = &mut *c.parent;
        let pp = &mut *p.parent;
        match (color(pp.left), color(pp.right)) {
            (Red, Black) => {
                if ptr::eq(p.right, c) {
                    rotate_left(p);
                }
                c = rotate_right(pp);
                break;
            }
            (Black, Red) => {
                if ptr::eq(p.left, c) {
                    rotate_right(p);
                }
                c = rotate_left(pp);
                break;
            }
            _ => {}
        }
        (*pp.left).color = Black;
        pp.color = Red;
        (*pp.right).color = Black;
        p.update();
        pp.update();
        c = pp;
    }
    while let Some(p) = c.parent.as_mut() {
        p.update();
        c = p;
    }
    c.color = Black;
    c
}

unsafe fn bh(x: *const Node) -> u8 {
    x.as_ref().map_or(0, |x| x.bh)
}

unsafe fn color(x: *const Node) -> Color {
    x.as_ref().map_or(Black, |x| x.color)
}

unsafe fn rotate_left(x: &mut Node) -> &mut Node {
    let y = &mut *x.right;
    assert_eq!(y.color, Red);
    y.color = x.color;
    x.color = Red;
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
    assert_eq!(y.color, Red);
    y.color = x.color;
    x.color = Red;
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
                "{overlines}{corner}{branch} {padding} {color} {value} {{ parent: {parent}, left: {left}, right: {right}, len: {len}, bh: {bh} }}",
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
                color = match x.color {
                    Red => '○',
                    Black => '●',
                },
                value = x.value,
                parent = x.parent.as_ref().map_or_else(||"*".to_owned(), |x| x.value.to_string()),
                left = x.left.as_ref().map_or_else(||"*".to_owned(), |x| x.value.to_string()),
                right = x.right.as_ref().map_or_else(||"*".to_owned(), |x| x.value.to_string()),
                len = x.len,
                bh = x.bh,
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
        unsafe {
            let left_bh = bh(self.left) + u8::from(color(self.left) == Black);
            let right_bh = bh(self.right) + u8::from(color(self.right) == Black);
            self.bh == left_bh
                && self.bh == right_bh
                && self.len
                    == self.left.as_ref().map_or(0, |l| l.len)
                        + 1
                        + self.right.as_ref().map_or(0, |r| r.len)
        }
    }
    fn parent(&self) -> Option<&Self> {
        unsafe { self.parent.as_ref() }
    }
}

impl crate::test_utils::Tree for RbTreeWithParent {
    type Node = Node;

    fn root(&self) -> Option<&Self::Node> {
        unsafe { self.root.as_ref() }
    }
    fn len(&self) -> usize {
        self.len()
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
            let tree: RbTreeWithParent = vec.iter().copied().collect();
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
            let mut tree = RbTreeWithParent::new();
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
        test_utils::analyze_tree_stats::<RbTreeWithParent>();
    }
}
