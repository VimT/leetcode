//! 二叉树中的最大路径和

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

/// 精髓：node.borrow().val + max(0, left) + max(0, right)， 节点自身值
/// node.borrow().val + max(0, max(left, right))  可用来递归，可连接父节点的 本节点最大值
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    pub fn inner(node: &Option<Rc<RefCell<TreeNode>>>, current_max: &mut i32) -> i32 {
        if node.is_none() { return 0; }
        let node = node.clone().unwrap();
        let left = inner(&node.borrow().left, current_max);
        let right = inner(&node.borrow().right, current_max);
        let lmr = node.borrow().val + 0.max(left) + 0.max(right);
        let ret = node.borrow().val + 0.max(left.max(right));
        *current_max = (*current_max).max(lmr);
        return ret;
    }
    let mut ret = root.clone().unwrap_or(Rc::new(RefCell::new(TreeNode::new(0)))).borrow().val;
    inner(&root, &mut ret);
    return ret;
}

fn main() {
    assert_eq!(max_path_sum(tree![1,2,3]), 6);
    assert_eq!(max_path_sum(tree![-10,9,20,null,null,15,7]), 42);
}
