//! 所有可能的满二叉树

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if n & 1 == 0 { return vec![]; }
    fn dfs(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))];
        }
        let mut result = vec![];
        for left_num in (1..=n - 2).step_by(2) {
            for left in &dfs(left_num) {
                for right in dfs(n - 1 - left_num) {
                    let mut n = TreeNode::new(0);
                    n.left = left.clone();
                    n.right = right;
                    result.push(Some(Rc::new(RefCell::new(n))))
                }
            }
        }
        result
    }
    dfs(n)
}


fn main() {
    assert_eq!(all_possible_fbt(7), vec![tree![0,0,0,null,null,0,0,null,null,0,0], tree![0,0,0,null,null,0,0,0,0], tree![0,0,0,0,0,0,0], tree![0,0,0,0,0,null,null,null,null,0,0], tree![0,0,0,0,0,null,null,0,0]]);
    assert_eq!(all_possible_fbt(3), vec![tree![0,0,0]]);
}
