//! 路径总和 II

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn path_sum(root: Node, sum: i32) -> Vec<Vec<i32>> {
    fn inner(node: Node, current: &mut Vec<i32>, current_num: i32, sum: i32, ans: &mut Vec<Vec<i32>>) {
        if node.is_none() { return; }
        let real_node = node.as_ref().unwrap().borrow();
        let val = real_node.val;
        let new_sum = current_num + val;
        if new_sum > sum { return; }
        current.push(val);
        if real_node.left.is_none() && real_node.right.is_none() && new_sum == sum {
            ans.push(current.clone());
            current.pop();
            return;
        }
        if real_node.left.is_some() {
            inner(real_node.left.clone(), current, new_sum, sum, ans);
        }
        if real_node.right.is_some() {
            inner(real_node.right.clone(), current, new_sum, sum, ans);
        }
        current.pop();
    }
    let mut ans = vec![];
    inner(root, &mut vec![], 0, sum, &mut ans);
    ans
}


fn main() {
    assert_eq!(path_sum(tree![5,4,8,11,null,13,4,7,2,null,null,5,1], 22), vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]);
    assert_eq!(path_sum(tree![1,2,3], 5).is_empty(), true);
    assert_eq!(path_sum(tree![1,2], 0).is_empty(), true);
}
