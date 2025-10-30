use std::{cmp::Ordering, ptr::null_mut};
use Color::{Black, Red};

#[allow(dead_code)]
pub struct Rbtree {
    root: *mut Node,
}

impl Rbtree {
    pub fn insert(&mut self, key: u64) {
        unsafe {
            self.root = insert_impl(self.root, key);
            (*self.root).color = Black;
        }
    }

    pub fn remove(&mut self, key: u64) {
        unsafe {
            if !self.root.is_null() {
                if color((*self.root).left) == Color::Black {
                    (*self.root).color = Red;
                }
                self.root = remove_impl(&mut *self.root, key);
            }
            if !self.root.is_null() {
                (*self.root).color = Black;
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Color {
    Red,
    Black,
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
}

unsafe fn insert_impl(root: *mut Node, key: u64) -> *mut Node {
    let Some(root) = root.as_mut() else {
        return Box::leak(Box::new(Node {
            left: null_mut(),
            right: null_mut(),
            key,
            color: Color::Red,
        }));
    };
    match root.key.cmp(&key) {
        Ordering::Greater => root.left = insert_impl(root.left, key),
        Ordering::Equal => (),
        Ordering::Less => root.right = insert_impl(root.right, key),
    }
    fix_up(root)
}

unsafe fn remove_impl(mut root: &mut Node, key: u64) -> *mut Node {
    match root.key.cmp(&key) {
        Ordering::Greater => {
            let Some(left) = root.left.as_mut() else {
                return root;
            };
            if left.color == Black && color(left.left) == Black {
                root = move_red_left(root);
            }
            root.left = remove_impl(&mut *root.left, key);
        }
        cmp @ (Ordering::Equal | Ordering::Less) => {
            if color(root.left) == Red {
                root = rotate_right(root);
            }
            let Some(right) = root.right.as_mut() else {
                return if cmp == Ordering::Equal {
                    let _ = Box::from_raw(root);
                    null_mut()
                } else {
                    root
                };
            };
            if right.color == Black && color(right.left) == Black {
                root = move_red_right(root);
            }
            if root.key == key {
                let (handle, removed) = remove_min(&mut *root.right);
                root.key = removed;
                root.right = handle;
            } else if !root.right.is_null() {
                root.right = remove_impl(&mut *root.right, key);
            }
        }
    }
    fix_up(root)
}

unsafe fn move_red_left(mut root: &mut Node) -> &mut Node {
    join_two_nodes(root);
    if color((*root.right).left) == Color::Red {
        root.right = rotate_right(&mut *root.right);
        root = rotate_left(root);
        split_four_node(root);
    }
    root
}

unsafe fn move_red_right(mut root: &mut Node) -> &mut Node {
    join_two_nodes(root);
    if color((*root.left).left) == Red {
        root = rotate_right(root);
        split_four_node(root);
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
        split_four_node(root);
    }
    root
}

unsafe fn remove_min(mut root: &mut Node) -> (*mut Node, u64) {
    if root.left.is_null() {
        let removed = Box::from_raw(root).key;
        return (null_mut(), removed);
    }
    if color(root.left) == Black && color((*root.left).left) == Black {
        root = move_red_left(root);
    }
    let (handle, removed) = remove_min(&mut *root.left);
    root.left = handle;
    (fix_up(root), removed)
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

unsafe fn split_four_node(root: &mut Node) {
    root.color = Red;
    (*root.left).color = Black;
    (*root.right).color = Black;
}

unsafe fn join_two_nodes(root: &mut Node) {
    root.color = Black;
    (*root.left).color = Red;
    (*root.right).color = Red;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

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
            Some(left + usize::from(root.color == Black))
        }
        unsafe { color(root) == Black && try_bh(root).is_some() }
    }

    #[test]
    fn test_insert_remove() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..1000 {
            let q = 100;
            let insert_ratio = rng.random_range(0.0..=1.0);
            let key_lim = 30;
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
