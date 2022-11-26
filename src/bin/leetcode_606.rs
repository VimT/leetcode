//! 根据二叉树创建字符串

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<u8> {
        let mut result = vec![];
        if root.is_none() {
            return vec![b'(', b')'];
        }
        let node = root.as_ref().unwrap().borrow();
        result.extend_from_slice(node.val.to_string().as_bytes());
        if node.left.is_none() && node.right.is_none() { return result; }
        result.push(b'(');
        if node.left.is_some() {
            result.extend_from_slice(&dfs(node.left.clone()));
        }
        result.push(b')');
        if node.right.is_some() {
            result.push(b'(');
            result.extend_from_slice(&dfs(node.right.clone()));
            result.push(b')');
        }
        result
    }
    unsafe { String::from_utf8_unchecked(dfs(root)) }
}

fn main() {
    assert_eq!(tree2str(tree![1,2,3,4]), String::from("1(2(4))(3)"));
    assert_eq!(tree2str(tree![1,2,3,null,4]), String::from("1(2()(4))(3)"));
}
