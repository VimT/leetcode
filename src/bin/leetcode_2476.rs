//! 二叉搜索树最近节点查询

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

/// 最坏情况下二叉搜索树是一条链。所以这个会超时
pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
    fn query(root: Option<Rc<RefCell<TreeNode>>>, val: i32, result: &mut Vec<i32>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        if node.val == val {
            result.fill(val);
        } else if node.val > val {
            result[1] = node.val;
            query(node.left.clone(), val, result);
        } else {
            result[0] = node.val;
            query(node.right.clone(), val, result);
        }
    }
    queries.into_iter().map(|x| {
        let mut result = vec![-1, -1];
        query(root.clone(), x, &mut result);
        result
    }).collect()
}

pub fn closest_nodes2(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        dfs(node.left.clone(), result);
        result.push(node.val);
        dfs(node.right.clone(), result);
    }
    let mut vals = vec![];
    dfs(root, &mut vals);
    queries.into_iter().map(|x| {
        let idx = vals.binary_search(&x).unwrap_or_else(|x| x);
        if idx < vals.len() && vals[idx] == x {
            vec![x, x]
        } else {
            vec![if idx > 0 { vals[idx - 1] } else { -1 }, if idx < vals.len() { vals[idx] } else { -1 }]
        }
    }).collect()
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>>) {
        assert_eq!(func(tree![6,2,13,1,4,9,15,null,null,null,null,null,null,14], vec![2, 5, 16]), vec![vec![2, 2], vec![4, 6], vec![15, -1]]);
        assert_eq!(func(tree![4,null,9], vec![3]), vec![vec![-1, 4]]);
    }
    test(closest_nodes);
    test(closest_nodes2);
}
