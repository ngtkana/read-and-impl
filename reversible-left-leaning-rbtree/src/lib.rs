#![allow(unsafe_op_in_unsafe_fn)]
use Color::{Black, Red};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fmt::Debug,
    mem,
    ptr::{self, null_mut},
};

const P: u64 = 998_244_353;
type Fp = fp::Fp<P>;

pub struct Rbtree {
    root: *mut Node,
}
impl Default for Rbtree {
    fn default() -> Self {
        Self { root: null_mut() }
    }
}
impl Rbtree {
    pub fn insert(&mut self, index: usize, value: Fp) {
        unsafe {
            let c = Box::leak(Box::new(Node {
                left: null_mut(),
                right: null_mut(),
                color: Color::Red,
                rev: false,
                c1: Fp::new(1),
                c0: Fp::new(0),
                value,
                sum: value,
                len: 1,
                bh: 1,
            }));
            let (l, r) = split2(self.root, index);
            self.root = merge3(l, c, r);
        }
    }
    pub fn remove(&mut self, index: usize) {
        unsafe {
            let (l, _, r) = split3(&mut *self.root, index);
            self.root = merge2(l, r);
        }
    }
    pub fn reverse(&mut self, start: usize, end: usize) {
        unsafe {
            let root: &mut Node = &mut *self.root;
            let (lc, r) = split2(root, end);
            let (l, c) = split2(lc, start);
            (*c).rev ^= true;
            self.root = merge2(merge2(l, c), r);
        }
    }
    pub fn affine(&mut self, start: usize, end: usize, c1: Fp, c0: Fp) {
        unsafe {
            let root: &mut Node = &mut *self.root;
            let (lc, r) = split2(root, end);
            let (l, c) = split2(lc, start);
            ((*c).c1, (*c).c0) = ((*c).c1 * c1, (*c).c0 * c1 + c0);
            (*c).value = (*c).value * c1 + c0;
            (*c).sum = (*c).sum * c1 + c0 * Fp::new((*c).len as u64);
            self.root = merge2(merge2(l, c), r);
        }
    }
    pub fn sum(&mut self, start: usize, end: usize) -> Fp {
        unsafe {
            let mut ans = Fp::new(0);
            let root: &mut Node = &mut *self.root;
            let (lc, r) = split2(root, end);
            let (l, c) = split2(lc, start);
            ans += (*c).sum;
            self.root = merge2(merge2(l, c), r);
            ans
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Red,
    Black,
}

unsafe fn color(x: *const Node) -> Color {
    x.as_ref().map_or(Black, |x| x.color)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    left: *mut Self,
    right: *mut Self,
    color: Color,
    value: Fp,
    rev: bool,
    c1: Fp,
    c0: Fp,
    sum: Fp,
    len: usize,
    bh: u8,
}

impl Node {
    fn update(&mut self) {
        unsafe {
            self.len = 1;
            self.sum = self.value;
            self.bh = 1;
            if let Some(p) = self.left.as_mut() {
                self.len += p.len;
                self.sum += p.sum;
                self.bh = p.bh + u8::from(p.color == Black);
                self.left = p;
            }
            if let Some(p) = self.right.as_mut() {
                self.len += p.len;
                self.sum += p.sum;
                self.right = p;
            }
        }
    }
}

fn push(mut x: &mut Node) -> &mut Node {
    unsafe {
        if (x.c1, x.c0) != (Fp::new(1), Fp::new(0)) {
            if let Some(p) = x.left.as_mut() {
                (p.c1, p.c0) = (p.c1 * x.c1, p.c0 * x.c1 + x.c0);
                p.value = p.value * x.c1 + x.c0;
                p.sum = p.sum * x.c1 + x.c0 * Fp::new(p.len as u64);
            }
            if let Some(p) = x.right.as_mut() {
                (p.c1, p.c0) = (p.c1 * x.c1, p.c0 * x.c1 + x.c0);
                p.value = p.value * x.c1 + x.c0;
                p.sum = p.sum * x.c1 + x.c0 * Fp::new(p.len as u64);
            }
            (x.c1, x.c0) = (Fp::new(1), Fp::new(0));
        }
        if x.rev {
            x.rev = false;
            if color(x.left) == Red {
                x.left = push(&mut *x.left);
                x = rotate_right(x);
            }
            mem::swap(&mut x.left, &mut x.right);
            if let Some(p) = x.left.as_mut() {
                p.rev ^= true;
            }
            if let Some(p) = x.right.as_mut() {
                p.rev ^= true;
            }
        }
        x
    }
}

fn merge2(l: *mut Node, r: *mut Node) -> *mut Node {
    unsafe {
        let Some(l) = l.as_mut() else { return r };
        let Some(r) = r.as_mut() else { return l };
        let (_, c, r) = split3(r, 0);
        merge3(l, c, r)
    }
}

unsafe fn merge3(l: *mut Node, c: &mut Node, r: *mut Node) -> &mut Node {
    let root = merge_recurse(l, c, r);
    root.color = Black;
    root
}

unsafe fn merge_recurse(l: *mut Node, c: &mut Node, r: *mut Node) -> &mut Node {
    let root = match (l.as_ref().map_or(0, |p| p.bh)).cmp(&r.as_ref().map_or(0, |p| p.bh)) {
        Less => {
            let r = push(&mut *r);
            r.left = merge_recurse(l, c, r.left);
            r
        }
        Equal => {
            c.left = l;
            c.right = r;
            c.color = Red;
            c
        }
        Greater => {
            let l = push(&mut *l);
            l.right = merge_recurse(l.right, c, r);
            l
        }
    };
    root.update();
    fixup(root)
}

unsafe fn split2(x: *mut Node, index: usize) -> (*mut Node, *mut Node) {
    let Some(indexm1) = index.checked_sub(1) else {
        return (null_mut(), x);
    };
    let (l, c, r) = split3(&mut *x, indexm1);
    (merge3(l, c, null_mut()), r)
}

unsafe fn split3(mut x: &mut Node, index: usize) -> (*mut Node, &mut Node, *mut Node) {
    if x.color == Black && color(x.left) == Black {
        x.color = Red;
    }
    x = push(x);
    let (l, removed, r) = split_recurse(x, index);
    (l, removed, r)
}

unsafe fn split_recurse(mut x: &mut Node, index: usize) -> (*mut Node, &mut Node, *mut Node) {
    let (l, removed, r) = match index.cmp(&(x.left.as_mut().map_or(0, |p| p.len))) {
        Less => {
            x.left = push(&mut *x.left);
            if (*x.left).color == Black && color((*x.left).left) == Black {
                x = move_red_left(x);
                (*x.right).color = Black;
            }
            let (l, removed, r) = split_recurse(&mut *x.left, index);
            let r = merge_recurse(r, x, x.right);
            r.color = Black;
            (l, removed, ptr::from_mut(r))
        }
        Equal => {
            let l = mem::replace(&mut x.left, null_mut());
            let r = mem::replace(&mut x.right, null_mut());
            if let Some(l) = l.as_mut() {
                l.color = Black;
            }
            x.color = Red;
            x.update();
            (l, x, r)
        }
        Greater => {
            if color(x.left) == Red {
                x = rotate_right(x);
            }
            x.right = push(&mut *x.right);
            if (*x.right).color == Black && color((*x.right).left) == Black {
                x = move_red_right(x);
                (*x.left).color = Black;
            }
            let (l, removed, r) = split_recurse(&mut *x.right, index + (*x.right).len - x.len);
            let l = merge_recurse(x.left, x, l);
            l.color = Black;
            (ptr::from_mut(l), removed, r)
        }
    };
    (l, removed, r)
}

unsafe fn fixup(mut x: &mut Node) -> &mut Node {
    if color(x.right) == Red {
        x = rotate_left(x);
    }
    if color(x.left) == Red && color((*x.left).left) == Red {
        x = rotate_right(x);
    }
    if color(x.left) == Red && color(x.right) == Red {
        split_four_node(x);
    }
    x
}

unsafe fn split_four_node(x: &mut Node) {
    x.color = Red;
    (*x.left).color = Black;
    (*x.right).color = Black;
    x.bh += 1;
}

unsafe fn join_two_nodes(x: &mut Node) {
    x.color = Black;
    (*x.left).color = Red;
    (*x.right).color = Red;
    x.bh -= 1;
}

unsafe fn move_red_left(mut x: &mut Node) -> &mut Node {
    x.right = push(&mut *x.right);
    join_two_nodes(x);
    if color((*x.right).left) == Red {
        x.right = rotate_right(&mut *x.right);
        x = rotate_left(x);
        split_four_node(x);
    }
    x
}

unsafe fn move_red_right(mut x: &mut Node) -> &mut Node {
    x.left = push(&mut *x.left);
    join_two_nodes(x);
    if color((*x.left).left) == Red {
        x = rotate_right(x);
        split_four_node(x);
    }
    x
}

unsafe fn rotate_left(x: &mut Node) -> &mut Node {
    let y = &mut *x.right;
    let x = push(x);
    let y = push(y);
    x.right = y.left;
    y.left = x;
    y.color = x.color;
    x.color = Red;
    x.update();
    y.update();
    y
}

unsafe fn rotate_right(x: &mut Node) -> &mut Node {
    let y = &mut *x.left;
    let x = push(x);
    let y = push(y);
    x.left = y.right;
    y.right = x;
    y.color = x.color;
    x.color = Red;
    x.update();
    y.update();
    y
}

mod fp {
    use std::ops::Add;
    use std::ops::AddAssign;
    use std::ops::Mul;

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Fp<const P: u64> {
        value: u64,
    }
    impl<const P: u64> Fp<P> {
        pub const fn new(value: u64) -> Self {
            Self { value: value % P }
        }
        pub const fn value(self) -> u64 {
            self.value
        }
    }
    impl<const P: u64> std::fmt::Debug for Fp<P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }
    impl<const P: u64> std::fmt::Display for Fp<P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value())
        }
    }
    impl<const P: u64> Add for Fp<P> {
        type Output = Self;
        fn add(mut self, rhs: Fp<P>) -> Self::Output {
            self += rhs;
            self
        }
    }
    impl<const P: u64> AddAssign for Fp<P> {
        fn add_assign(&mut self, rhs: Self) {
            self.value += rhs.value;
            if self.value >= P {
                self.value -= P;
            }
        }
    }
    impl<const P: u64> Mul for Fp<P> {
        type Output = Self;
        fn mul(self, rhs: Fp<P>) -> Self::Output {
            Self {
                value: self.value * rhs.value % P,
            }
        }
    }
}

