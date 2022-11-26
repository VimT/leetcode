//! 二叉搜索树的范围和

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, result: &mut i32) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        if low < node.val {
            dfs(node.left.clone(), low, high, result);
        }
        if node.val >= low && node.val <= high {
            *result += node.val;
        }
        if node.val < high {
            dfs(node.right.clone(), low, high, result);
        }
    }
    let mut result = 0;
    dfs(root, low, high, &mut result);
    result
}


fn main() {
    assert_eq!(range_sum_bst(tree![10,5,15,3,7,null,18], 7, 15), 32);
    assert_eq!(range_sum_bst(tree![10,5,15,3,7,13,18,1,null,6], 6, 10), 23);
}
