//! 二叉搜索树中的插入操作

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() { return Some(Rc::new(RefCell::new(TreeNode::new(val)))); }
        let node = root.as_ref().unwrap();
        if node.borrow().val > val {
            let new_left = dfs(node.borrow_mut().left.take(), val);
            node.borrow_mut().left = new_left
        } else {
            let new_right = dfs(node.borrow_mut().right.take(), val);
            node.borrow_mut().right = new_right;
        }
        root
    }
    dfs(root, val)
}


fn main() {
    assert_eq!(insert_into_bst(tree![4,2,7,1,3], 5), tree![4,2,7,1,3,5]);
    assert_eq!(insert_into_bst(tree![40,20,60,10,30,50,70], 25), tree![40,20,60,10,30,50,70,null,null,25]);
    assert_eq!(insert_into_bst(tree![4,2,7,1,3,null,null,null,null,null,null], 5), tree![4,2,7,1,3,5]);
}
