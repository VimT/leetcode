//! 感染二叉树需要的总时间

use leetcode::treenode::{leetcode_tree, TreeNode};
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
    // 返回子树高度，和距离start的高度
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, start: i32, result: &mut i32) -> (i32, Option<i32>) {
        if root.is_none() { return (0, None); }
        let node = root.as_ref().unwrap().borrow();
        let (left_height, left_start) = dfs(node.left.clone(), start, result);
        let (right_height, right_start) = dfs(node.right.clone(), start, result);
        let height = left_height.max(right_height) + 1;
        if node.val == start {
            *result = (*result).max(left_height).max(right_height);
            return (height, Some(1));
        }
        if let Some(left_start) = left_start {
            *result = (*result).max(left_start + right_height);
            return (height, Some(left_start + 1));
        }
        if let Some(right_start) = right_start {
            *result = (*result).max(right_start + left_height);
            return (height, Some(right_start + 1));
        }
        return (height, None);
    }
    let mut result = 0;
    dfs(root, start, &mut result);
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32) {
        assert_eq!(func(tree![1,5,3,null,4,10,6,9,2], 3), 4);
        assert_eq!(func(tree![1], 1), 0);
    }
    test(amount_of_time);
}
