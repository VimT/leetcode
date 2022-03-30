//! 求根节点到叶节点数字之和

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() { return 0; }
    let mut q = VecDeque::new();
    q.push_back((root, 0));
    let mut ans = 0;
    while !q.is_empty() {
        let (node, val) = q.pop_front().unwrap();
        let nv = node.as_ref().unwrap().borrow();
        let new_val = val * 10 + nv.val;
        if nv.left.is_none() && nv.right.is_none() {
            ans += new_val;
            continue;
        }
        if nv.left.is_some() {
            q.push_back((nv.left.clone(), new_val));
        }
        if nv.right.is_some() {
            q.push_back((nv.right.clone(), new_val));
        }
    }
    ans
}

fn main() {
    assert_eq!(sum_numbers(tree![1,2,3]), 25);
    assert_eq!(sum_numbers(tree![4,9,0,5,1]), 1026);
}
