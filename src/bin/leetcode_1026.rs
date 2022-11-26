//! 节点与其祖先之间的最大差值

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cur: &mut Vec<i32>, result: &mut i32) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        for &pre in cur.iter() {
            if (pre - node.val).abs() > *result {
                *result = (pre - node.val).abs();
            }
        }
        cur.push(node.val);
        if node.left.is_some() { dfs(node.left.clone(), cur, result); }
        if node.right.is_some() { dfs(node.right.clone(), cur, result); }
        cur.pop();
    }
    let mut result = 0;
    dfs(root, &mut vec![], &mut result);
    result
}

fn main() {
    assert_eq!(max_ancestor_diff(tree![8,3,10,1,6,null,14,null,null,4,7,13]), 7);
    assert_eq!(max_ancestor_diff(tree![1,null,2,null,0,3]), 3);
}
