//! 奇偶树


use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.as_ref().unwrap().borrow().val & 1 == 0 { return false; }
    let mut q = vec![root];
    let mut level = 0;
    while !q.is_empty() {
        let mut newq = vec![];
        let mut prev = if level & 1 == 1 { i32::MAX } else { i32::MIN };
        for i in 0..q.len() {
            let node = q[i].as_ref().unwrap().borrow();
            if (node.val & 1 == level & 1) || (level & 1 == 1 && node.val >= prev) || (level & 1 == 0 && node.val <= prev) { return false; }
            if node.left != None {
                newq.push(node.left.clone());
            }
            if node.right != None {
                newq.push(node.right.clone());
            }
            prev = node.val;
        }
        level += 1;
        q = newq;
    }
    true
}

fn main() {
    assert_eq!(is_even_odd_tree(tree![1,10,4,3,null,7,9,12,8,6,null,null,2]), true);
    assert_eq!(is_even_odd_tree(tree![2,12,8,5,9,null,null,18,16]), false);
    assert_eq!(is_even_odd_tree(tree![5,9,1,3,5,7]), false);
    assert_eq!(is_even_odd_tree(tree![5,4,2,3,3,7]), false);
    assert_eq!(is_even_odd_tree(tree![1]), true);
    assert_eq!(is_even_odd_tree(tree![11,8,6,1,3,9,11,30,20,18,16,12,10,4,2,17]), true);
}

