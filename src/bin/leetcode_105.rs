//! 从前序与中序遍历序列构造二叉树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Node {
    fn inner(pre: &[i32], inorder: &[i32]) -> Node {
        if pre.len() == 0 { return None; }
        let root_val = pre[0];
        let mut root = TreeNode::new(root_val);
        let mut root_pos = 0;
        for i in 0..inorder.len() {
            if inorder[i] == root_val {
                root_pos = i;
                break;
            }
        }
        let in_left = &inorder[..root_pos];
        let pre_left = &pre[1..1 + in_left.len()];
        root.left = inner(pre_left, in_left);
        root.right = inner(&pre[1 + in_left.len()..], &inorder[root_pos + 1..]);
        Some(Rc::new(RefCell::new(root)))
    }
    inner(&preorder, &inorder)
}

fn main() {
    assert_eq!(build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]), tree![3,9,20,null,null,15,7]);
    assert_eq!(build_tree(vec![-1], vec![-1]), tree![-1]);
}
