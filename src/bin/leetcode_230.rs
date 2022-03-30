//! 二叉搜索树中第K小的元素

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

/// stack中序遍历
pub fn kth_smallest(mut root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
    let mut stack: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

    loop {
        while root.is_some() {
            stack.push_back(root.clone());
            root = (*root.unwrap()).borrow().left.clone();
        }
        root = stack.pop_back().unwrap();
        k -= 1;
        let rc = root.unwrap();
        let node = (*rc).borrow();
        if k == 0 {
            return node.val;
        }
        root = node.right.clone();
    }
}


fn main() {
    assert_eq!(kth_smallest(tree![3,1,4,null,2], 1), 1);
    assert_eq!(kth_smallest(tree![5,3,6,2,4,null,null,1], 3), 3);
}
