//! 移除子树后的二叉树高度

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, level_sub_high: &mut Vec<Vec<i32>>, current_level: usize) -> i32 {
        if root.is_none() { return 0; }
        let node = root.as_ref().unwrap().borrow();
        let left = dfs(node.left.clone(), level_sub_high, current_level + 1);
        let right = dfs(node.right.clone(), level_sub_high, current_level + 1);
        let result = left.max(right);
        if current_level >= level_sub_high.len() {
            level_sub_high.resize(current_level + 1, vec![]);
        }
        level_sub_high[current_level].push(result);
        result + 1
    }

    let mut level_sub_high = vec![];
    dfs(root.clone(), &mut level_sub_high, 0);
    for val in &mut level_sub_high {
        val.sort_unstable_by_key(|x| -*x);
    }
    let mut result = vec![0; queries.len()];
    let mut query_map: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, query) in queries.into_iter().enumerate() {
        query_map.entry(query).or_default().push(i);
    }
    fn dfs2(root: Option<Rc<RefCell<TreeNode>>>, level_sub_high: &Vec<Vec<i32>>, current_level: usize, queries: &HashMap<i32, Vec<usize>>, result: &mut Vec<i32>) -> i32 {
        if root.is_none() { return 0; }
        let node = root.as_ref().unwrap().borrow();
        let left = dfs2(node.left.clone(), level_sub_high, current_level + 1, queries, result);
        let right = dfs2(node.right.clone(), level_sub_high, current_level + 1, queries, result);
        let sub_high = left.max(right);
        if let Some(is) = queries.get(&node.val) {
            let this_level_sub_high = &level_sub_high[current_level];
            let other_high = if this_level_sub_high[0] == sub_high && this_level_sub_high.len() == 1 {
                current_level as i32 - 1
            } else if this_level_sub_high[0] == sub_high {
                current_level as i32 + this_level_sub_high[1]
            } else {
                current_level as i32 + this_level_sub_high[0]
            };
            for &i in is {
                result[i] = other_high;
            }
        }
        sub_high + 1
    }
    dfs2(root, &level_sub_high, 0, &query_map, &mut result);
    result
}

fn main() {
    assert_eq!(tree_queries(tree![1,null,5,3,null,2,4], vec![3, 5, 4, 2, 4]), [1,0,3,3,3]);
    assert_eq!(tree_queries(tree![1,3,4,2,null,6,5,null,null,null,null,null,7], vec![4]), [2]);
    assert_eq!(tree_queries(tree![5,8,9,2,1,3,7,4,6], vec![3, 2, 4, 8]), [3, 2, 3, 2]);
}
