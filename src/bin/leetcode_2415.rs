//! 反转二叉树的奇数层

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut v = vec![root.clone()];
    let mut level = 0;
    while !v.is_empty() {
        if level & 1 == 1 {
            let next_val: Vec<i32> = v.iter().map(|x| x.as_ref().unwrap().borrow().val).collect();
            for (node, val) in v.iter().zip(next_val.into_iter().rev()) {
                node.as_ref().unwrap().borrow_mut().val = val;
            }
        }
        let mut next = Vec::with_capacity(v.len() * 2);
        for node in &v {
            if let Some(node) = node {
                if node.borrow().left.is_some() {
                    next.push(node.borrow().left.clone());
                    next.push(node.borrow().right.clone());
                }
            }
        }
        v = next;
        level += 1;
    }
    root
}


fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>) {
        assert_eq!(func(tree![2,3,5,8,13,21,34]), tree![2,5,3,8,13,21,34]);
        assert_eq!(func(tree![7,13,11]), tree![7,11,13]);
        assert_eq!(func(tree![0,1,2,0,0,0,0,1,1,1,1,2,2,2,2]), tree![0,2,1,0,0,0,0,2,2,2,2,1,1,1,1]);
    }
    test(reverse_odd_levels);
}
