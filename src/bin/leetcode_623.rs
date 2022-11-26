//! 在二叉树中增加一行


use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if depth == 1 {
        let mut node = TreeNode::new(val);
        node.left = root;
        return Some(Rc::new(RefCell::new(node)));
    }
    let rootclone = root.clone();
    let mut q = vec![root];
    for _ in 1..depth - 1 {
        let mut newq = vec![];
        for i in 0..q.len() {
            let node = q[i].as_ref().unwrap().borrow();
            if node.left.is_some() { newq.push(node.left.clone()); }
            if node.right.is_some() { newq.push(node.right.clone()); }
        }
        q = newq;
    }
    for i in 0..q.len() {
        let node = q[i].as_ref().unwrap();
        let ori_left = node.borrow_mut().left.take();
        let ori_right = node.borrow_mut().right.take();
        let mut new_left = TreeNode::new(val);
        new_left.left = ori_left;
        let mut new_right = TreeNode::new(val);
        new_right.right = ori_right;
        node.borrow_mut().left = Some(Rc::new(RefCell::new(new_left)));
        node.borrow_mut().right = Some(Rc::new(RefCell::new(new_right)));
    }
    rootclone
}

fn main() {
    assert_eq!(add_one_row(tree![4,2,6,3,1,5], 1, 2), tree![4,1,1,2,null,null,6,3,1,5]);
    assert_eq!(add_one_row(tree![4,2,null,3,1], 1, 3), tree![4,2,null,1,1,3,null,null,1]);
}
