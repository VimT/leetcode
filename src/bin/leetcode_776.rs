//! 拆分二叉搜索树

use leetcode::treenode::{leetcode_tree, TreeNode};
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn split_bst(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if root.is_none() { return vec![None, None]; }
    let val = root.as_ref().unwrap().borrow().val;
    if val <= target {
        let mut bns = split_bst(root.as_ref().unwrap().borrow().right.clone(), target);
        root.as_ref().unwrap().borrow_mut().right = bns[0].take();
        bns[0] = root;
        bns
    } else {
        let mut bns = split_bst(root.as_ref().unwrap().borrow().left.clone(), target);
        root.as_ref().unwrap().borrow_mut().left = bns[1].take();
        bns[1] = root;
        bns
    }
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>>) {
        assert_eq!(func(tree![4,2,6,1,3,5,7], 2), vec![tree![2,1], tree![4,3,6,null,null,5,7]]);
        assert_eq!(func(tree![1], 1), vec![tree![1], tree![]]);
    }
    test(split_bst);
}
