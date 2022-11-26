//! 从中序与后序遍历序列构造二叉树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn inner(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let len = inorder.len();
        if len == 0 { return None; }
        let mid_val = *postorder.last().unwrap();
        let mut node = TreeNode::new(mid_val);
        let mut in_idx = 0;
        while inorder[in_idx] != mid_val { in_idx += 1; }
        node.left = inner(&inorder[..in_idx], &postorder[..in_idx]);
        node.right = inner(&inorder[in_idx + 1..], &postorder[in_idx..postorder.len() - 1]);
        Some(Rc::new(RefCell::new(node)))
    }
    inner(&inorder, &postorder)
}

fn main() {
    assert_eq!(build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]), tree![3,9,20,null,null,15,7]);
    assert_eq!(build_tree(vec![-1], vec![-1]), tree![-1]);
}
