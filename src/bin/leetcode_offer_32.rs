//! 从上到下打印二叉树 III

use leetcode::treenode::{leetcode_tree, TreeNode};
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() { return vec![]; }
    let mut q = vec![];
    q.push(root);
    let mut result = vec![];
    while !q.is_empty() {
        let mut v: Vec<i32> = q.iter().map(|x| x.as_ref().unwrap().borrow().val).collect();
        if result.len() & 1 == 1 {
            v.reverse();
        }
        result.push(v);
        let mut next = vec![];
        for node in q {
            let v = node.as_ref().unwrap().borrow();
            if v.left.is_some() {
                next.push(v.left.clone());
            }
            if v.right.is_some() {
                next.push(v.right.clone());
            }
        }
        q = next;
    }
    result
}


fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(tree![3,9,20,null,null,15,7]), vec![vec![3], vec![20, 9], vec![15, 7]]);
    }
    test(level_order);
}
