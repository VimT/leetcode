//! 出现次数最多的子树元素和


use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cnt: &mut HashMap<i32, i32>) -> i32 {
        if let Some(v) = root.as_ref() {
            let node = v.borrow();
            let left = dfs(node.left.clone(), cnt);
            let right = dfs(node.right.clone(), cnt);
            let sum = node.val + left + right;
            *cnt.entry(sum).or_default() += 1;
            return sum;
        }
        0
    }
    let mut cnt = HashMap::new();
    dfs(root, &mut cnt);
    let max_cnt = *cnt.values().max().unwrap_or(&0);
    cnt.iter().filter(|x| *x.1 == max_cnt).map(|x| *x.0).collect()
}

fn main() {
    // assert_eq!(find_frequent_tree_sum(tree![5,2,-3]), vec![2, -3, 4]);
    assert_eq!(find_frequent_tree_sum(tree![5,2,-5]), vec![2]);
}
