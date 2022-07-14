//! 统计同值子树

use leetcode::treenode::{leetcode_tree, TreeNode};
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> bool {
        if root.is_none() { return true; }
        let node = root.as_ref().unwrap().borrow();
        let le = dfs(node.left.clone(), result);
        let re = dfs(node.right.clone(), result);
        if le && re {
            if node.left.is_some() && node.val != node.left.as_ref().unwrap().borrow().val {
                return false;
            }
            if node.right.is_some() && node.val != node.right.as_ref().unwrap().borrow().val {
                return false;
            }
            *result += 1;
            return true;
        }
        false
    }
    let mut result = 0;
    dfs(root, &mut result);
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![5,1,5,5,5,null,5]), 4);
        assert_eq!(func(tree![]), 0);
        assert_eq!(func(tree![5,5,5,5,5,null,5]), 6);
    }
    test(count_unival_subtrees);
}
