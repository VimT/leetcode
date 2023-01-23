//! 删除给定值的叶子节点

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() { return root; }
        let left = dfs(root.as_ref().unwrap().borrow().left.clone(), target);
        let right = dfs(root.as_ref().unwrap().borrow().right.clone(), target);
        if root.as_ref().unwrap().borrow().val == target && left.is_none() && right.is_none() {
            return None;
        } else {
            root.as_ref().unwrap().borrow_mut().left = left;
            root.as_ref().unwrap().borrow_mut().right = right;
        }
        root
    }
    dfs(root, target)
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>>) {
        assert_eq!(func(tree![1,2,3,2,null,2,4], 2), tree![1,null,3,null,4]);
        assert_eq!(func(tree![1,3,3,3,2], 3), tree![1,3,null,null,2]);
        assert_eq!(func(tree![1,2,null,2,null,2], 2), tree![1]);
    }
    test(remove_leaf_nodes);
}
