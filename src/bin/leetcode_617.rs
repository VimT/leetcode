//! 合并二叉树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root1.is_none() && root2.is_none() {
        return None;
    }
    if root1.is_none() { return root2; }
    if root2.is_none() { return root1; }
    let mut node = TreeNode::new(root1.as_ref().unwrap().borrow().val + root2.as_ref().unwrap().borrow().val);
    node.left = merge_trees(root1.as_ref().unwrap().borrow().left.clone(), root2.as_ref().unwrap().borrow().left.clone());
    node.right = merge_trees(root1.as_ref().unwrap().borrow().right.clone(), root2.as_ref().unwrap().borrow().right.clone());
    Some(Rc::new(RefCell::new(node)))
}

fn main() {
    assert_eq!(merge_trees(tree![1,3,2,5], tree![2,1,3,null,4,null,7]), tree![3,4,5,5,4,null,7]);
    assert_eq!(merge_trees(tree![1], tree![1,2]), tree![2,2]);
}
