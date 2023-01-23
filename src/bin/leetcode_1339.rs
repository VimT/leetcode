//! 分裂二叉树的最大乘积

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }
        let node = root.as_ref().unwrap().borrow();
        node.val + dfs1(node.left.clone()) + dfs1(node.right.clone())
    }
    fn dfs2(root: Option<Rc<RefCell<TreeNode>>>, total: i32, result: &mut i64) -> i32 {
        if root.is_none() { return 0; }
        let node = root.as_ref().unwrap().borrow();
        let left = dfs2(node.left.clone(), total, result);
        let right = dfs2(node.right.clone(), total, result);
        *result = (*result).max((total - left) as i64 * left as i64);
        *result = (*result).max((total - right) as i64 * right as i64);
        node.val + left + right
    }
    let mut result = 0;
    let sum = dfs1(root.clone());
    dfs2(root, sum, &mut result);
    (result % (1e9 as i64 + 7)) as i32
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![1,2,3,4,5,6]), 110);
        assert_eq!(func(tree![1,null,2,3,4,null,null,5,6]), 90);
    }
    test(max_product);
}