#[allow(dead_code)]
mod debug {
    use super::{Black, Fp, Node, Red, color};

    pub(super) fn display(x: *const Node) -> String {
        unsafe fn inner(
            x: *const Node,
            depth: usize,
            f: impl Fn(&Node, &mut String, usize) + Copy,
            out: &mut String,
        ) {
            let Some(x) = x.as_ref() else { return };
            inner(
                x.left,
                depth + 1 + usize::from(x.color == Black && color(x.left) == Black),
                f,
                &mut *out,
            );
            f(x, out, depth);
            inner(
                x.right,
                depth + 1 + usize::from(x.color == Black && color(x.right) == Black),
                f,
                &mut *out,
            );
        }
        unsafe {
            if let Some(root) = x.as_ref() {
                let mut s = String::new();
                inner(root, 0, format_node, &mut s);
                s
            } else {
                "× (empty)\n".to_owned()
            }
        }
    }

    pub(super) fn format_node(node: &Node, s: &mut String, depth: usize) {
        use std::fmt::Write;
        let Node {
            color,
            value,
            rev,
            c1,
            c0,
            len,
            sum,
            ..
        } = node;
        writeln!(
        s,
        "{depth}{color} {{ value: {value}, rev: {rev}, affine: ({c1},{c0}), sum: {sum}, len: {len} }}",
        depth = " ".repeat(depth),
        color = match color {
            Black => "●",
            Red => "○",
        },
    )
    .unwrap();
    }

