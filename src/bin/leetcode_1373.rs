//! 二叉搜索子树的最大键值和

use leetcode::treenode::TreeNode;
use std::rc::Rc;

use std::cell::RefCell;

pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> (i32, i32, i32) {
        if root.is_none() {
            return (i32::MAX, i32::MIN, 0);
        }
        let node = root.as_ref().unwrap().borrow();
        let (lmn, lmx, lsum) = dfs(node.left.clone(), result);
        let (rmn, rmx, rsum) = dfs(node.right.clone(), result);
        if lmx < node.val && node.val < rmn {
            (*result) = (*result).max(lsum + rsum + node.val);
            (lmn.min(rmn).min(node.val), lmx.max(rmx).max(node.val), lsum + rsum + node.val)
        } else {
            (i32::MIN, i32::MAX, 0)
        }
    }
    let mut result = 0;
    dfs(root, &mut result);
    result
}

fn main() {
    use leetcode::tree;
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![4,3,null,1,2]), 2);
        assert_eq!(func(tree![1,4,3,2,4,2,5,null,null,null,null,null,null,4,6]), 20);
        assert_eq!(func(tree![-4,-2,-5]), 0);
        assert_eq!(func(tree![2,1,3]), 6);
        assert_eq!(func(tree![5,4,8,3,null,6,3]), 7);
    }
    test(max_sum_bst);
}
