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
