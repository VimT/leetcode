//! 二叉树的右视图

use leetcode::treenode::{leetcode_tree, TreeNode};
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() { return vec![]; }
    let mut q = VecDeque::new();
    q.push_back(root);
    let mut ans = vec![];
    while !q.is_empty() {
        let len = q.len();
        for i in 0..len {
            let node = q.pop_front().unwrap();
            let unode = node.as_ref().unwrap().borrow();
            if unode.left.is_some() {
                q.push_back(unode.left.clone());
            }
            if unode.right.is_some() {
                q.push_back(unode.right.clone());
            }
            if i == len - 1 {
                ans.push(unode.val);
            }
        }
    }
    ans
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>) {
        assert_eq!(func(tree![1,2,3,null,5,null,4]), vec![1, 3, 4]);
        assert_eq!(func(tree![1,null,3]), vec![1, 3]);
        assert_eq!(func(tree![]), vec![]);
    }
    test(right_side_view);
}
