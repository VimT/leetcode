//! 二叉树中的伪回文路径

use leetcode::treenode::TreeNode;
use std::rc::Rc;

use std::cell::RefCell;

pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cnt: &mut [i32; 10], result: &mut i32) {
        let node = root.as_ref().unwrap().borrow();
        cnt[node.val as usize] += 1;
        if node.left.is_none() && node.right.is_none() {
            if cnt.iter().map(|&x| x & 1).sum::<i32>() <= 1 {
                *result += 1;
            }
        }
        if node.left.is_some() { dfs(node.left.clone(), cnt, result); }
        if node.right.is_some() { dfs(node.right.clone(), cnt, result); }
        cnt[node.val as usize] -= 1;
    }
    let mut result = 0;
    dfs(root, &mut [0; 10], &mut result);
    result
}

fn main() {
    use leetcode::tree;
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> i32) {
        assert_eq!(func(tree![2,3,1,3,1,null,1]), 2);
        assert_eq!(func(tree![2,1,1,1,3,null,null,null,null,null,1]), 1);
        assert_eq!(func(tree![9]), 1);
    }
    test(pseudo_palindromic_paths);
}
