//! 两棵二叉搜索树中的所有元素

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap().borrow();
        dfs(node.left.clone(), result);
        result.push(node.val);
        dfs(node.right.clone(), result);
    }
    let mut v1 = vec![];
    let mut v2 = vec![];
    dfs(root1, &mut v1);
    dfs(root2, &mut v2);
    let mut result = Vec::with_capacity(v1.len() + v2.len());
    let mut i = 0;
    let mut j = 0;
    while i < v1.len() && j < v2.len() {
        if v1[i] < v2[j] {
            result.push(v1[i]);
            i += 1;
        } else {
            result.push(v2[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&v1[i..]);
    result.extend_from_slice(&v2[j..]);
    result
}

fn main() {
    assert_eq!(get_all_elements(tree![2,1,4], tree![1,0,3]), vec![0, 1, 1, 2, 3, 4]);
    assert_eq!(get_all_elements(tree![1,null,8], tree![8,1]), vec![1, 1, 8, 8]);
}
