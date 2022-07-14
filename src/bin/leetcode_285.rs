//! 二叉搜索树中的中序后继

use leetcode::treenode::{leetcode_tree, TreeNode};
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

/// 非递归版 中序遍历
pub fn inorder_successor(mut root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut s = vec![];
    let pval = p.as_ref().unwrap().borrow().val;
    let mut found = false;
    while root.is_some() || !s.is_empty() {
        while let Some(node) = root {
            s.push(node.clone());
            root = node.borrow().left.clone();
        }
        let node = s.pop().unwrap();
        if found {
            return Some(node);
        }
        root = node.borrow().right.clone();
        if node.borrow().val == pval {
            found = true;
        }
    }
    None
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>) {
        assert_eq!(func(tree![2,1,3], tree![1]), tree![2,1,3]);
        assert_eq!(func(tree![5,3,6,2,4,null,null,1], tree![6]), tree![]);
    }
    test(inorder_successor);
}
