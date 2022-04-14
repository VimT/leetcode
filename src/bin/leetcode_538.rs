//! 把二叉搜索树转换为累加树

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use leetcode::tree;
use leetcode::treenode::{leetcode_tree, TreeNode};

pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut s = VecDeque::new();
    let mut cur = root.clone();
    while cur.is_some() {
        s.push_back(cur.clone());
        let tmp = cur.as_ref().unwrap().borrow().right.clone();
        cur = tmp;
    }
    let mut sum = 0;
    while !s.is_empty() {
        let cur = s.pop_back().unwrap();
        let mut node = cur.as_ref().unwrap().borrow_mut();
        sum += node.val;
        node.val = sum;
        let mut cur = node.left.clone();
        while cur.is_some() {
            s.push_back(cur.clone());
            let tmp = cur.as_ref().unwrap().borrow().right.clone();
            cur = tmp;
        }
    }
    root
}

// stack dfs
pub fn convert_bst_better(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() { return None; }

    let mut cursor = root.clone();
    let mut stack = VecDeque::new();
    let mut prev_sum = 0;

    while cursor.is_some() || !stack.is_empty() {
        while cursor.is_some() {
            stack.push_back(cursor.clone());
            cursor = cursor.unwrap().borrow().right.clone();
        }
        cursor = stack.pop_back().unwrap();
        prev_sum += cursor.as_ref().unwrap().borrow().val;
        cursor.as_mut().unwrap().borrow_mut().val = prev_sum;
        cursor = cursor.unwrap().borrow().left.clone();
    }
    return root;
}

fn main() {
    fn test(func: fn(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>) {
        assert_eq!(func(tree![4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]), tree![30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]);
        assert_eq!(func(tree![0,null,1]), tree![1,null,1]);
    }
    test(convert_bst);
    test(convert_bst_better);
}
