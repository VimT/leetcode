//! 二叉树剪枝

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
        if root.is_none() { return (None, 0); }
        let node = root.as_ref().unwrap().borrow();
        let (left, left_val) = dfs(node.left.clone());
        let (right, right_val) = dfs(node.right.clone());
        if node.val.max(left_val).max(right_val) == 0 {
            return (None, 0);
        }
        drop(node);
        root.as_ref().unwrap().borrow_mut().left = left;
        root.as_ref().unwrap().borrow_mut().right = right;
        (root, 1)
    }
    dfs(root).0
}


fn main() {
    assert_eq!(prune_tree(tree![1,null,0,0,1]), tree![1,null,0,null,1]);
    assert_eq!(prune_tree(tree![1,0,1,0,0,0,1]), tree![1,null,1,null,1]);
    assert_eq!(prune_tree(tree![1,1,0,1,1,0,1,0]), tree![1,1,0,1,1,null,1]);
}
