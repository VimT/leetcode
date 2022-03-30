//! 从根到叶的二进制数之和

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cur: &mut Vec<i32>, result: &mut i32) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        cur.push(node.val);
        if node.left.is_none() && node.right.is_none() {
            let len = cur.len();
            let mut num = 0;
            for i in 0..len {
                num |= cur[i] << (len - i - 1);
            }
            *result += num;
        } else {
            if node.left.is_some() {
                dfs(node.left.clone(), cur, result);
            }
            if node.right.is_some() {
                dfs(node.right.clone(), cur, result);
            }
        }
        cur.pop();
    }
    let mut result = 0;
    dfs(root, &mut vec![], &mut result);
    result
}

fn main() {
    assert_eq!(sum_root_to_leaf(tree![1,0,1,0,1,0,1]), 22);
    assert_eq!(sum_root_to_leaf(tree![0]), 0);
}
