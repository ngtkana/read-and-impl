use std::ptr;

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
