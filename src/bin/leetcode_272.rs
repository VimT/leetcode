//! 最接近的二叉搜索树值 II

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

/// 中序遍历，栈，双指针
pub fn closest_k_values(root: Option<Rc<RefCell<TreeNode>>>, target: f64, k: i32) -> Vec<i32> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target: f64, k: usize, s: &mut Vec<i32>, result: &mut Vec<i32>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        dfs(node.left.clone(), target, k, s, result);
        if result.len() >= k { return; }
        if (node.val as f64) < target {
            s.push(node.val);
        } else {
            let diff = node.val as f64 - target;
            while !s.is_empty() && diff > (target - *s.last().unwrap() as f64) && result.len() < k {
                result.push(s.pop().unwrap());
            }
            result.push(node.val);
        }
        if result.len() >= k { return; }
        dfs(node.right.clone(), target, k, s, result);
    }
    let k = k as usize;
    let mut result = Vec::with_capacity(k + 1);
    let mut s = vec![];
    dfs(root, target, k, &mut s, &mut result);
    while result.len() > k {
        result.pop();
    }
    while result.len() < k {
        result.push(s.pop().unwrap());
    }
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, target: f64, k: i32) -> Vec<i32>) {
        assert_eq!(func(tree![4,2,5,1,3], 3.714286, 2), vec![4, 3]);
        assert_eq!(func(tree![1], 0.000000, 1), vec![1]);
        assert_eq!(func(tree![1], 4.000000, 1), vec![1]);
    }
    test(closest_k_values);
}
