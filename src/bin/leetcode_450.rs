//! 删除二叉搜索树中的节点

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

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
    assert_eq!(delete_node(tree![5,3,6,2,4,null,7], 3), tree![5,4,6,2,null,null,7]);
    assert_eq!(delete_node(tree![5,3,6,2,4,null,7], 0), tree![5,3,6,2,4,null,7]);
    assert_eq!(delete_node(tree![], 0), tree![]);
}
