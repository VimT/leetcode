//! 剑指 Offer II 053. 二叉搜索树中的中序后继

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::treenode::{TreeNode, vec_to_tree};

pub fn inorder_successor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn inner(root: Option<Rc<RefCell<TreeNode>>>, target: i32, result: &mut Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            if node.borrow().val == target {
                if node.borrow().right.is_some() {
                    let mut right = node.borrow().right.clone();
                    while right.as_ref().unwrap().borrow().left.is_some() {
                        let tmp = right.as_ref().unwrap().borrow().left.clone();
                        right = tmp;
                    }
                    *result = right.clone();
                }
                return true;
            } else if node.borrow().val < target {
                return inner(node.borrow().right.clone(), target, result);
            } else {
                let found = inner(node.borrow().left.clone(), target, result);
                if found && result.is_none() {
                    *result = Some(node.clone());
                }
            }
        }
        false
    }
    let mut result = None;
    inner(root, p.as_ref().unwrap().borrow().val, &mut result);
    result
}

pub fn inorder_successor_best(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut cur = root.clone();
    let mut result = None;
    let target = p.as_ref().unwrap().borrow().val;
    while cur.is_some() {
        if cur.as_ref().unwrap().borrow().val > target {
            result = cur.clone();
            let tmp = cur.as_ref().unwrap().borrow().left.clone();
            cur = tmp;
        } else {
            let tmp = cur.as_ref().unwrap().borrow().right.clone();
            cur = tmp;
        }
    }
    result
}

fn main() {
    println!("{:?}", inorder_successor_best(vec_to_tree(vec![2, 0, 3]), Some(Rc::new(RefCell::new(TreeNode::new(2))))));
}
