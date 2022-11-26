//! 两数之和 IV - 输入 BST


use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    fn find(root: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
        let mut p = root.clone();
        while p.is_some() {
            if p.as_ref().unwrap().borrow().val == target { return true; } else if p.as_ref().unwrap().borrow().val > target {
                let tmp = p.as_ref().unwrap().borrow().left.clone();
                p = tmp;
            } else {
                let tmp = p.as_ref().unwrap().borrow().right.clone();
                p = tmp;
            }
        }
        false
    }
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, node: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if let Some(node) = node.as_ref() {
            let node = node.borrow();
            if node.val * 2 != k && find(root, k - node.val) { return true; };
            if node.left.is_some() { if dfs(root, node.left.clone(), k) { return true; } }
            if node.right.is_some() { if dfs(root, node.right.clone(), k) { return true; } }
        }
        false
    }
    dfs(&root, root.clone(), k)
}

fn main() {
    assert_eq!(find_target(tree![2,1,3], 4), true);
    assert_eq!(find_target(tree![1], 2), false);
    assert_eq!(find_target(tree![5,3,6,2,4,null,7], 9), true);
    assert_eq!(find_target(tree![5,3,6,2,4,null,7], 28), false);
}
