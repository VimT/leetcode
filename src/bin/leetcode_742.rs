//! 二叉树最近的叶节点

use std::cell::RefCell;
use leetcode::treenode::{leetcode_tree, TreeNode};
use leetcode::tree;

use std::rc::Rc;
use std::collections::VecDeque;

/// 转换成图
/// 另一种做法是 找 root到k路径上每个节点 的最近叶节点和距离，找最短距离的
pub fn find_closest_leaf(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut graph = vec![vec![]; 1001];
    let mut nodes = vec![None; 1001];
    fn dfs(graph: &mut Vec<Vec<Rc<RefCell<TreeNode>>>>, nodes: &mut Vec<Option<Rc<RefCell<TreeNode>>>>, node: Rc<RefCell<TreeNode>>, parent: Rc<RefCell<TreeNode>>) {
        graph[node.borrow().val as usize].push(parent.clone());
        graph[parent.borrow().val as usize].push(node.clone());
        nodes[node.borrow().val as usize] = Some(node.clone());
        if node.borrow().left.is_some() {
            dfs(graph, nodes, node.borrow().left.clone().unwrap(), node.clone());
        }
        if node.borrow().right.is_some() {
            dfs(graph, nodes, node.borrow().right.clone().unwrap(), node.clone());
        }
    }
    if root.is_none() { return 0; }
    let root = root.unwrap();
    nodes[root.borrow().val as usize] = Some(root.clone());
    if root.borrow().left.is_some() {
        dfs(&mut graph, &mut nodes, root.borrow().left.clone().unwrap(), root.clone());
    }
    if root.borrow().right.is_some() {
        dfs(&mut graph, &mut nodes, root.borrow().right.clone().unwrap(), root.clone());
    }
    let mut q = VecDeque::new();
    let mut seen = vec![false; 1001];
    for i in 1..=1000 {
        if let Some(node) = &nodes[i] {
            if node.borrow().val == k {
                q.push_back(node.clone());
            }
        }
    }
    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return node.borrow().val;
        }
        for nei in &graph[node.borrow().val as usize] {
            if !seen[nei.borrow().val as usize] {
                seen[nei.borrow().val as usize] = true;
                q.push_back(nei.clone());
            }
        }
    }
    unreachable!()
}


fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32) {
        assert_eq!(func(tree![1,2], 1), 2);
        assert_eq!(func(tree![1,3,2], 1), 3);
        assert_eq!(func(tree![1], 1), 1);
        assert_eq!(func(tree![1,2,3,4,null,null,null,5,null,6], 2), 3);
    }
    test(find_closest_leaf);
}
