//! 从二叉搜索树到更大和树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cur_sum: &mut i32) {
        if root.is_none() { return; }
        dfs(root.as_ref().unwrap().borrow().right.clone(), cur_sum);
        *cur_sum += root.as_ref().unwrap().borrow().val;
        root.as_ref().unwrap().borrow_mut().val = *cur_sum;
        dfs(root.as_ref().unwrap().borrow().left.clone(), cur_sum);
    }
    dfs(root.clone(), &mut 0);
    root
}

fn main() {
    assert_eq!(bst_to_gst(tree![4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]), tree![30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]);
    assert_eq!(bst_to_gst(tree![0,null,1]), tree![1,null,1]);
}
