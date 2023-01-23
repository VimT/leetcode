//! 最接近的二叉搜索树值

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target: f64, min_diff: &mut f64, result: &mut i32) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        let diff = (node.val as f64 - target).abs();
        if diff < *min_diff {
            *min_diff = diff;
            *result = node.val;
        }
        if target > node.val as f64 {
            dfs(node.right.clone(), target, min_diff, result);
        } else {
            dfs(node.left.clone(), target, min_diff, result);
        }
    }
    let mut result = 0;
    let mut min_diff = f64::MAX;
    dfs(root, target, &mut min_diff, &mut result);
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32) {
        assert_eq!(func(tree![4,2,5,1,3], 3.714286), 4);
        assert_eq!(func(tree![1], 4.428571), 1);
    }
    test(closest_value);
}
