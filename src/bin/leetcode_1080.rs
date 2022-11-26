//! 根到叶路径上的不足节点

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn sufficient_subset(root: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, limit: i32) -> bool {
        let node = root.as_ref().unwrap();
        let val = node.borrow().val;
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return sum + node.borrow().val < limit;
        }
        let mut left_deleted = true;
        let mut right_deleted = true;
        if node.borrow().left.is_some() {
            left_deleted = dfs(node.borrow().left.clone(), sum + val, limit);
        }
        if node.borrow().right.is_some() {
            right_deleted = dfs(node.borrow().right.clone(), sum + val, limit);
        }
        if left_deleted {
            node.borrow_mut().left = None;
        }
        if right_deleted {
            node.borrow_mut().right = None;
        }
        left_deleted && right_deleted
    }
    let deleted = dfs(root.clone(), 0, limit);
    if deleted {
        return None;
    }
    root
}

fn main() {
    assert_eq!(sufficient_subset(tree![1,2,-3,-5,null,4,null], -1), tree![1,null,-3,4]);
    assert_eq!(sufficient_subset(tree![1,2,3,4,-99,-99,7,8,9,-99,-99,12,13,-99,14], 1), tree![1,2,3,4,null,null,7,8,9,null,14]);
    assert_eq!(sufficient_subset(tree![5,4,8,11,null,17,4,7,1,null,null,5,3], 22), tree![5,4,8,11,null,17,4,7,null,null,null,5]);
    assert_eq!(sufficient_subset(tree![5,-6,-6], 0), tree![]);
}
