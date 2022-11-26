//! 最大 BST 子树

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() { return 0; }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> (i32, i32, bool, i32) {
        let node = root.as_ref().unwrap().borrow();
        let mut min = node.val;
        let mut max = node.val;
        let mut ok = true;
        let mut tree_size = 1;
        if node.left.is_some() {
            let (lmin, lmax, lok, lsize) = dfs(node.left.clone(), result);
            min = min.min(lmin);
            max = max.max(lmax);
            ok = ok && lok && node.val > lmax;
            tree_size += lsize;
        }
        if node.right.is_some() {
            let (rmin, rmax, rok, rsize) = dfs(node.right.clone(), result);
            min = min.min(rmin);
            max = max.max(rmax);
            ok = ok && rok && node.val < rmin;
            tree_size += rsize;
        }
        if ok {
            *result = (*result).max(tree_size);
        }
        (min, max, ok, tree_size)
    }
    let mut result = 0;
    dfs(root, &mut result);
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![10,5,15,1,8,null,7]), 3);
        assert_eq!(func(tree![4,2,7,2,3,5,null,2,null,null,null,null,null,1]), 2);
    }
    test(largest_bst_subtree);
}
