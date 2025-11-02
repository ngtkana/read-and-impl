use std::{cmp::Ordering, ptr::null_mut};
use Color::{Black, Red};

pub struct Rbtree {
    root: *mut Node,
}

impl Default for Rbtree {
    fn default() -> Self {
        Self { root: null_mut() }
    }
}

impl Rbtree {
    pub fn insert(&mut self, key: i64) {
        unsafe { self.root = insert(self.root, key) }
    }

    pub fn remove(&mut self, key: i64) {
        unsafe { self.root = remove(self.root, key) }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Color {
    Red,
    Black,
}

unsafe fn color(x: *const Node) -> Color {
    x.as_ref().map_or(Black, |x| x.color)
}

pub struct Node {
    left: *mut Self,
    right: *mut Self,
    key: i64,
    color: Color,
}

unsafe fn insert(mut x: *mut Node, key: i64) -> *mut Node {
    x = insert_recurse(x, key);
    (*x).color = Black;
    x
}

unsafe fn insert_recurse(x: *mut Node, key: i64) -> *mut Node {
    let Some(x) = x.as_mut() else {
        return Box::leak(Box::new(Node {
            left: null_mut(),
            right: null_mut(),
            key,
            color: Red,
        }));
    };
    match key.cmp(&x.key) {
        Ordering::Less => x.left = insert_recurse(x.left, key),
        Ordering::Equal => return x,
        Ordering::Greater => x.right = insert_recurse(x.right, key),
    }
    fixup(x)
}

unsafe fn remove(x: *mut Node, key: i64) -> *mut Node {
    let Some(x) = x.as_mut() else {
        return null_mut();
    };
    if color(x.left) == Color::Black {
        x.color = Red;
    }
    let x = remove_recurse(&mut *x, key);
    if let Some(x) = x.as_mut() {
        x.color = Black;
    }
    x
}

unsafe fn remove_recurse(mut x: &mut Node, key: i64) -> *mut Node {
    match key.cmp(&x.key) {
        Ordering::Less => {
            let Some(l) = x.left.as_mut() else {
                return x;
            };
            if l.color == Black && color(l.left) == Black {
                x = move_red_left(x);
            }
            x.left = remove_recurse(&mut *x.left, key);
        }
        Ordering::Equal | Ordering::Greater => {
            if color(x.left) == Red {
                x = rotate_right(x);
            }
            let Some(r) = x.right.as_mut() else {
                return match key.cmp(&x.key) {
                    Ordering::Less => unreachable!(),
                    Ordering::Equal => null_mut(),
                    Ordering::Greater => x,
                };
            };
            if r.color == Black && color(r.left) == Black {
                x = move_red_right(x);
            }
            if x.key == key {
                let removed;
                (x.right, removed) = remove_min(&mut *x.right);
                x.key = (*removed).key;
            } else if !x.right.is_null() {
                x.right = remove_recurse(&mut *x.right, key);
            }
        }
    }
    fixup(x)
}

unsafe fn remove_min(mut x: &mut Node) -> (*mut Node, *mut Node) {
    if x.left.is_null() {
        return (null_mut(), x);
    }
    if color(x.left) == Black && color((*x.left).left) == Black {
        x = move_red_left(x);
    }
    let removed;
    (x.left, removed) = remove_min(&mut *x.left);
    (fixup(x), removed)
}

unsafe fn move_red_left(mut x: &mut Node) -> &mut Node {
    join_two_nodes(x);
    if color((*x.right).left) == Color::Red {
        x.right = rotate_right(&mut *x.right);
        x = rotate_left(x);
        split_four_node(x);
    }
    x
}

unsafe fn move_red_right(mut x: &mut Node) -> &mut Node {
    join_two_nodes(x);
    if color((*x.left).left) == Red {
        x = rotate_right(x);
        split_four_node(x);
    }
    x
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

unsafe fn rotate_right(x: &mut Node) -> &mut Node {
    let y = &mut *x.left;
    x.left = y.right;
    y.right = x;
    y.color = x.color;
    x.color = Red;
    y
}

unsafe fn rotate_left(x: &mut Node) -> &mut Node {
    let y = &mut *x.right;
    x.right = y.left;
    y.left = x;
    y.color = x.color;
    x.color = Red;
    y
}

unsafe fn split_four_node(x: &mut Node) {
    x.color = Red;
    (*x.left).color = Black;
    (*x.right).color = Black;
}

unsafe fn join_two_nodes(x: &mut Node) {
    x.color = Black;
    (*x.left).color = Red;
    (*x.right).color = Red;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::collections::BTreeSet;

    fn is_valid(x: *const Node) -> bool {
        unsafe fn is_valid_recurse(x: *const Node) -> Option<u8> {
            let Some(root) = x.as_ref() else {
                return Some(0);
            };
            if root.color == Red && color(root.right) == Red {
                return None;
            }
            let lbh = is_valid_recurse(root.left)?;
            let rbh = is_valid_recurse(root.right)?;
            if lbh != rbh {
                return None;
            }
            Some(lbh + u8::from(root.color == Black))
        }
        unsafe { color(x) == Black && is_valid_recurse(x).is_some() }
    }

    fn collect(x: *const Node) -> Vec<i64> {
        unsafe fn collect_recurse(x: *const Node, out: &mut Vec<i64>) {
            if let Some(x) = x.as_ref() {
                collect_recurse(x.left, out);
                out.push(x.key);
                collect_recurse(x.right, out);
            }
        }
        let mut out = Vec::new();
        unsafe { collect_recurse(x, &mut out) };
        out
    }

    #[test]
    fn test_insert_remove() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..2000 {
            let q = 100;
            let insert_ratio = rng.random_range(0.0..=1.0);
            let key_lim = 30;
            let mut tree = Rbtree::default();
            let mut btree_set = BTreeSet::new();
            for _ in 0..q {
                let die = rng.random_range(0.0..=1.0);
                let key = rng.random_range(0..key_lim);
                if die <= insert_ratio {
                    eprintln!("Insert {key}");
                    tree.insert(key);
                    btree_set.insert(key);
                } else {
                    eprintln!("Remove {key}");
                    tree.remove(key);
                    btree_set.remove(&key);
                }
                assert!(is_valid(tree.root));
                assert_eq!(
                    collect(tree.root),
                    btree_set.iter().copied().collect::<Vec<_>>().as_slice(),
                );
            }
        }
    }
}
