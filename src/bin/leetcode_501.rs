//! 二叉搜索树中的众数

use std::cell::RefCell;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn inner(root: Option<Rc<RefCell<TreeNode>>>, current: &mut i32, current_count: &mut i32, max: &mut i32, ans: &mut Vec<i32>) {
        if root.is_none() { return; }
        let node = root.as_ref().unwrap().borrow();
        inner(node.left.clone(), current, current_count, max, ans);
        if *current == node.val {
            *current_count += 1;
        } else {
            *current = node.val;
            *current_count = 1;
        }
        if current_count == max {
            ans.push(*current);
        } else if current_count > max {
            *max = *current_count;
            ans.clear();
            ans.push(*current);
        }
        inner(node.right.clone(), current, current_count, max, ans);
    }
    let mut ans = vec![];
    inner(root, &mut 0, &mut 0, &mut 0, &mut ans);
    ans
}


fn main() {
    assert_eq!(find_mode(tree![1,null,2,2]), vec![2]);
    assert_eq!(find_mode(tree![0]), vec![0]);
}
