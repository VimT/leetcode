//! 二叉树最大宽度

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((root, 0, 0usize));
    let mut result = 1;
    let mut cur_depth = 0;
    let mut left = 0;
    while !q.is_empty() {
        let (node, depth, pos) = q.pop_front().unwrap();
        if let Some(node) = node.as_ref() {
            let node = node.borrow();
            q.push_back((node.left.clone(), depth + 1, pos * 2));
            q.push_back((node.right.clone(), depth + 1, pos * 2 + 1));
            if cur_depth != depth {
                cur_depth = depth;
                left = pos;
            }
            result = result.max(pos + 1 - left);
        }
    }
    result as i32
}

fn main() {
    assert_eq!(width_of_binary_tree(tree![1,3,2,5,3,null,9]), 4);
    assert_eq!(width_of_binary_tree(tree![1,3,null,5,3]), 2);
    assert_eq!(width_of_binary_tree(tree![1,3,2,5]), 2);
}
