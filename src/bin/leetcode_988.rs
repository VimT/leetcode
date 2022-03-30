//! 从叶结点开始的最小字符串

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

fn reverse_cmp(a: &[u8], b: &[u8]) -> Ordering {
    let l = a.len().min(b.len());
    let lhs = &a[a.len() - l..];
    let rhs = &b[b.len() - l..];
    for i in (0..l).rev() {
        match lhs[i].cmp(&rhs[i]) {
            Ordering::Equal => (),
            non_eq => return non_eq,
        }
    }
    a.len().cmp(&b.len())
}

pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cur: &mut Vec<u8>, result: &mut Vec<u8>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        cur.push(node.val as u8 + b'a');
        if node.left.is_none() && node.right.is_none() {
            if result.is_empty() || reverse_cmp(cur, result) == Ordering::Less {
                *result = cur.clone();
            }
        }
        if node.left.is_some() {
            dfs(node.left.clone(), cur, result);
        }
        if node.right.is_some() {
            dfs(node.right.clone(), cur, result);
        }
        cur.pop();
    }
    let mut result = vec![];
    dfs(root, &mut vec![], &mut result);
    result.reverse();
    unsafe { String::from_utf8_unchecked(result) }
}


fn main() {
    assert_eq!(smallest_from_leaf(tree![0,1,2,3,4,3,4]), String::from("dba"));
    assert_eq!(smallest_from_leaf(tree![25,1,3,1,3,0,2]), String::from("adz"));
    assert_eq!(smallest_from_leaf(tree![2,2,1,null,1,0,null,0]), String::from("abc"));
}
