//! 翻转等价二叉树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root1.is_none() && root2.is_none() {
            return true;
        }
        if root1.is_some() ^ root2.is_some() {
            return false;
        }
        let n1 = root1.as_ref().unwrap().borrow();
        let n2 = root2.as_ref().unwrap().borrow();
        if n1.val != n2.val {
            return false;
        }
        (dfs(n1.left.clone(), n2.left.clone()) && dfs(n1.right.clone(), n2.right.clone())) ||
            (dfs(n1.left.clone(), n2.right.clone()) && dfs(n1.right.clone(), n2.left.clone()))
    }
    dfs(root1, root2)
}


fn main() {
    assert_eq!(flip_equiv(tree![1,2,3,4,5,6,null,null,null,7,8], tree![1,3,2,null,6,4,5,null,null,null,null,8,7]), true);
    assert_eq!(flip_equiv(tree![], tree![]), true);
    assert_eq!(flip_equiv(tree![], tree![1]), false);
}
