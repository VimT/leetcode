//! 前序遍历构造二叉搜索树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn inner(preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 { return None; }
        let val = preorder[0];
        let mut node = TreeNode::new(val);
        let mut idx = 1;
        while idx < preorder.len() && preorder[idx] < val {
            idx += 1;
        }
        node.left = inner(&preorder[1..idx]);
        node.right = inner(&preorder[idx..]);
        Some(Rc::new(RefCell::new(node)))
    }
    inner(&preorder)
}

fn main() {
    assert_eq!(bst_from_preorder(vec![8, 5, 1, 7, 10, 12]), tree![8,5,10,1,7,null,12]);
    assert_eq!(bst_from_preorder(vec![1, 3]), tree![1,null,3]);
}
