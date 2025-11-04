#![allow(clippy::unnecessary_box_returns)]
use std::cmp::Ordering;
use std::mem;

use fp::Fp;
use fp::fp;

const P: u64 = 998_244_353;

#[derive(Default)]
pub struct AvlTree {
    root: Option<Box<Node>>,
}
impl AvlTree {
    pub fn insert(&mut self, index: usize, value: Fp<P>) {
        let (l, r) = split2(self.root.take(), index);
        let c = Box::new(Node {
            left: None,
            right: None,
            value,
            sum: value,
            len: 1,
            h: 1,
            rev: false,
            c1: Fp::new(1),
            c0: Fp::new(0),
        });
        self.root = Some(merge3(l, c, r));
    }
    pub fn remove(&mut self, index: usize) {
        let (l, _, r) = split3(self.root.take().unwrap(), index);
        self.root = merge2(l, r);
    }
    pub fn reverse(&mut self, start: usize, end: usize) {
        let (lc, r) = split2(self.root.take(), end);
        let (l, mut c) = split2(lc, start);
        if let Some(c) = c.as_mut() {
            c.rev ^= true;
        }
        self.root = merge2(merge2(l, c), r);
    }
    pub fn affine(&mut self, start: usize, end: usize, c1: Fp<P>, c0: Fp<P>) {
        let (lc, r) = split2(self.root.take(), end);
        let (l, mut c) = split2(lc, start);
        if let Some(c) = c.as_mut() {
            c.value = c1 * c.value + c0;
            c.sum = c1 * c.sum + c0 * fp!(c.len);
            (c.c1, c.c0) = (c1 * c.c1, c1 * c.c0 + c0);
        }
        self.root = merge2(merge2(l, c), r);
    }
    pub fn sum(&mut self, start: usize, end: usize) -> Fp<P> {
        let mut sum = fp!(0);
        let (lc, r) = split2(self.root.take(), end);
        let (l, mut c) = split2(lc, start);
        if let Some(c) = c.as_mut() {
            sum += c.sum;
        }
        self.root = merge2(merge2(l, c), r);
        sum
    }
}

#[derive(Debug)]
pub struct Node {
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    value: Fp<P>,
    sum: Fp<P>,
    len: usize,
    h: u8,
    rev: bool,
    c1: Fp<P>,
    c0: Fp<P>,
}
impl Node {
    fn update(&mut self) {
        assert!(!self.rev);
        self.len = 1;
        self.sum = self.value;
        self.h = 1;
        if let Some(l) = self.left.as_ref() {
            self.len += l.len;
            self.sum += l.sum;
            self.h = self.h.max(l.h + 1);
        }
        if let Some(r) = self.right.as_ref() {
            self.len += r.len;
            self.sum += r.sum;
            self.h = self.h.max(r.h + 1);
        }
    }
    #[allow(dead_code)]
    fn push(&mut self) {
        if self.rev {
            self.rev = false;
            mem::swap(&mut self.left, &mut self.right);
            if let Some(p) = self.left.as_mut() {
                p.rev ^= true;
            }
            if let Some(p) = self.right.as_mut() {
                p.rev ^= true;
            }
        }
        if (self.c1, self.c0) != (fp!(1), fp!(0)) {
            if let Some(p) = self.left.as_mut() {
                p.value = self.c1 * p.value + self.c0;
                p.sum = self.c1 * p.sum + self.c0 * fp!(p.len);
                (p.c1, p.c0) = (self.c1 * p.c1, self.c1 * p.c0 + self.c0);
            }
            if let Some(p) = self.right.as_mut() {
                p.value = self.c1 * p.value + self.c0;
                p.sum = self.c1 * p.sum + self.c0 * fp!(p.len);
                (p.c1, p.c0) = (self.c1 * p.c1, self.c1 * p.c0 + self.c0);
            }
            (self.c1, self.c0) = (fp!(1), fp!(0));
        }
    }
}

fn merge2(l: Option<Box<Node>>, mut r: Option<Box<Node>>) -> Option<Box<Node>> {
    let Some(r) = r.take() else { return l };
    let (_, c, r) = split3(r, 0);
    Some(merge3(l, c, r))
}

fn merge3(l: Option<Box<Node>>, mut c: Box<Node>, r: Option<Box<Node>>) -> Box<Node> {
    match ht(l.as_deref()).cmp(&ht(r.as_deref())) {
        Ordering::Less => {
            let mut r = r.unwrap();
            r.push();
            r.left = Some(merge3(l, c, r.left));
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
            l.push();
            l.right = Some(merge3(l.right, c, r));
            balance(l)
        }
    }
}

