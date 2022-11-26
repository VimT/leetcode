//! 路径总和 III

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

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
    assert_eq!(path_sum(tree![10,5,-3,3,2,null,11,3,-2,null,1], 8), 3);
    assert_eq!(path_sum(tree![5,4,8,11,null,13,4,7,2,null,null,5,1], 22), 3);
}
