//! 二叉树的垂直遍历

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};

pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() { return vec![]; }
    let mut q = VecDeque::new();
    q.push_back((root, 0));
    let mut map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    while !q.is_empty() {
        let (root, col) = q.pop_front().unwrap();
        let node = root.as_ref().unwrap().borrow();
        map.entry(col).or_default().push(node.val);
        if node.left.is_some() {
            q.push_back((node.left.clone(), col - 1));
        }
        if node.right.is_some() {
            q.push_back((node.right.clone(), col + 1));
        }
    }
    map.into_iter().map(|(_, v)| v).collect()
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(tree![3,9,20,null,null,15,7]), vec![vec![9], vec![3, 15], vec![20], vec![7]]);
        assert_eq!(func(tree![3,9,8,4,0,1,7]), vec![vec![4], vec![9], vec![3, 0, 1], vec![8], vec![7]]);
        assert_eq!(func(tree![3,9,8,4,0,1,7,null,null,null,2,5]), vec![vec![4], vec![9, 5], vec![3, 0, 1], vec![8, 2], vec![7]]);
    }
    test(vertical_order);
}