fn split2(x: Option<Box<Node>>, index: usize) -> (Option<Box<Node>>, Option<Box<Node>>) {
    if index == 0 {
        return (None, x);
    }
    let (l, c, r) = split3(x.unwrap(), index - 1);
    (Some(merge3(l, c, None)), r)
}

fn split3(mut x: Box<Node>, index: usize) -> (Option<Box<Node>>, Box<Node>, Option<Box<Node>>) {
    x.push();
    let llen = x.left.as_ref().map_or(0, |l| l.len);
    let l = x.left.take();
    let r = x.right.take();
    match index.cmp(&llen) {
        Ordering::Less => {
            let (ll, lc, lr) = split3(l.unwrap(), index);
            (ll, lc, Some(merge3(lr, x, r)))
        }
        Ordering::Equal => {
            x.update();
            (l, x, r)
        }
        Ordering::Greater => {
            let (rl, rc, rr) = split3(r.unwrap(), index - 1 - llen);
            (Some(merge3(l, x, rl)), rc, rr)
        }
    }
}

fn balance(mut x: Box<Node>) -> Box<Node> {
    match ht(x.left.as_deref()) as i8 - ht(x.right.as_deref()) as i8 {
        -2 => {
            x.right = x.right.map(|mut r| {
                if ht(r.left.as_deref()) > ht(r.right.as_deref()) {
                    r.push();
                    rotate_right(r)
                } else {
                    r
                }
            });
            x = rotate_left(x);
        }
        -1..=1 => x.update(),
        2 => {
            x.left = x.left.map(|mut l| {
                if ht(l.left.as_deref()) < ht(l.right.as_deref()) {
                    l.push();
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
    x.map_or(0, |x| x.h)
}

fn rotate_left(mut x: Box<Node>) -> Box<Node> {
    x.push();
    let mut y = x.right.take().unwrap();
    y.push();
    x.right = y.left.take();
    x.update();
    y.left = Some(x);
    y.update();
    y
}

fn rotate_right(mut x: Box<Node>) -> Box<Node> {
    x.push();
    let mut y = x.left.take().unwrap();
    y.push();
    x.left = y.right.take();
    x.update();
    y.right = Some(x);
    y.update();
    y
}

#[allow(dead_code)]
#[allow(clippy::unnecessary_box_returns)]
#[allow(clippy::borrowed_box)]
mod debug {
    use super::{Fp, Node, P};

    pub(crate) fn display(x: Option<&Node>) -> String {
        fn display_recur(x: &Node, d: u8, s: &mut String) {
            use std::fmt::Write;
            if let Some(p) = x.left.as_ref() {
                display_recur(p, d + 1, s);
            }
            writeln!(
                s,
                "{spaces}▶{value} {{ sum: {sum}, c1: {c1}, c0: {c0} }} {rev}",
                spaces = " ".repeat(d as usize),
                value = x.value,
                sum = x.sum,
                c1 = x.c1,
                c0 = x.c0,
                rev = if x.rev { " [rev]" } else { "" }
            )
            .unwrap();
            if let Some(p) = x.right.as_ref() {
                display_recur(p, d + 1, s);
            }
        }
        {
            let Some(x) = x else {
                return "(empty)\n".to_string();
            };
            let mut s = String::new();
            display_recur(x, 0, &mut s);
            s
        }
    }

    pub(crate) fn collect(x: Option<&Node>) -> Vec<Fp<P>> {
        fn collect_recur(
            x: Option<&Node>,
            out: &mut Vec<Fp<P>>,
            mut rev: bool,
            c1: Fp<P>,
            c0: Fp<P>,
        ) {
            let Some(x) = x.as_ref() else { return };
            rev ^= x.rev;
            let (d1, d0) = (c1 * x.c1, c1 * x.c0 + c0);
            if rev {
                collect_recur(x.right.as_deref(), out, rev, d1, d0);
                out.push(c1 * x.value + c0);
                collect_recur(x.left.as_deref(), out, rev, d1, d0);
            } else {
                collect_recur(x.left.as_deref(), out, rev, d1, d0);
                out.push(c1 * x.value + c0);
                collect_recur(x.right.as_deref(), out, rev, d1, d0);
            }
        }
        {
            let mut out = Vec::new();
            collect_recur(x, &mut out, false, Fp::new(1), Fp::new(0));
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::{collect, display};
    use Query::*;
    use rand::{Rng, SeedableRng, rngs::StdRng};

    #[derive(Debug)]
    enum Query {
        Insert {
            index: usize,
            value: Fp<P>,
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
            c1: Fp<P>,
            c0: Fp<P>,
        },
        Sum {
            start: usize,
            end: usize,
        },
    }

    #[test]
    fn test() {
        let mut rng = StdRng::seed_from_u64(42);
        for tid in 0..300 {
            let q = 200;
            let len_max = rng.random_range(5..=100);
            let value_lim = 10;
            let mut tree = AvlTree::default();
            let mut vec = vec![];
            let mut n = 0usize;
            for qid in 0..q {
                let query = match rng.random_range(0..=5) {
                    0 => {
                        let mut start = rng.random_range(0..=n + 1);
                        let mut end = rng.random_range(0..=n);
                        if start > end {
                            (start, end) = (end, start - 1);
                        }
                        Reverse { start, end }
                    }
                    1 => {
                        let mut start = rng.random_range(0..=n + 1);
                        let mut end = rng.random_range(0..=n);
                        if start > end {
                            (start, end) = (end, start - 1);
                        }
                        let c1 = Fp::new(rng.random_range(0..value_lim));
                        let c0 = Fp::new(rng.random_range(0..value_lim));
                        Affine { start, end, c1, c0 }
                    }
                    2 => {
                        let mut start = rng.random_range(0..=n + 1);
                        let mut end = rng.random_range(0..=n);
                        if start > end {
                            (start, end) = (end, start - 1);
                        }
                        Sum { start, end }
                    }
                    3..=5 => {
                        if rng.random_ratio(n as u32, len_max as u32) {
                            let index = rng.random_range(0..n);
                            Remove { index }
                        } else {
                            let index = rng.random_range(0..=n);
                            let value = Fp::new(rng.random_range(0..value_lim));
                            Insert { index, value }
                        }
                    }
                    _ => unreachable!(),
                };
                eprintln!("Query #{tid}.{qid}: {query:?}");
                match query {
                    Insert { index, value } => {
                        n += 1;
                        tree.insert(index, value);
                        vec.insert(index, value);
                    }
                    Remove { index } => {
                        n -= 1;
                        tree.remove(index);
                        vec.remove(index);
                    }
                    Reverse { start, end } => {
                        tree.reverse(start, end);
                        vec[start..end].reverse();
                    }
                    Affine { start, end, c1, c0 } => {
                        tree.affine(start, end, c1, c0);
                        for x in &mut vec[start..end] {
                            *x = *x * c1 + c0;
                        }
                    }
                    Sum { start, end } => {
                        let result = tree.sum(start, end);
                        let expected = vec[start..end].iter().sum::<Fp<_>>();
                        assert_eq!(result, expected);
                    }
                }
                let result = collect(tree.root.as_deref());
                eprintln!("structure:\n{}", display(tree.root.as_deref()));
                eprintln!("n = {n}");
                eprintln!("vec = {vec:?}");
                eprintln!("result = {result:?}");
                assert_eq!(result, vec);
                eprintln!();
            }
        }
    }
}
// lg {{{
// https://ngtkana.github.io/ac-adapter-rs/lg/index.html
#[allow(unused_imports)]
#[allow(dead_code)]
mod lg {
    mod map {
        use crate::lg::align_of;
        use crate::lg::format;
        use crate::lg::table::Align;
        use crate::lg::table::Cell;
        use crate::lg::table::Table;
        use std::collections;
        use std::collections::BTreeMap;
        use std::collections::HashMap;
        use std::fmt;
        use std::iter;
        use std::slice;
        use std::vec;
        pub fn vmap<'a, K, V, M>(title: &str, map: M) -> Table
        where
            M: Copy + Map<'a, K = K, V = V>,
            K: fmt::Debug,
            V: fmt::Debug,
        {
            Table {
                table: iter::once(vec![
                    Cell {
                        text: String::new(),
                        align: Align::Left,
                    },
                    Cell {
                        text: title.to_string(),
                        align: Align::Center,
                    },
                ])
                .chain(map.map_iter().map(|(k, v)| {
                    let v = format(&v);
                    vec![
                        Cell {
                            text: format(&k),
                            align: Align::Center,
                        },
                        Cell {
                            align: align_of(&v),
                            text: v,
                        },
                    ]
                }))
                .collect(),
            }
        }
        pub fn hmap<'a, K, V, M>(title: &str, map: M) -> Table
        where
            M: Copy + Map<'a, K = K, V = V>,
            K: fmt::Debug,
            V: fmt::Debug,
        {
            Table {
                table: vec![
                    iter::once(Cell {
                        text: String::new(),
                        align: Align::Left,
                    })
                    .chain(map.map_iter().map(|(k, _)| Cell {
                        text: format(&k),
                        align: Align::Center,
                    }))
                    .collect(),
                    iter::once(Cell {
                        text: title.to_string(),
                        align: Align::Left,
                    })
                    .chain(map.map_iter().map(|(_, v)| {
                        let v = format(&v);
                        Cell {
                            align: align_of(&v),
                            text: v,
                        }
                    }))
                    .collect(),
                ],
            }
        }
        pub fn deconstruct_ref_tuple<K, V>((k, v): &(K, V)) -> (&K, &V) {
            (k, v)
        }
        pub trait Map<'a>: 'a {
            type K;
            type V;
            type I: Iterator<Item = (&'a Self::K, &'a Self::V)>;
            fn map_iter(self) -> Self::I;
        }
        impl<'a, K, V, S> Map<'a> for &'a HashMap<K, V, S> {
            type I = collections::hash_map::Iter<'a, K, V>;
            type K = K;
            type V = V;
            fn map_iter(self) -> Self::I {
                self.iter()
            }
        }
        impl<'a, K, V> Map<'a> for &'a BTreeMap<K, V> {
            type I = collections::btree_map::Iter<'a, K, V>;
            type K = K;
            type V = V;
            fn map_iter(self) -> Self::I {
                self.iter()
            }
        }
        impl<'a, K, V> Map<'a> for &'a [(K, V)] {
            type I = iter::Map<slice::Iter<'a, (K, V)>, fn(&(K, V)) -> (&K, &V)>;
            type K = K;
            type V = V;
            fn map_iter(self) -> Self::I {
                self.iter().map(deconstruct_ref_tuple)
            }
        }
        impl<'a, K, V> Map<'a> for &'a Vec<(K, V)> {
            type I = iter::Map<slice::Iter<'a, (K, V)>, fn(&(K, V)) -> (&K, &V)>;
            type K = K;
            type V = V;
            fn map_iter(self) -> Self::I {
                self.iter().map(deconstruct_ref_tuple)
            }
        }
        impl<'a, const N: usize, K, V> Map<'a> for &'a [(K, V); N] {
            type I = iter::Map<slice::Iter<'a, (K, V)>, fn(&(K, V)) -> (&K, &V)>;
            type K = K;
            type V = V;
            fn map_iter(self) -> Self::I {
                self.iter().map(deconstruct_ref_tuple)
            }
        }
    }
    mod table {
        use core::fmt;
        const GRAY: &str = "\x1b[48;2;127;127;127;37m";
        const RESET: &str = "\x1b[0m";
        pub struct Table {
            pub table: Vec<Vec<Cell>>,
        }
        pub struct Cell {
            pub text: String,
            pub align: Align,
        }
        pub enum Align {
            Left,
            Center,
            Right,
        }
        impl fmt::Display for Table {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                struct ColumnFormat<'a> {
                    pre: &'a str,
                    width: usize,
                    post: &'a str,
                }
                let Self { table } = self;
                let w = table[0].len();
                assert!(table.iter().all(|row| row.len() == w));
                let column_format = (0..w)
                    .map(|j| ColumnFormat {
                        pre: " ",
                        width: table
                            .iter()
                            .map(|row| row[j].text.len().max(1))
                            .max()
                            .unwrap(),
                        post: if j == 0 { " │" } else { " " },
                    })
                    .collect::<Vec<_>>();
                for (i, row) in table.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{GRAY}")?;
                    }
                    for (&ColumnFormat { pre, width, post }, Cell { text, align }) in
                        column_format.iter().zip(row)
                    {
                        write!(f, "{pre}")?;
                        match align {
                            Align::Left => write!(f, "{text:<width$}")?,
                            Align::Center => write!(f, "{text:^width$}")?,
                            Align::Right => write!(f, "{text:>width$}")?,
                        }
                        write!(f, "{post}")?;
                    }
                    if i == 0 {
                        write!(f, "{RESET}")?;
                    }
                    writeln!(f)?;
                }
                Ok(())
            }
        }
    }
    mod vec2 {
        use crate::lg::align_of;
        use crate::lg::format;
        use crate::lg::table::Align;
        use crate::lg::table::Cell;
        use crate::lg::table::Table;
        use std::fmt;
        use std::iter;
        pub fn vec2<'a, T, R, S>(title: &str, vec2: &'a S) -> Table
        where
            T: fmt::Debug + 'a,
            R: ?Sized,
            &'a R: Copy + IntoIterator<Item = &'a T> + 'a,
            &'a S: Copy + IntoIterator<Item = &'a R>,
        {
            let w = vec2
                .into_iter()
                .map(|row| row.into_iter().count())
                .max()
                .unwrap();
            Table {
                table: iter::once(
                    iter::once(Cell {
                        text: title.to_string(),
                        align: Align::Left,
                    })
                    .chain((0..w).map(|i| Cell {
                        text: i.to_string(),
                        align: Align::Center,
                    }))
                    .collect(),
                )
                .chain(vec2.into_iter().enumerate().map(|(j, row)| {
                    iter::once(Cell {
                        text: j.to_string(),
                        align: Align::Center,
                    })
                    .chain(row.into_iter().map(|v| {
                        let v = format(&v);
                        Cell {
                            align: align_of(&v),
                            text: v,
                        }
                    }))
                    .chain(iter::repeat_with(|| Cell {
                        text: String::new(),
                        align: Align::Left,
                    }))
                    .take(1 + w)
                    .collect()
                }))
                .collect(),
            }
        }
    }
    mod vecs {
        use super::table::Cell;
        use super::table::Table;
        use crate::lg::align_of;
        use crate::lg::table::Align;
        use std::iter;
        pub fn hvec(vecs: &[(String, Vec<String>)]) -> Table {
            let w = vecs.iter().map(|(_, row)| row.len()).max().unwrap();
            Table {
                table: iter::once(
                    iter::once(Cell {
                        text: String::new(),
                        align: Align::Left,
                    })
                    .chain((0..w).map(|i| Cell {
                        text: i.to_string(),
                        align: Align::Center,
                    }))
                    .collect(),
                )
                .chain(vecs.iter().map(|(title, row)| {
                    iter::once(Cell {
                        text: title.to_string(),
                        align: Align::Center,
                    })
                    .chain(row.iter().map(|v| Cell {
                        align: align_of(v),
                        text: v.clone(),
                    }))
                    .chain(iter::repeat_with(|| Cell {
                        text: String::new(),
                        align: Align::Left,
                    }))
                    .take(1 + w)
                    .collect()
                }))
                .collect(),
            }
        }
        pub fn vvec(vecs: &[(String, Vec<String>)]) -> Table {
            let h = vecs.iter().map(|(_, col)| col.len()).max().unwrap();
            Table {
                table: iter::once(
                    iter::once(Cell {
                        text: String::new(),
                        align: Align::Center,
                    })
                    .chain(vecs.iter().map(|(title, _)| Cell {
                        text: title.to_string(),
                        align: Align::Center,
                    }))
                    .collect(),
                )
                .chain((0..h).map(|i| {
                    iter::once(Cell {
                        text: i.to_string(),
                        align: Align::Center,
                    })
                    .chain(vecs.iter().map(|(_, vec)| {
                        let v = vec.get(i).map_or("", String::as_str);
                        Cell {
                            align: align_of(v),
                            text: v.to_string(),
                        }
                    }))
                    .collect()
                }))
                .collect(),
            }
        }
    }
    pub use map::hmap;
    pub use map::vmap;
    use std::borrow::Borrow;
    use std::fmt;
    use table::Align;
    pub use vec2::vec2;
    pub use vecs::hvec;
    pub use vecs::vvec;
    pub fn bools<B, I>(iter: I) -> String
    where
        B: Borrow<bool>,
        I: IntoIterator<Item = B>,
    {
        format!(
            "[{}]",
            iter.into_iter()
                .map(|b| ['.', '#'][usize::from(*(b.borrow()))])
                .collect::<String>(),
        )
    }
    pub fn align_of(s: &str) -> Align {
        // To improve this: https://doc.rust-lang.org/reference/tokens.html#floating-point-literals
        match s.parse::<f64>() {
            Ok(_) => Align::Right,
            Err(_) => Align::Left,
        }
    }
    #[macro_export]
    macro_rules! lg {
        (@contents $head:expr $(, $tail:expr)*) => {{
            $crate::__lg_internal!($head);
            $(
                eprint!(",");
                $crate::__lg_internal!($tail);
            )*
            eprintln!();
        }};
        ($($expr:expr),* $(,)?) => {{
            eprint!("{} \u{276f}", line!());
            $crate::lg!(@contents $($expr),*)
        }};
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __lg_internal {
        ($value:expr) => {{
            match $value {
                head => {
                    eprint!(" {} = {}", stringify!($value), $crate::lg::format(&head));
                }
            }
        }};
    }
    #[macro_export]
    macro_rules! table {
        ($vec2:expr) => {
            eprint!(
                "{}",
                $crate::lg::vec2($crate::lg::remove_ampersand(stringify!($vec2)), $vec2)
            );
        };
    }
    #[macro_export]
    macro_rules! vmap {
        ($map:expr) => {
            eprint!(
                "{}",
                $crate::lg::vmap($crate::lg::remove_ampersand(stringify!($map)), $map)
            );
        };
    }
    #[macro_export]
    macro_rules! hmap {
        ($map:expr) => {
            eprint!(
                "{}",
                $crate::lg::hmap($crate::lg::remove_ampersand(stringify!($map)), $map)
            );
        };
    }
    #[macro_export]
    macro_rules! vvec {
        ($($(@field $field:ident)* $vecs:expr),+ $(,)?) => {
            let mut vecs = Vec::new();
            $(
                let name = $crate::lg::remove_ampersand(stringify!($vecs));
                #[allow(unused_mut, unused_assignments)]
                let mut has_field = false;
                $(
                    #[allow(unused_mut, unused_assignments)]
                    {
                        let mut name = name.to_owned();
                        has_field = true;
                        name.push_str(".");
                        name.push_str(stringify!($field));
                        let values = (&$vecs).into_iter().map(|v| $crate::lg::format(&v.$field)).collect::<Vec<_>>();
                        vecs.push((name, values))
                    }
                )*
                if !has_field {
                    let values = (&$vecs).into_iter().map(|v| $crate::lg::format(&v)).collect::<Vec<_>>();
                    vecs.push((name.to_owned(), values))
                }
            )+
            eprint!("{}", $crate::lg::vvec(&vecs));
        };
    }
    #[macro_export]
    macro_rules! hvec {
        ($($(@field $field:ident)* $vecs:expr),+ $(,)?) => {
            let mut vecs = Vec::new();
            $(
                let name = $crate::lg::remove_ampersand(stringify!($vecs));
                #[allow(unused_mut, unused_assignments)]
                let mut has_field = false;
                $(
                    #[allow(unused_mut, unused_assignments)]
                    {
                        let mut name = name.to_owned();
                        has_field = true;
                        name.push_str(".");
                        name.push_str(stringify!($field));
                        let values = (&$vecs).into_iter().map(|v| $crate::lg::format(&v.$field)).collect::<Vec<_>>();
                        vecs.push((name, values))
                    }
                )*
                if !has_field {
                    let values = (&$vecs).into_iter().map(|v| $crate::lg::format(&v)).collect::<Vec<_>>();
                    vecs.push((name.to_owned(), values))
                }
            )+
            eprint!("{}", $crate::lg::hvec(&vecs));
        };
    }
    pub fn remove_ampersand(mut s: &str) -> &str {
        while let Some(t) = s.strip_prefix('&') {
            s = t;
        }
        s
    }
    pub fn format<T: fmt::Debug>(t: &T) -> String {
        let s = format!("{t:?}")
            .replace("340282366920938463463374607431768211455", "*") // u128
            .replace("170141183460469231731687303715884105727", "*") // i128
            .replace("18446744073709551615", "*") // u64
            .replace("9223372036854775807", "*") // i64
            .replace("-9223372036854775808", "*") // i64
            .replace("4294967295", "*") // u32
            .replace("2147483647", "*") // i32
            .replace("-2147483648", "*") // i32
            .replace("None", "*")
            .replace("true", "#")
            .replace("false", ".");
        let mut s = s.as_str();
        while s.starts_with("Some(") {
            s = s.strip_prefix("Some(").unwrap();
            s = s.strip_suffix(')').unwrap();
        }
        while s.len() > 2 && s.starts_with('"') && s.ends_with('"') {
            s = s.strip_prefix('"').unwrap();
            s = s.strip_suffix('"').unwrap();
        }
        s.to_owned()
    }
}
// }}}
