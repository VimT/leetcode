//! 完全二叉树的节点个数

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut p = root.clone();
    let mut depth = 0;
    while !p.is_none() {
        let tmp = p.as_ref().unwrap().borrow().left.clone();
        p = tmp;
        depth += 1;
    }
    if depth <= 1 {
        return depth as i32;
    }
    let mut left = 1_u64 << (depth - 1);
    let mut right = (1_u64 << depth) - 1;
    while left < right {
        // !! right - left + 1
        let mid = left + (right - left + 1) / 2;
        let mut p = root.clone();
        for i in (0..depth - 1).rev() {
            if mid >> i & 1 == 1 {
                let tmp = p.as_ref().unwrap().borrow().right.clone();
                p = tmp;
            } else {
                let tmp = p.as_ref().unwrap().borrow().left.clone();
                p = tmp;
            }
        }
        if p.is_none() {
            // !!
            right = mid - 1;
        } else {
            left = mid;
        }
    }

    left as i32
}

fn main() {
    assert_eq!(count_nodes(tree![1,2,3,4,5,6]), 6);
    assert_eq!(count_nodes(tree![]), 0);
    assert_eq!(count_nodes(tree![1]), 1);
}
