//! 路径总和 III


use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use leetcode::treenode::{TreeNode, vec_to_tree};

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, presum: &mut HashMap<i32, i32>, mut cur_sum: i32, target_sum: i32, result: &mut i32) {
        if let Some(node) = root.as_ref() {
            let n = node.borrow();
            cur_sum += n.val;
            *result += *presum.get(&(cur_sum - target_sum)).unwrap_or(&0);
            *presum.entry(cur_sum).or_default() += 1;
            if n.left.is_some() {
                dfs(n.left.clone(), presum, cur_sum, target_sum, result);
            }
            if n.right.is_some() {
                dfs(n.right.clone(), presum, cur_sum, target_sum, result);
            }
            *presum.entry(cur_sum).or_default() -= 1;
        }
    }
    let mut result = 0;
    let mut presum = HashMap::new();
    presum.insert(0, 1);
    dfs(root, &mut presum, 0, target_sum, &mut result);
    result
}

fn main() {
    assert_eq!(path_sum(vec_to_tree(vec![10, 5, 3, 3, 0, 0, -2, 0, 0, 2, 0, 1, 0, 0, -3, 0, 11]), 8), 3);
    assert_eq!(path_sum(vec_to_tree(vec![5, 4, 11, 7, 0, 0, 2, 0, 0, 0, 8, 13, 5, 0, 0, 1, 0, 0, 4]), 22), 3);
}
