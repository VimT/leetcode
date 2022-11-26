//! 寻找二叉树的叶子节点

use leetcode::treenode::TreeNode;
use leetcode::tree;

use std::rc::Rc;
use std::cell::RefCell;

pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<Vec<i32>>) -> usize {
        if root.is_none() { return 0; }
        let node = root.as_ref().unwrap().borrow();
        let left_level = dfs(node.left.clone(), result);
        let right_level = dfs(node.right.clone(), result);
        let idx = left_level.max(right_level);
        if result.len() == idx { result.push(vec![]); }
        result[idx].push(node.val);
        return idx + 1;
    }
    let mut result = vec![];
    dfs(root, &mut result);
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(tree![1,2,3,4,5]), vec![vec![4, 5, 3], vec![2], vec![1]]);
        assert_eq!(func(tree![1]), vec![vec![1]]);
        assert_eq!(func(tree![]).is_empty(), true);
    }
    test(find_leaves);
}
