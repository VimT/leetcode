//! 删点成林

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, to_delete: &HashSet<i32>, result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() { return root; }
        let node = root.as_ref().unwrap();
        let left = dfs(node.borrow().left.clone(), to_delete, result);
        let right = dfs(node.borrow().right.clone(), to_delete, result);
        node.borrow_mut().left = left;
        node.borrow_mut().right = right;
        if to_delete.contains(&node.borrow().val) {
            if node.borrow().left.is_some() {
                result.push(node.borrow().left.clone());
            }
            if node.borrow().right.is_some() {
                result.push(node.borrow().right.clone());
            }
            return None;
        }
        root
    }
    if root.is_none() { return vec![]; }
    let to_delete: HashSet<i32> = to_delete.into_iter().collect();
    let mut result = vec![];
    let root = dfs(root, &to_delete, &mut result);
    if root.is_some() { result.insert(0, root); }
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>>) {
        assert_eq!(func(tree![1,2,3,4,5,6,7], vec![3, 5]), vec![tree![1,2,null,4], tree![6], tree![7]]);
        assert_eq!(func(tree![1,2,4,null,3], vec![3]), vec![tree![1,2,4]]);
    }
    test(del_nodes);
}
