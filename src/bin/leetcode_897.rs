//! 递增顺序搜索树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

struct Wrap(Option<Rc<RefCell<TreeNode>>>);

pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Wrap) {
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap().borrow();
        if node.left.is_some() { dfs(node.left.clone(), result); }
        let new_node = Some(Rc::new(RefCell::new(TreeNode::new(node.val))));
        result.0.as_ref().unwrap().borrow_mut().right = new_node.clone();
        *result = Wrap(new_node.clone());
        if node.right.is_some() { dfs(node.right.clone(), result); }
    }
    let result = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    dfs(root, &mut Wrap(result.clone()));
    let x = result.as_ref().unwrap().borrow_mut().right.take();
    x
}


fn main() {
    assert_eq!(increasing_bst(tree![5,3,6,2,4,null,8,1,null,null,null,7,9]), tree![1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]);
    assert_eq!(increasing_bst(tree![5,1,7]), tree![1,null,5,null,7]);
}
