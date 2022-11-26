//! 二叉树的边界

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn boundary_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    if root.is_none() { return result; }
    result.push(root.as_ref().unwrap().borrow().val);
    if root.as_ref().unwrap().borrow().left.is_none() && root.as_ref().unwrap().borrow().right.is_none() {
        return result;
    }
    let mut p = root.as_ref().unwrap().borrow().left.clone();
    while let Some(node) = p {
        let node = node.borrow();
        if node.left.is_none() && node.right.is_none() {
            break;
        }
        result.push(node.val);
        if node.left.is_some() {
            p = node.left.clone();
            continue;
        }
        p = node.right.clone();
    }

    let mut p = root.as_ref().unwrap().borrow().right.clone();
    let mut right = vec![];
    while let Some(node) = p {
        let node = node.borrow();
        if node.left.is_none() && node.right.is_none() {
            break;
        }
        right.push(node.val);
        if node.right.is_some() {
            p = node.right.clone();
            continue;
        }
        p = node.left.clone();
    }
    fn get_leaf(root: Option<Rc<RefCell<TreeNode>>>, leaf: &mut Vec<i32>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        get_leaf(node.left.clone(), leaf);
        if node.left.is_none() && node.right.is_none() {
            leaf.push(node.val);
        }
        get_leaf(node.right.clone(), leaf);
    }
    let mut leaf = vec![];
    get_leaf(root, &mut leaf);
    result.extend_from_slice(&leaf);
    right.reverse();
    result.extend_from_slice(&right);

    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>) {
        assert_eq!(func(tree![1]), vec![1]);
        assert_eq!(func(tree![1,null,2,3,4]), vec![1, 3, 4, 2]);
        assert_eq!(func(tree![1,2,3,4,5,6,null,null,null,7,8,9,10]), vec![1, 2, 4, 7, 8, 9, 10, 6, 3]);
    }
    test(boundary_of_binary_tree);
}
