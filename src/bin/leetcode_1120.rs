//! 子树的最大平均值

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn maximum_average_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut f64) -> (i32, i32) {
        if root.is_none() { return (0, 0); }
        let node = root.as_ref().unwrap().borrow();
        let (lcnt, lsum) = dfs(node.left.clone(), result);
        let (rcnt, rsum) = dfs(node.right.clone(), result);
        *result = (*result).max((lsum + node.val + rsum) as f64 / (lcnt + rcnt + 1) as f64);
        (lcnt + rcnt + 1, lsum + rsum + node.val)
    }
    let mut result = 0f64;
    dfs(root, &mut result);
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> f64) {
        assert_eq!(func(tree![5,6,1]), 6.00000);
        assert_eq!(func(tree![0,null,1]), 1.00000);
    }
    test(maximum_average_subtree);
}
