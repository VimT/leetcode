//! 二叉树的锯齿形层序遍历

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() { return vec![]; }
    let mut stack = VecDeque::new();
    stack.push_back(root);
    let mut ans = vec![];
    let mut is_reverse = false;
    while !stack.is_empty() {
        let size = stack.len();
        let mut tmp = vec![];
        let mut new_stack = VecDeque::new();
        if is_reverse {
            for _ in 0..size {
                let n = stack.pop_back().unwrap();
                tmp.push(n.as_ref().unwrap().borrow().val);
                if n.as_ref().unwrap().borrow().right.is_some() {
                    new_stack.push_back(n.as_ref().unwrap().borrow().right.clone());
                }
                if n.as_ref().unwrap().borrow().left.is_some() {
                    new_stack.push_back(n.as_ref().unwrap().borrow().left.clone());
                }
            }
        } else {
            for _ in 0..size {
                let n = stack.pop_back().unwrap();
                tmp.push(n.as_ref().unwrap().borrow().val);
                if n.as_ref().unwrap().borrow().left.is_some() {
                    new_stack.push_back(n.as_ref().unwrap().borrow().left.clone());
                }
                if n.as_ref().unwrap().borrow().right.is_some() {
                    new_stack.push_back(n.as_ref().unwrap().borrow().right.clone());
                }
            }
        }
        ans.push(tmp);
        is_reverse = !is_reverse;
        stack = new_stack;
    }

    ans
}

fn main() {
    assert_eq!(zigzag_level_order(tree![3,9,20,null,null,15,7]), vec![vec![3], vec![20, 9], vec![15, 7]]);
    assert_eq!(zigzag_level_order(tree![1]), [[1]]);
    assert_eq!(zigzag_level_order(tree![]).is_empty(), true);
}
