//! 将二叉搜索树变平衡

use leetcode::treenode::TreeNode;
use std::rc::Rc;

use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;

/// 转成list再转回去。
/// 如果要像avl那样旋转，Node属性需要有高度信息
pub fn balance_bst(root: Node) -> Node {
    fn dfs(root: Node, inorder: &mut Vec<i32>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        dfs(node.left.clone(), inorder);
        inorder.push(node.val);
        dfs(node.right.clone(), inorder);
    }
    let mut arr = vec![];
    dfs(root, &mut arr);
    fn build(arr: &[i32]) -> Node {
        if arr.len() == 0 { return None; }
        let len = arr.len();
        let mid = len / 2;
        let mut node = TreeNode::new(arr[mid]);
        node.left = build(&arr[..mid]);
        node.right = build(&arr[mid + 1..]);
        Some(Rc::new(RefCell::new(node)))
    }
    build(&arr)
}

fn main() {
    use leetcode::tree;
    fn test(func: fn(root: Node) -> Node) {
        assert_eq!(func(tree![1,null,2,null,3,null,4,null,null]), tree![3, 2,4,1]);
        assert_eq!(func(tree![2,1,3]), tree![2,1,3]);
    }
    test(balance_bst);
}
