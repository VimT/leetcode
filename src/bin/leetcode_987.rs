//! 二叉树的垂序遍历

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut left_min = 0;
    let mut right = 0;
    let mut q = vec![(root, 0, 0)];
    let mut vals = vec![];
    while !q.is_empty() {
        let mut nq = Vec::with_capacity(q.len() * 2);
        for (root, x, y) in q {
            let node = root.as_ref().unwrap().borrow();
            vals.push((x, y, node.val));
            left_min = left_min.min(y);
            right = right.max(y);
            if node.left.is_some() {
                nq.push((node.left.clone(), x + 1, y - 1));
            }
            if node.right.is_some() {
                nq.push((node.right.clone(), x + 1, y + 1));
            }
        }
        q = nq;
    }
    vals.sort_unstable();
    let mut result = vec![vec![]; (right - left_min + 1) as usize];
    for (_, y, val) in vals {
        result[(y - left_min) as usize].push(val);
    }
    result
}


fn main() {
    assert_eq!(vertical_traversal(tree![3,9,20,null,null,15,7]), vec![vec![9], vec![3, 15], vec![20], vec![7]]);
    assert_eq!(vertical_traversal(tree![1,2,3,4,5,6,7]), vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]]);
    assert_eq!(vertical_traversal(tree![1,2,3,4,6,5,7]), vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]]);
}
