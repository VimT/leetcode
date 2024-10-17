//! 第 K 大的完美二叉子树的大小

use leetcode::treenode::TreeNode;
use std::rc::Rc;

use std::cell::RefCell;
pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    // 以 root 节点为根节点的完美二叉子树大小
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) -> Option<i32> {
        if root.is_none() { return Some(0); }
        let r = root.as_ref().unwrap().borrow();
        let left = dfs(r.left.clone(), result);
        let right = dfs(r.right.clone(), result);
        if let (Some(a), Some(b)) = (left, right) {
            if a == b {
                let num = a + b + 1;
                result.push(num);
                return Some(num);
            }
        }
        None
    }
    let mut result = vec![];
    dfs(root, &mut result);
    result.sort_unstable();
    let k = k as usize;
    if k <= result.len() { result[result.len() - k] } else { -1 }
}

fn main() {
    use leetcode::tree;
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32) {
        assert_eq!(func(tree![5,3,6,5,2,5,7,1,8,null,null,6,8], 2), 3);
        assert_eq!(func(tree![1,2,3,4,5,6,7], 1), 7);
        assert_eq!(func(tree![1,2,3,null,4], 3), -1);
    }
    test(kth_largest_perfect_subtree);
}
