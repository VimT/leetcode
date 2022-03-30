//! 删除二叉搜索树中的节点


use std::cell::RefCell;
use std::rc::Rc;

use leetcode::treenode::{NodeTravel, TreeNode, vec_to_tree};

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_some() {
        let node = root.as_ref().unwrap();
        if node.borrow().val == key {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                return None;
            } else if node.borrow().right.is_some() {
                let mut p = node.borrow().right.clone();
                while p.as_ref().unwrap().borrow().left.is_some() {
                    let tmp = p.as_ref().unwrap().borrow().left.clone();
                    p = tmp;
                }
                let val = p.as_ref().unwrap().borrow().val;
                node.borrow_mut().val = val;
                let right = delete_node(node.borrow().right.clone(), val);
                node.borrow_mut().right = right;
            } else {
                let mut p = node.borrow().left.clone();
                while p.as_ref().unwrap().borrow().right.is_some() {
                    let tmp = p.as_ref().unwrap().borrow().right.clone();
                    p = tmp;
                }
                let val = p.as_ref().unwrap().borrow().val;
                node.borrow_mut().val = val;
                let left = delete_node(node.borrow().left.clone(), val);
                node.borrow_mut().left = left;
            }
        } else if node.borrow().val > key {
            let lego = delete_node(node.borrow().left.clone(), key);
            node.borrow_mut().left = lego
        } else {
            let right = delete_node(node.borrow().right.clone(), key);
            node.borrow_mut().right = right
        }
    }
    root
}

fn main() {
    assert_eq!(NodeTravel(delete_node(vec_to_tree(vec![5, 3, 2, 0, 0, 4, 0, 0, 6, 0, 7]), 3)).preorder(), vec![5, 4, 2, 6, 7]);
    assert_eq!(NodeTravel(delete_node(vec_to_tree(vec![5, 3, 2, 0, 0, 4, 0, 0, 6, 0, 7]), 0)).preorder(), vec![5, 3, 2, 4, 6, 7]);
    assert_eq!(NodeTravel(delete_node(vec_to_tree(vec![]), 0)).preorder(), vec![]);
}
