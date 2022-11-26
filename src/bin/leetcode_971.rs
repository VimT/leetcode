//! 翻转二叉树以匹配先序遍历

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, voyage: &Vec<i32>, i: &mut usize, result: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let mut node = root.as_ref().unwrap().borrow_mut();
        if node.val != voyage[*i] {
            *result = vec![-1];
            return;
        }
        if node.left.is_some() && node.left.as_ref().unwrap().borrow().val != voyage[*i + 1] {
            let left = node.left.clone();
            let right = std::mem::replace(&mut node.right, left);
            node.left = right;
            result.push(node.val);
        }
        *i += 1;
        dfs(node.left.clone(), voyage, i, result);
        dfs(node.right.clone(), voyage, i, result);
    }
    let mut result = vec![];
    dfs(root, &voyage, &mut 0, &mut result);
    if !result.is_empty() && result[0] == -1 {
        return vec![-1];
    }
    result
}


fn main() {
    assert_eq!(flip_match_voyage(tree![1,2], vec![2, 1]), vec![-1]);
    assert_eq!(flip_match_voyage(tree![1,2,3], vec![1, 3, 2]), vec![1]);
    assert_eq!(flip_match_voyage(tree![1,2,3], vec![1, 2, 3]), vec![]);
}
