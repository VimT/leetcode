//! 验证二叉搜索树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn is_valid_bst(root: Node) -> bool {
    fn inner(root: Node, lower: i64, upper: i64) -> bool {
        return match root {
            Some(node) => {
                let val = (*node).borrow().val as i64;
                if val <= lower || val >= upper { return false; }
                inner((*node).borrow().left.clone(), lower, val) && inner((*node).borrow().right.clone(), val, upper)
            }
            None => true
        };
    }
    inner(root, i64::MIN, i64::MAX)
}


fn main() {
    assert_eq!(is_valid_bst(tree![2,1,3]), true);
    assert_eq!(is_valid_bst(tree![5,1,4,null,null,3,6]), false);
}