    pub(super) fn is_valid(x: *const Node) -> bool {
        fn is_valid_recurse(x: *const Node) -> Result<(), ()> {
            unsafe {
                let Some(x) = x.as_ref() else { return Ok(()) };
                if color(x.right) == Red {
                    return Err(());
                }
                if color(x.left) == Red && color((*x.left).left) == Red {
                    return Err(());
                }
                let llen = x.left.as_ref().map_or(0, |p| p.len);
                let rlen = x.right.as_ref().map_or(0, |p| p.len);
                let len = 1 + llen + rlen;
                assert_eq!(len, x.len);
                let lh = x
                    .left
                    .as_ref()
                    .map_or(1, |p| p.bh + u8::from(p.color == Black));
                let rh = x
                    .right
                    .as_ref()
                    .map_or(1, |p| p.bh + u8::from(p.color == Black));
                assert_eq!(lh, x.bh);
                assert_eq!(rh, x.bh);
                let lsum = x
                    .left
                    .as_ref()
                    .map_or(Fp::new(0), |p| p.sum * x.c1 + x.c0 * Fp::new(p.len as u64));
                let rsum = x
                    .right
                    .as_ref()
                    .map_or(Fp::new(0), |p| p.sum * x.c1 + x.c0 * Fp::new(p.len as u64));
                assert_eq!(lsum + x.value + rsum, x.sum);
                is_valid_recurse(x.left)?;
                is_valid_recurse(x.right)?;
                Ok(())
            }
        }
        is_valid_recurse(x).is_ok()
    }

