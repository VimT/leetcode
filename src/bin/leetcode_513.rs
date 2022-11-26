//! 找树左下角的值

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut q = vec![root];
    while !q.is_empty() {
        let mut newq = Vec::with_capacity(q.len() * 2);
        for i in 0..q.len() {
            let node = q[i].as_ref().unwrap().borrow();
            if node.left.is_some() {
                newq.push(node.left.clone());
            }
            if node.right.is_some() {
                newq.push(node.right.clone());
            }
        }
        if newq.is_empty() {
            return q[0].as_ref().unwrap().borrow().val;
        }
        q = newq;
    }
    0
}

fn main() {
    assert_eq!(find_bottom_left_value(tree![2,1,3]), 1);
    assert_eq!(find_bottom_left_value(tree![1,2,3,4,null,5,6,null,null,7]), 7);
}
