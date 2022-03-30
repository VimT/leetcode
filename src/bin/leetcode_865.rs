//! 具有所有最深节点的最小子树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
        if root.is_none() {
            return (None, 0);
        }
        let node = root.as_ref().unwrap().borrow();
        let (left, left_height) = dfs(node.left.clone());
        let (right, right_height) = dfs(node.right.clone());
        drop(node);
        return if left_height == right_height {
            (root, left_height + 1)
        } else if left_height > right_height {
            (left, left_height + 1)
        } else {
            (right, right_height + 1)
        };
    }
    dfs(root).0
}


fn main() {
    assert_eq!(subtree_with_all_deepest(tree![3,5,1,6,2,0,8,null,null,7,4]), tree![2,7,4]);
    assert_eq!(subtree_with_all_deepest(tree![1]), tree![1]);
    assert_eq!(subtree_with_all_deepest(tree![0,1,3,null,2]), tree![2]);
}
