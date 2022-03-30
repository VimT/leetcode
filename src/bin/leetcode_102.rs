//! 二叉树的层序遍历

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

type Node = Option<Rc<RefCell<TreeNode>>>;

/// 维护一个最后点
pub fn level_order(root: Node) -> Vec<Vec<i32>> {
    if root.is_none() { return vec![]; }
    let mut this_line_last_node = root.clone();
    let mut next_line_last_node = root.clone();
    let mut q = VecDeque::new();
    q.push_back(root);
    let mut ans = vec![];
    let mut tmp = vec![];
    while !q.is_empty() {
        let n = q.pop_front().unwrap();
        tmp.push(n.as_ref().unwrap().borrow().val);
        let left = n.as_ref().unwrap().borrow().left.clone();
        if left.is_some() {
            next_line_last_node = left.clone();
            q.push_back(left);
        }
        let right = n.as_ref().unwrap().borrow().right.clone();
        if right.is_some() {
            next_line_last_node = right.clone();
            q.push_back(right);
        }
        if n == this_line_last_node {
            ans.push(tmp);
            tmp = vec![];
            this_line_last_node = next_line_last_node.clone();
        }
    }
    ans
}

/// 普通BFS就可以
pub fn level_order_bfs(root: Node) -> Vec<Vec<i32>> {
    if root.is_none() { return vec![]; }
    let mut q = VecDeque::new();
    q.push_back(root);
    let mut ans = vec![];
    while !q.is_empty() {
        let current_level_size = q.len();
        let mut tmp = vec![];
        for _ in 0..current_level_size {
            let n = q.pop_front().unwrap();
            let nn = n.as_ref().unwrap().borrow();
            tmp.push(nn.val);
            if nn.left.is_some() { q.push_back(nn.left.clone()) }
            if nn.right.is_some() { q.push_back(nn.right.clone()) }
        }
        ans.push(tmp);
    }
    ans
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(tree![3,9,20,null,null,15,7]), vec![vec![3], vec![9, 20], vec![15, 7]]);
        assert_eq!(func(tree![1]), [[1]]);
        assert_eq!(func(tree![]).is_empty(), true);
    }
    test(level_order);
    test(level_order_bfs);
}
