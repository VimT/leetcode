//! 最深叶节点的最近公共祖先

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, max_depth: &mut usize, result: &mut Option<Rc<RefCell<TreeNode>>>) -> usize {
        if root.is_none() { return 0; }
        let node = root.as_ref().unwrap().borrow();
        let left_depth = dfs(node.left.clone(), depth + 1, max_depth, result);
        let right_depth = dfs(node.right.clone(), depth + 1, max_depth, result);
        if left_depth == right_depth {
            if depth + left_depth >= *max_depth {
                *max_depth = depth + left_depth;
                *result = root.clone();
            }
        }
        return left_depth.max(right_depth) + 1;
    }
    let mut result = None;
    dfs(root, 0, &mut 0, &mut result);
    result
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>) {
        assert_eq!(func(tree![3,5,1,6,2,0,8,null,null,7,4]), tree![2,7,4]);
        assert_eq!(func(tree![1]), tree![1]);
        assert_eq!(func(tree![0,1,3,null,2]), tree![2]);
    }
    test(lca_deepest_leaves);
}
