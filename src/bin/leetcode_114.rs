//! 二叉树展开为链表

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut node = root.clone();
    while node.is_some() {
        if node.as_ref().unwrap().borrow().left.is_some() {
            let mut n = node.as_ref().unwrap().borrow_mut();
            if n.right.is_some() {
                let mut last_node = n.left.clone();
                while last_node.as_ref().unwrap().borrow().right.is_some() {
                    let tmp = last_node.as_ref().unwrap().borrow().right.clone();
                    last_node = tmp;
                }
                last_node.as_ref().unwrap().borrow_mut().right = n.right.take();
            }
            n.right = n.left.take();
        }
        let tmp = node.as_ref().unwrap().borrow().right.clone();
        node = tmp;
    }
}

fn main() {
    fn test(func: fn(root: &mut Option<Rc<RefCell<TreeNode>>>)) {
        let help = |mut root: Option<Rc<RefCell<TreeNode>>>| {
            func(&mut root);
            root
        };
        assert_eq!(help(tree![1,2,5,3,4,null,6]), tree![1,null,2,null,3,null,4,null,5,null,6]);
        assert_eq!(help(tree![]), tree![]);
        assert_eq!(help(tree![0]), tree![0]);
    }
    test(flatten);
}
