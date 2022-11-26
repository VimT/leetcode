//! 二叉树中所有距离为 K 的结点

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> Vec<i32> {
    let mut parent = vec![None; 501];
    let mut q = VecDeque::new();
    q.push_back(root);
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        let nb = node.as_ref().unwrap().borrow();
        if nb.left.is_some() {
            parent[nb.left.as_ref().unwrap().borrow().val as usize] = node.clone();
            q.push_back(nb.left.clone());
        }
        if nb.right.is_some() {
            parent[nb.right.as_ref().unwrap().borrow().val as usize] = node.clone();
            q.push_back(nb.right.clone());
        }
    }
    let mut seen = vec![false; 501];
    seen[target.as_ref().unwrap().borrow().val as usize] = true;
    let mut q = vec![target];
    while !q.is_empty() {
        if k == 0 {
            return q.into_iter().map(|x| x.as_ref().unwrap().borrow().val).collect();
        }
        let mut nq = Vec::with_capacity(q.len() * 3);
        for node in q {
            let nb = node.as_ref().unwrap().borrow();
            if parent[nb.val as usize].is_some() && !seen[parent[nb.val as usize].as_ref().unwrap().borrow().val as usize] {
                seen[parent[nb.val as usize].as_ref().unwrap().borrow().val as usize] = true;
                nq.push(parent[nb.val as usize].clone());
            }
            if nb.left.is_some() && !seen[nb.left.as_ref().unwrap().borrow().val as usize] {
                seen[nb.left.as_ref().unwrap().borrow().val as usize] = true;
                nq.push(nb.left.clone());
            }
            if nb.right.is_some() && !seen[nb.right.as_ref().unwrap().borrow().val as usize] {
                seen[nb.right.as_ref().unwrap().borrow().val as usize] = true;
                nq.push(nb.right.clone());
            }
        }
        k -= 1;
        q = nq;
    }

    vec![]
}


fn main() {
    assert_eq!(distance_k(tree![3,5,1,6,2,0,8,null,null,7,4], tree![5,6,2,null,null,7,4], 2), vec![1, 7, 4]);
    assert_eq!(distance_k(tree![1], tree![1], 3), vec![]);
}
