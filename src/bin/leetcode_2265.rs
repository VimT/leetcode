//! 统计值等于子树平均值的节点数

use std::cell::RefCell;
use std::rc::Rc;
use leetcode::tree;
use leetcode::treenode::TreeNode;

pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> (i32, i32) {
        if root.is_none() { return (0, 0); }
        let node = root.as_ref().unwrap().borrow();
        let (left, left_cnt) = dfs(node.left.clone(), result);
        let (right, right_cnt) = dfs(node.right.clone(), result);
        if (node.val + left + right) / (1 + left_cnt + right_cnt) == node.val {
            *result += 1;
        }
        (node.val + left + right, 1 + left_cnt + right_cnt)
    }
    let mut result = 0;
    dfs(root, &mut result);
    result
}

fn main() {
    assert_eq!(average_of_subtree(tree![4,8,5,0,1,null,6]), 5);
    assert_eq!(average_of_subtree(tree![1]), 1);
}
