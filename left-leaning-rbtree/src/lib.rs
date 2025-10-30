use std::{cmp::Ordering, ptr::null_mut};
use Color::{Black, Red};

#[allow(dead_code)]
pub struct Rbtree {
    root: *mut Node,
}
impl Rbtree {
    pub fn insert(&mut self, key: u64) {
        unsafe {
            self.root = insert(self.root, key);
            (*self.root).color = Black;
        }
    }
    pub fn remove(&mut self, key: u64) {
        unsafe {
            if !self.root.is_null() {
                if !self.root.is_null() && color((*self.root).left) == Color::Black {
                    (*self.root).color = Red;
                }
                self.root = remove(&mut *self.root, key);
                if !self.root.is_null() {
                    (*self.root).color = Black;
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Color {
    Red,
    Black,
}
impl Color {
    fn other(self) -> Self {
        match self {
            Red => Black,
            Black => Red,
        }
    }
}

unsafe fn color(node: *const Node) -> Color {
    if let Some(node) = node.as_ref() {
        node.color
    } else {
        Black
    }
}

struct Node {
    left: *mut Self,
    right: *mut Self,
    key: u64,
    color: Color,
    len: usize,
}
impl Node {
    fn new(key: u64, color: Color) -> Self {
        Self {
            left: null_mut(),
            right: null_mut(),
            key,
            color,
            len: 1,
        }
    }
    unsafe fn update(&mut self) {
        self.len = 1;
        if let Some(left) = self.left.as_ref() {
            self.len += left.len;
        }
        if let Some(right) = self.right.as_ref() {
            self.len += right.len;
        }
    }
}

unsafe fn rotate_right(x: &mut Node) -> &mut Node {
    let y = &mut *x.left;
    assert_eq!(y.color, Red, "Called rotate_right for a black link!");
    x.left = y.right;
    y.right = x;
    y.color = x.color;
    x.color = Red;
    x.update();
    y.update();
    y
}

unsafe fn rotate_left(x: &mut Node) -> &mut Node {
    let y = &mut *x.right;
    assert_eq!(y.color, Red, "Called rotate_left for a black link!");
    x.right = y.left;
    y.left = x;
    y.color = x.color;
    x.color = Red;
    x.update();
    y.update();
    y
}

unsafe fn insert(root: *mut Node, key: u64) -> *mut Node {
    let Some(root) = root.as_mut() else {
        return Box::leak(Box::new(Node::new(key, Red)));
    };
    match root.key.cmp(&key) {
        Ordering::Greater => root.left = insert(root.left, key),
        Ordering::Equal => (),
        Ordering::Less => root.right = insert(root.right, key),
    }
    root.update();
    fix_up(root)
}

unsafe fn remove(mut root: &mut Node, key: u64) -> *mut Node {
    match root.key.cmp(&key) {
        Ordering::Greater => {
            if color(root.left) == Black
                && !root.left.is_null()
                && color((*root.left).left) == Black
            {
                root = move_red_left(root);
            }
            if !root.left.is_null() {
                root.left = remove(&mut *root.left, key);
                root.update();
            }
        }
        cmp @ (Ordering::Equal | Ordering::Less) => {
            if color(root.left) == Red {
                root = rotate_right(root);
            }
            if cmp == Ordering::Equal && root.right.is_null() {
                return null_mut();
            }
            if color(root.right) == Black
                && !root.right.is_null()
                && color((*root.right).left) == Black
            {
                root = move_red_right(root);
            }
            if root.key == key {
                root.key = min(&mut *root.right).key;
                root.right = remove_min(&mut *root.right);
            } else if !root.right.is_null() {
                root.right = remove(&mut *root.right, key);
            }
            root.update();
        }
    }
    fix_up(root)
}

unsafe fn min(root: &mut Node) -> &mut Node {
    if let Some(left) = root.left.as_mut() {
        min(left)
    } else {
        root
    }
}

unsafe fn move_red_left(mut root: &mut Node) -> &mut Node {
    color_flip(root);
    if color((*root.right).left) == Color::Red {
        root.right = rotate_right(&mut *root.right);
        root.update();
        root = rotate_left(root);
        color_flip(root);
    }
    root
}

unsafe fn move_red_right(mut root: &mut Node) -> &mut Node {
    color_flip(root);
    if color((*root.left).left) == Red {
        root = rotate_right(root);
        color_flip(root);
    }
    root
}

unsafe fn fix_up(mut root: &mut Node) -> &mut Node {
    if color(root.left) == Black && color(root.right) == Red {
        root = rotate_left(root);
    }
    if color(root.left) == Red && color((*root.left).left) == Red {
        root = rotate_right(root);
    }
    if color(root.left) == Red && color(root.right) == Red {
        color_flip(root);
    }
    root
}

unsafe fn remove_min(mut root: &mut Node) -> *mut Node {
    if root.left.is_null() {
        let _ = Box::from_raw(root);
        return null_mut();
    }
    if color(root.left) == Black && color((*root.left).left) == Black {
        root = move_red_left(root);
    }
    root.left = remove_min(&mut *root.left);
    root.update();
    fix_up(root)
}

unsafe fn color_flip(root: &mut Node) {
    root.color = root.color.other();
    (*root.left).color = (*root.left).color.other();
    (*root.right).color = (*root.right).color.other();
}

#[allow(dead_code)]
unsafe fn dump(root: *const Node) -> String {
    unsafe fn dump_impl(root: *const Node, out: &mut String) {
        if let Some(root) = root.as_ref() {
            use std::fmt::Write as _;
            out.push('(');
            dump_impl(root.left, out);
            write!(
                out,
                "\x1b[0;{}m{}\x1b[m",
                match root.color {
                    Red => "31",
                    Black => "30",
                },
                root.key,
            )
            .unwrap();
            dump_impl(root.right, out);
            out.push(')');
        }
    }
    let mut out = String::new();
    dump_impl(root, &mut out);
    out
}

#[allow(dead_code)]
fn is_valid(root: *const Node) -> bool {
    unsafe fn try_bh(root: *const Node) -> Option<usize> {
        let Some(root) = root.as_ref() else {
            return Some(0);
        };
        if root.color == Red && color(root.right) == Red {
            return None;
        }
        let left = try_bh(root.left)?;
        let right = try_bh(root.right)?;
        if left != right {
            return None;
        }
        if root.len
            == 1 + root.left.as_ref().map_or(0, |left| left.len)
                + root.right.as_ref().map_or(0, |right| right.len)
        {
            Some(left + usize::from(root.color == Black))
        } else {
            None
        }
    }
    unsafe { color(root) == Black && try_bh(root).is_some() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[test]
    fn test_insert_remove() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..100 {
            let q = 1000;
            let insert_ratio = rng.random_range(0.0..=1.0);
            let key_lim = 100;
            let mut tree = Rbtree { root: null_mut() };
            for _ in 0..q {
                let die = rng.random_range(0.0..=1.0);
                let key = rng.random_range(0..key_lim);
                if die <= insert_ratio {
                    eprintln!("Insert {key}");
                    tree.insert(key);
                } else {
                    eprintln!("Remove {key}");
                    tree.remove(key);
                }
                eprintln!("tree = {}", unsafe { dump(tree.root) });
                eprintln!();
                assert!(is_valid(tree.root));
            }
        }
    }
}
