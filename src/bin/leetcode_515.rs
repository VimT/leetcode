//! 在每个树行中找最大值

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() { return vec![]; }
    let mut q = vec![root];
    let mut result = vec![];
    while !q.is_empty() {
        let mut newq = vec![];
        let mut max = i32::MIN;
        for i in 0..q.len() {
            let node = q[i].as_ref().unwrap().borrow();
            if node.val > max {
                max = node.val;
            }
            if node.left.is_some() { newq.push(node.left.clone()); }
            if node.right.is_some() { newq.push(node.right.clone()); }
        }
        result.push(max);
        q = newq;
    }
    result
}

fn main() {
    assert_eq!(largest_values(tree![1,3,2,5,3,null,9]), vec![1, 3, 9]);
    assert_eq!(largest_values(tree![1,2,3]), vec![1, 3]);
}