    pub(super) fn collect(x: *const Node) -> Vec<Fp> {
        unsafe fn collect_recurse(x: *const Node, out: &mut Vec<Fp>, c1: Fp, c0: Fp, rev: bool) {
            if let Some(x) = x.as_ref() {
                let (d1, d0) = (c1 * x.c1, c1 * x.c0 + c0);
                let rev = rev ^ x.rev;
                if rev {
                    collect_recurse(x.right, out, d1, d0, rev);
                    out.push(x.value * c1 + c0);
                    collect_recurse(x.left, out, d1, d0, rev);
                } else {
                    collect_recurse(x.left, out, d1, d0, rev);
                    out.push(x.value * c1 + c0);
                    collect_recurse(x.right, out, d1, d0, rev);
                }
            }
        }
        unsafe {
            let mut out = Vec::new();
            collect_recurse(x, &mut out, Fp::new(1), Fp::new(0), false);
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::debug::*;
    use super::*;
    use Query::*;
    use rand::{Rng, SeedableRng, rngs::StdRng};

    #[derive(Clone, Copy, PartialEq, Debug)]
    enum Query {
        Insert {
            index: usize,
            value: Fp,
        },
        Remove {
            index: usize,
        },
        Reverse {
            start: usize,
            end: usize,
        },
        Affine {
            start: usize,
            end: usize,
            c1: Fp,
            c0: Fp,
        },
        Sum {
            start: usize,
            end: usize,
        },
    }

    #[test]
    fn test() {
        let mut rng = StdRng::seed_from_u64(42);
        for tid in 0..200 {
            let query_count = 300;
            let value_lim = P;
            let init_len = 100;
            let q = rng.random_range(1..=query_count);
            let mut queries = Vec::new();
            let mut n = 0;
            for _ in 0..rng.random_range(0..=init_len) {
                let index = rng.random_range(0..=n);
                let value = Fp::new(rng.random_range(0..value_lim));
                n += 1;
                queries.push(Insert { index, value });
            }
            for _ in 0..q {
                let index = rng.random_range(0..=n);
                let value = Fp::new(rng.random_range(0..value_lim));
                let query = if n == 0 {
                    n += 1;
                    Insert { index, value }
                } else {
                    let mut start = rng.random_range(0..n);
                    let mut end = rng.random_range(0..=n);
                    if start >= end {
                        mem::swap(&mut start, &mut end);
                        end += 1;
                    }
                    match rng.random_range(0i32..=4) {
                        0 => {
                            n += 1;
                            Insert { index, value }
                        }
                        1 => {
                            let index = rng.random_range(0..n);
                            n -= 1;
                            Remove { index }
                        }
                        2 => Reverse { start, end },
                        3 => {
                            let c1 = Fp::new(rng.random_range(0..value_lim));
                            Affine {
                                start,
                                end,
                                c1,
                                c0: value,
                            }
                        }
                        4 => Sum { start, end },
                        _ => unreachable!(),
                    }
                };
                queries.push(query);
            }
            let mut rb = Rbtree::default();
            let mut vec = Vec::new();
            for (qid, &query) in queries.iter().enumerate() {
                eprintln!();
                eprintln!("Query #{tid}.{qid}: {query:?}");
                match query {
                    Insert { index, value } => {
                        rb.insert(index, value);
                        vec.insert(index, value);
                    }
                    Remove { index } => {
                        rb.remove(index);
                        vec.remove(index);
                    }
                    Reverse { start, end } => {
                        rb.reverse(start, end);
                        vec[start..end].reverse();
                    }
                    Affine { start, end, c1, c0 } => {
                        rb.affine(start, end, c1, c0);
                        for x in &mut vec[start..end] {
                            *x = *x * c1 + c0;
                        }
                    }
                    Sum { start, end } => {
                        let result = rb.sum(start, end);
                        let expected = vec[start..end]
                            .iter()
                            .copied()
                            .fold(Fp::new(0), std::ops::Add::add);
                        eprintln!("result = {result}, expected = {expected}");
                        assert_eq!(result, expected);
                    }
                }
                eprintln!("vec = {vec:?}");
                eprintln!("rb:\n{}", display(rb.root));
                assert_eq!(&collect(rb.root), &vec);
                assert!(is_valid(rb.root));
                eprintln!();
            }
        }
    }
}
