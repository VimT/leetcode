//! 叶子相似的树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, leaf: &mut Vec<i32>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        if node.left.is_none() && node.right.is_none() {
            leaf.push(node.val);
            return;
        }
        if node.left.is_some() { dfs(node.left.clone(), leaf); }
        if node.right.is_some() { dfs(node.right.clone(), leaf); }
    }
    let mut leaf1 = vec![];
    dfs(root1, &mut leaf1);
    let mut leaf2 = vec![];
    dfs(root2, &mut leaf2);
    leaf1 == leaf2
}


fn main() {
    assert_eq!(leaf_similar(tree![3,5,1,6,2,9,8,null,null,7,4], tree![3,5,1,6,7,4,2,null,null,null,null,null,null,9,8]), true);
}
