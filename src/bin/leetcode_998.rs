//! 最大二叉树 II

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn insert_into_max_tree(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() { return Some(Rc::new(RefCell::new(TreeNode::new(val)))); }
        if val > root.as_ref().unwrap().borrow().val {
            let mut new = TreeNode::new(val);
            new.left = root;
            return Some(Rc::new(RefCell::new(new)));
        } else {
            let new_right = dfs(root.as_ref().unwrap().borrow().right.clone(), val);
            root.as_ref().unwrap().borrow_mut().right = new_right;
            root
        }
    }
    dfs(root, val)
}


fn main() {
    assert_eq!(insert_into_max_tree(tree![4,1,3,null,null,2], 5), tree![5,4,null,1,3,null,null,2]);
    assert_eq!(insert_into_max_tree(tree![5,2,4,null,1], 3), tree![5,2,4,null,1,null,3]);
    assert_eq!(insert_into_max_tree(tree![5,2,3,null,1], 4), tree![5,2,4,null,1,3]);
}
